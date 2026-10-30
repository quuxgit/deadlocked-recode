use egui::Color32;
use serde::{Deserialize, Serialize};
use crate::cs2::key_codes::KeyCode;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BhopMode {
    Always,
    Hold,
    Toggle,
}

impl Default for BhopMode {
    fn default() -> Self {
        Self::Hold
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UnsafeConfig {
    pub no_flash: bool,
    pub max_flash_alpha: f32,
    pub fov_changer: bool,
    pub desired_fov: u32,
    pub no_smoke: bool,
    pub change_smoke_color: bool,
    pub smoke_color: Color32,
    pub bunnyhop: bool,
    pub bhop_mode: BhopMode,
    pub bhop_key: KeyCode,
    pub no_recoil: bool,
}

impl Default for UnsafeConfig {
    fn default() -> Self {
        Self {
            no_flash: false,
            max_flash_alpha: 127.0,
            fov_changer: false,
            desired_fov: 90,
            no_smoke: false,
            change_smoke_color: false,
            smoke_color: Color32::RED,
            bunnyhop: false,
            bhop_mode: BhopMode::default(),
            bhop_key: KeyCode::Space,
            no_recoil: false,
        }
    }
}
