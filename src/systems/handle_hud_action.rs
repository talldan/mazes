use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use fastrand;

pub fn handle_hud_action(
    interaction_query: Query<(&Interaction, &HudAction), (Changed<Interaction>, With<Button>)>,
    mut overlay_state: ResMut<OverlayState>,
    mut rng_seed: ResMut<RngSeed>,
    mut maze_builder_type: ResMut<MazeBuilderType>,
) {
    for (interaction, hud_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match hud_action {
                HudAction::ToggleOverlay => {
                    overlay_state.0 = !overlay_state.0;
                }
                HudAction::RandomiseSeed => {
                    rng_seed.0 = fastrand::u64(..u64::MAX);
                }
                HudAction::ChangeMazeType(new_maze_type) => {
                    *maze_builder_type = *new_maze_type;
                }
                HudAction::None => (),
            }
        }
    }
}
