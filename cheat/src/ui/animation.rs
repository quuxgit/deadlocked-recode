use egui::Context;
use std::time::Instant;

#[derive(Clone, Copy)]
pub struct AnimationState {
    pub value: f32,
    target: f32,
    last_update: Option<Instant>,
    speed: f32,
}

impl AnimationState {
    pub fn new(speed: f32) -> Self {
        Self {
            value: 0.0,
            target: 0.0,
            last_update: None,
            speed,
        }
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target.clamp(0.0, 1.0);
    }

    pub fn update(&mut self, ctx: &Context) -> bool {
        let now = Instant::now();
        let dt = self
            .last_update
            .map(|last| now.duration_since(last).as_secs_f32())
            .unwrap_or(0.016);
        self.last_update = Some(now);

        let diff = self.target - self.value;
        if diff.abs() < 0.001 {
            self.value = self.target;
            return false;
        }

        self.value += diff * self.speed * dt * 60.0;
        ctx.request_repaint();
        true
    }

    pub fn ease_in_out_cubic(&self) -> f32 {
        let t = self.value;
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }
}

pub struct SlideAnimation {
    state: AnimationState,
}

impl SlideAnimation {
    pub fn new() -> Self {
        Self {
            state: AnimationState::new(10.0),
        }
    }

    pub fn update(&mut self, ctx: &Context, visible: bool) -> f32 {
        self.state.set_target(if visible { 1.0 } else { 0.0 });
        self.state.update(ctx);
        self.state.ease_in_out_cubic()
    }
}
