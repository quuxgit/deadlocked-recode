<div align="center">

# deadlocked recode

Fork of [deadlocked](https://github.com/avitran0/deadlocked) by avitrano

[![Telegram](https://img.shields.io/badge/Telegram-@quuxtg-blue?style=for-the-badge&logo=telegram)](https://t.me/quuxtg)

[![Casual Maintenance Intended](https://casuallymaintained.tech/badge.svg)](https://casuallymaintained.tech/)

<br>

Enhanced CS2 aimbot and ESP with improved UI and additional features, for Linux only.

<br>

Releases are tagged `v<version>-recode` matching the version in `Cargo.toml` (e.g. `v1.4.1-recode`).
The built-in update checker compares against the latest release tag and will prompt when a newer version is available.

</div>

<br>

## What's New in Recode

This fork adds several quality-of-life improvements and new features:

### New Features
- **Bunnyhop** - automatic bunny hopping with hold/toggle modes
- **FOV Changer** - adjust your field of view (unsafe)
- **Process Freeze** - pause CS2 entirely (great for breaks)
- **No Recoil** - recoil control assistance (beta, unsafe)
- **Custom Backgrounds** - themed UI backgrounds (Sweety Blue, Sweety Teal, Sweety Fox)
- **Background Opacity Control** - adjust background transparency

### UI Improvements
- **Modernized Interface** - smooth animations and transitions
- **Cleaner Layout** - removed radar features for simplicity
- **Better Organization** - reorganized settings for easier navigation
- **Visual Polish** - improved spacing and visual hierarchy

<br>

## Quick Start

> [!NOTE]
> Running NixOS, Fedora Atomic, Hyprland (Legacy .conf config)?
>
> See [compatibility.md](compatibility.md).

Download the [latest release](https://github.com/quuxgit/deadlocked-recode/releases). Each release contains the `deadlocked` (or `deadlocked-reduced-models`) binary and `setup.sh`.

**Setup (one-time only):**

```bash
./setup.sh
```

> **Restart your machine (required)**

This creates a `uinput` group, adds your user to it, and installs a udev rule.
You only need to do this once, even when updating to newer versions.

**Run:**

`./deadlocked` or `./deadlocked-reduced-models`, depending on which version you downloaded.

The binary will refuse to start if setup hasn't been completed.
Also make sure the `uinput` kernel module is loaded.

<br>

## Build from Source

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/quuxgit/deadlocked-recode
cd deadlocked-recode
cargo run --release --bin deadlocked
```

<br>

## Features

### Aimbot

- Hotkey
- FOV
- Smooth
- Start bullet
- Targeting mode
- Visibility check (VPK parsing)
- Head only/whole body
- Flash check
- FOV circle

### ESP

- Hotkey
- Box
- Skeleton
- Health bar
- Armor bar
- Player name
- Weapon icon
- Player tags (helmet, defuser, bomb)
- Dropped weapons
- Bomb timer

### Triggerbot

- Activation mode
- Min/max delay
- Additional Duration
- Visibility check
- Flash check
- Scope check
- Velocity threshold
- Head only mode

### Standalone RCS

- Smoothing

### Per-Weapon Overrides

- Aimbot
- Triggerbot
- RCS

### Misc

- Sniper crosshair
- Bomb timer
- Grenade lineup alignment
- **Bunnyhop** (always/hold/toggle)

### Unsafe

> [!WARNING]
> These features write to game memory and might get you banned.

- No flash (with max flash alpha)
- **FOV changer** (with reset button)
- No smoke
- Smoke color change
- **Process freeze** (pause CS2)
- **No recoil** (beta)

<br>

## FAQ

<details>
<summary>Where are my configs saved?</summary>

Configs are saved in `$XDG_CONFIG_HOME` with fallback to `$HOME/.config`. Otherwise they're saved alongside the executable.

</details>

<br>

<details>
<summary>Which desktop environments and window managers are supported?</summary>

**Best support:**

- GNOME (Mutter)
- KDE (KWin)

**Good support:**

- SwayWM
- Weston

**Fair support:**

- i3
- OpenBox
- XFCE
- Hyprland (tweaks may be needed, no guarantees; see [compatibility.md](compatibility.md/#hyprland))

</details>

<br>

<details>
<summary>I'm using Hyprland and something doesn't work</summary>

Hyprland has poor X11 support for the techniques this cheat uses, not much I can do about that.
Try another WM if possible.

</details>

<br>

<details>
<summary>I'm using Gamescope and the overlay is too small</summary>

The game still thinks it's running in 16:9 resolution, so the cheat gets the wrong window resolution.
Try running the game without Gamescope.

</details>

<br>

<details>
<summary>My screen/overlay is black</summary>

Your compositor or window manager doesn't support transparency, or it's not enabled.

On KDE, go into **Display and Monitor** settings, then **Compositor**, and tick **Enable compositor on startup**.

</details>

<br>

<details>
<summary>The overlay shows but I can't click anything</summary>

The window couldn't be made click-through. This is a window manager/compositor limitation.

</details>

<br>

<details>
<summary>The overlay doesn't show up</summary>

Your window manager doesn't support positioning or resizing windows.

</details>

<br>

<details>
<summary>The overlay isn't on top of other windows</summary>

Your window manager doesn't support always-on-top windows.

</details>

<br>

## Credits

- Original [deadlocked](https://github.com/avitran0/deadlocked) by [avitran0](https://github.com/avitran0)
- Recode modifications by [quuxgit](https://github.com/quuxgit)
