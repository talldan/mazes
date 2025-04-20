use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn get_toggle_overlay_text<'a>(is_overlay_shown: bool) -> String {
    if is_overlay_shown {
        String::from("Hide solution")
    } else {
        String::from("Show solution")
    }
}

pub fn setup_hud(mut commands: Commands, overlay_state: Res<OverlayState>) {
    let button_node = Node {
        width: Val::Px(150.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_font = TextFont {
        font_size: 16.0,
        ..default()
    };

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(75.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    HudAction::ToggleOverlay,
                ))
                .with_children(|parent| {
                    let toggle_overlay_value = get_toggle_overlay_text(overlay_state.0);
                    parent.spawn((
                        ToggleOverlayText,
                        Text::new(toggle_overlay_value),
                        button_text_font.clone(),
                    ));
                });
        });
}

// This system handles changing all buttons color based on mouse interaction
pub fn button_state_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        *background_color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}

pub fn hud_action(
    interaction_query: Query<
        (&Interaction, &HudAction, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut overlay_state: ResMut<OverlayState>,
) {
    for (interaction, hud_action, children) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match hud_action {
                HudAction::ToggleOverlay => {
                    overlay_state.0 = !overlay_state.0;
                    let mut text = text_query.get_mut(children[0]).unwrap();
                    let new_text = get_toggle_overlay_text(overlay_state.0);
                    **text = new_text;
                }
                _ => {}
            }
        }
    }
}
