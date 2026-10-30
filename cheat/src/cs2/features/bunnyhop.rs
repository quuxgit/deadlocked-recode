use std::{
    fs::File,
    io::Write,
    os::fd::AsRawFd,
    sync::{atomic::{AtomicBool, Ordering}, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use nix::{ioctl_none, ioctl_write_int, ioctl_write_ptr, libc::c_ulong};

use crate::{
    config::{Config, r#unsafe::BhopMode},
    cs2::{CS2, entity::player::Player},
};

const FL_ONGROUND: i32 = 1 << 0;

static KEYBOARD: Mutex<Option<Keyboard>> = Mutex::new(None);
static LAST_GROUNDED: AtomicBool = AtomicBool::new(false);
static BHOP_TOGGLED: AtomicBool = AtomicBool::new(false);

impl CS2 {
    pub fn run_bhop(&mut self, config: &Config) {
        let key_pressed = self.input.is_key_pressed(config.misc.bhop_key);
        let key_just_pressed = self.input.key_just_pressed(config.misc.bhop_key);

        if config.misc.bhop_mode == BhopMode::Toggle && key_just_pressed {
            BHOP_TOGGLED.fetch_xor(true, Ordering::Relaxed);
        }

        let should_bhop = match config.misc.bhop_mode {
            BhopMode::Always => true,
            BhopMode::Hold => key_pressed,
            BhopMode::Toggle => BHOP_TOGGLED.load(Ordering::Relaxed),
        };

        if !should_bhop {
            LAST_GROUNDED.store(false, Ordering::Relaxed);
            return;
        }

        let Some(local_player) = Player::local_player(self) else {
            LAST_GROUNDED.store(false, Ordering::Relaxed);
            return;
        };

        let flags: i32 = self.process.read(**local_player + self.offsets.pawn.flags);
        let on_ground = flags & FL_ONGROUND != 0;

        let last = LAST_GROUNDED.load(Ordering::Relaxed);
        let just_landed = !last && on_ground;
        LAST_GROUNDED.store(on_ground, Ordering::Relaxed);

        if just_landed {
            let mut kb = KEYBOARD.lock().unwrap();
            if kb.is_none() {
                *kb = Keyboard::open().ok();
            }
            if let Some(keyboard) = kb.as_mut() {
                keyboard.jump();
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Timeval {
    seconds: u64,
    microseconds: u64,
}

#[derive(Clone, Copy)]
struct InputEvent {
    time: Timeval,
    event_type: u16,
    code: u16,
    value: i32,
}

impl InputEvent {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(24);
        bytes.extend(&self.time.seconds.to_le_bytes());
        bytes.extend(&self.time.microseconds.to_le_bytes());
        bytes.extend(&self.event_type.to_le_bytes());
        bytes.extend(&self.code.to_le_bytes());
        bytes.extend(&self.value.to_le_bytes());
        bytes
    }
}

#[repr(C)]
struct DeviceSetup {
    id: InputId,
    name: [u8; 80],
    ff_effects_max: u32,
}

#[repr(C)]
struct InputId {
    bustype: u16,
    vendor: u16,
    product: u16,
    version: u16,
}

const DEVICE_SETUP: DeviceSetup = DeviceSetup {
    id: InputId {
        bustype: 0x03,
        vendor: 0x046d,
        product: 0xc52b,
        version: 1,
    },
    name: [
        66, 104, 111, 112, 32, 75, 101, 121, 98, 111, 97, 114, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    ff_effects_max: 0,
};

const UINPUT_IOCTL_BASE: c_ulong = b'U' as c_ulong;
ioctl_none!(ui_dev_create, UINPUT_IOCTL_BASE, 1);
ioctl_none!(ui_dev_destroy, UINPUT_IOCTL_BASE, 2);
ioctl_write_int!(ui_set_evbit, UINPUT_IOCTL_BASE, 100);
ioctl_write_int!(ui_set_keybit, UINPUT_IOCTL_BASE, 101);
ioctl_write_ptr!(ui_dev_setup, UINPUT_IOCTL_BASE, 3, DeviceSetup);

const EV_SYN: u16 = 0x00;
const EV_KEY: u16 = 0x01;
const SYN_REPORT: u16 = 0x00;
const KEY_SPACE: u16 = 57;

struct Keyboard {
    file: File,
}

impl Keyboard {
    fn open() -> Result<Self, String> {
        let file = File::options()
            .write(true)
            .open("/dev/uinput")
            .map_err(|e| e.to_string())?;
        let fd = file.as_raw_fd();

        unsafe {
            ui_set_evbit(fd, EV_SYN as u64).map_err(|e| e.to_string())?;
            ui_set_evbit(fd, EV_KEY as u64).map_err(|e| e.to_string())?;
            ui_set_keybit(fd, KEY_SPACE as u64).map_err(|e| e.to_string())?;
            ui_dev_setup(fd, &DEVICE_SETUP).map_err(|e| e.to_string())?;
            ui_dev_create(fd).map_err(|e| e.to_string())?;
        }

        Ok(Self { file })
    }

    fn jump(&mut self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let press_us = now.subsec_micros() as u64;
        let release_us_raw = press_us + 1000;

        let press_time = Timeval {
            seconds: now.as_secs(),
            microseconds: press_us,
        };

        let release_time = Timeval {
            seconds: now.as_secs() + release_us_raw / 1_000_000,
            microseconds: release_us_raw % 1_000_000,
        };

        let events = [
            InputEvent { time: press_time,   event_type: EV_KEY, code: KEY_SPACE,  value: 1 },
            InputEvent { time: press_time,   event_type: EV_SYN, code: SYN_REPORT, value: 0 },
            InputEvent { time: release_time, event_type: EV_KEY, code: KEY_SPACE,  value: 0 },
            InputEvent { time: release_time, event_type: EV_SYN, code: SYN_REPORT, value: 0 },
        ];

        for event in &events {
            let _ = self.file.write_all(&event.bytes());
        }
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        let _ = unsafe { ui_dev_destroy(self.file.as_raw_fd()) };
    }
}