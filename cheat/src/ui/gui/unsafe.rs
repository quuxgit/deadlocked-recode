use egui::{DragValue, Ui};

use crate::ui::{
    app::AppState,
    gui::helpers::{collapsing_open, color_picker, keybind},
};

impl AppState {
    pub fn unsafe_settings(&mut self, ui: &mut Ui) {
        ui.columns(2, |cols| {
            let left = &mut cols[0];
            egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .id_salt("unsafe_left")
                .show(left, |left| {
                    self.unsafe_left(left);
                });

            let right = &mut cols[1];
            egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .id_salt("unsafe_right")
                .show(right, |right| {
                    self.unsafe_right(right);
                });
        });
    }

    fn unsafe_left(&mut self, ui: &mut Ui) {
        collapsing_open(ui, "No Flash", |ui| {
            if ui
                .checkbox(&mut self.config.misc.no_flash, "No Flash")
                .changed()
            {
                self.send_config_game();
            }

            ui.horizontal(|ui| {
                if ui
                    .add(
                        DragValue::new(&mut self.config.misc.max_flash_alpha)
                            .range(0.0..=255.0)
                            .speed(0.5)
                            .max_decimals(0),
                    )
                    .changed()
                {
                    self.send_config_game();
                }
                ui.label("Max Flash Alpha");
            });
        });

        collapsing_open(ui, "Smokes", |ui| {
            if ui
                .checkbox(&mut self.config.misc.no_smoke, "No Smoke")
                .changed()
            {
                self.send_config_game();
            }

            if ui
                .checkbox(
                    &mut self.config.misc.change_smoke_color,
                    "Change Smoke Color",
                )
                .changed()
            {
                self.send_config_game();
            }

            if color_picker(ui, "", &mut self.config.misc.smoke_color) {
                self.send_config_game();
            }
        });
    }

    fn unsafe_right(&mut self, ui: &mut Ui) {
        collapsing_open(ui, "FOV Changer", |ui| {
            if ui
                .checkbox(&mut self.config.misc.fov_changer, "FOV Changer")
                .changed()
            {
                self.send_config_game();
            }

            ui.horizontal(|ui| {
                if ui
                    .add(
                        DragValue::new(&mut self.config.misc.desired_fov)
                            .speed(0.1)
                            .range(1..=179),
                    )
                    .changed()
                {
                    self.send_config_game();
                }
                ui.label("Desired FOV");

                if ui.button("Reset").clicked() {
                    self.config.misc.desired_fov = crate::constants::cs2::DEFAULT_FOV;
                    self.send_config_game();
                }
            });
        });

        collapsing_open(ui, "Bunnyhop", |ui| {
            if ui
                .checkbox(&mut self.config.misc.bunnyhop, "Bunnyhop")
                .changed()
            {
                self.send_config_game();
            }

            if self.config.misc.bunnyhop {
                ui.horizontal(|ui| {
                    ui.label("Mode:");
                    if ui
                        .selectable_value(
                            &mut self.config.misc.bhop_mode,
                            crate::config::r#unsafe::BhopMode::Always,
                            "Always",
                        )
                        .clicked()
                    {
                        self.send_config_game();
                    }
                    if ui
                        .selectable_value(
                            &mut self.config.misc.bhop_mode,
                            crate::config::r#unsafe::BhopMode::Hold,
                            "Hold",
                        )
                        .clicked()
                    {
                        self.send_config_game();
                    }
                    if ui
                        .selectable_value(
                            &mut self.config.misc.bhop_mode,
                            crate::config::r#unsafe::BhopMode::Toggle,
                            "Toggle",
                        )
                        .clicked()
                    {
                        self.send_config_game();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Key:");
                    if keybind(ui, "bhop_key", "Bhop Key", &mut self.config.misc.bhop_key) {
                        self.send_config_game();
                    }
                });
            }
        });

        collapsing_open(ui, "Process Freeze", |ui| {
            if let Some(pid) = self.cs2_pid {
                let button_text = if self.process_freeze.is_frozen() {
                    "Unfreeze CS2"
                } else {
                    "Freeze CS2"
                };

                if ui.button(button_text).clicked() {
                    if self.process_freeze.is_frozen() {
                        self.process_freeze.unfreeze(pid);
                    } else {
                        self.process_freeze.freeze(pid);
                    }
                }

                ui.label(if self.process_freeze.is_frozen() {
                    egui::RichText::new("Status: Frozen")
                        .color(egui::Color32::from_rgb(255, 100, 100))
                } else {
                    egui::RichText::new("Status: Running")
                        .color(egui::Color32::from_rgb(100, 255, 100))
                });
            } else {
                ui.label("Waiting for CS2...");
            }
        });

        collapsing_open(ui, "No Recoil (beta)", |ui| {
            if ui
                .checkbox(&mut self.config.misc.no_recoil, "No Recoil")
                .changed()
            {
                self.send_config_game();
            }
        });
    }
}
