use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn update_overlay_visibility(
    overlay_state: Res<OverlayState>,
    mut overlay_query: Query<(&mut Visibility, &OverlayVisibility)>,
) {
    let is_overlay_active = overlay_state.0;
    for (mut visibility, overlay_visibility) in &mut overlay_query {
        let is_entity_visible_when_overlay_active = overlay_visibility.0;
        let make_visible = (is_overlay_active && is_entity_visible_when_overlay_active)
            || (!is_overlay_active && !is_entity_visible_when_overlay_active);

        if make_visible {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
