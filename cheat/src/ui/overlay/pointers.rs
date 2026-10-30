use egui::{Painter, Pos2, Color32};
use shared::Data;

use crate::ui::app::AppState;

impl AppState {
    pub fn draw_pointers(&self, painter: &Painter, data: &Data) {
        if !self.config.hud.pointers.enabled || !data.esp_active {
            return;
        }

        let center_x = data.window_size.x / 2.0;
        let center_y = data.window_size.y / 2.0;

        let local_pos = data.local_player.position;
        let local_yaw = data.view_angles.y;

        for player in &data.players {
            if player.health <= 0 {
                continue;
            }

            let dx = player.position.x - local_pos.x;
            let dy = player.position.y - local_pos.y;
            
            let angle_to_player = dy.atan2(dx).to_degrees();
            let relative_angle = (local_yaw - angle_to_player + 360.0) % 360.0;
            let angle_rad = relative_angle.to_radians();

            let pointer_x = center_x + angle_rad.sin() * self.config.hud.pointers.radius;
            let pointer_y = center_y - angle_rad.cos() * self.config.hud.pointers.radius;
            let pointer_pos = Pos2::new(pointer_x, pointer_y);

            self.draw_triangle(painter, pointer_pos, angle_rad, self.config.hud.pointers.size, self.config.hud.pointers.color);
        }
    }

    fn draw_triangle(&self, painter: &Painter, pos: Pos2, angle: f32, size: f32, color: Color32) {
        let half_size = size / 2.0;
        
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        
        let rotate = |x: f32, y: f32| -> Pos2 {
            Pos2::new(
                pos.x + (x * cos_a - y * sin_a),
                pos.y + (x * sin_a + y * cos_a),
            )
        };
        
        let p1 = rotate(0.0, -size);
        let p2 = rotate(-half_size, half_size);
        let p3 = rotate(half_size, half_size);
        
        painter.add(egui::Shape::convex_polygon(
            vec![p1, p2, p3],
            color,
            egui::Stroke::NONE,
        ));
    }
}
