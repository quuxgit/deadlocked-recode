use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

pub struct ProcessFreeze {
    frozen: bool,
}

impl Default for ProcessFreeze {
    fn default() -> Self {
        Self { frozen: false }
    }
}

impl ProcessFreeze {
    pub fn freeze(&mut self, pid: i32) {
        if self.frozen {
            return;
        }

        match kill(Pid::from_raw(pid), Signal::SIGSTOP) {
            Ok(_) => {
                self.frozen = true;
                utils_log::info!("CS2 process frozen (PID: {})", pid);
            }
            Err(e) => {
                utils_log::warn!("Failed to freeze CS2 process: {}", e);
            }
        }
    }

    pub fn unfreeze(&mut self, pid: i32) {
        if !self.frozen {
            return;
        }

        match kill(Pid::from_raw(pid), Signal::SIGCONT) {
            Ok(_) => {
                self.frozen = false;
                utils_log::info!("CS2 process unfrozen (PID: {})", pid);
            }
            Err(e) => {
                utils_log::warn!("Failed to unfreeze CS2 process: {}", e);
            }
        }
    }

    pub fn is_frozen(&self) -> bool {
        self.frozen
    }
}

impl Drop for ProcessFreeze {
    fn drop(&mut self) {
        if self.frozen {
            utils_log::warn!("ProcessFreeze dropped while frozen - attempting to unfreeze");
        }
    }
}
