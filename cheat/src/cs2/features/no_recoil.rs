use crate::{
    config::Config,
    cs2::{CS2, entity::player::Player},
};

impl CS2 {
    pub fn no_recoil(&self, config: &Config) {
        if !config.misc.no_recoil {
            return;
        }

        let Some(local_player) = Player::local_player(self) else {
            return;
        };

        let Some(weapon) = local_player.weapon_address(self) else {
            return;
        };

        let current_penalty: f32 = self
            .process
            .read(weapon + self.offsets.weapon.accuracy_penalty);
        if current_penalty != 0.0 {
            self.process
                .write(weapon + self.offsets.weapon.accuracy_penalty, 0.0f32);
        }
    }
}
