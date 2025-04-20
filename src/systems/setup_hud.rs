use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn get_toggle_overlay_text(is_overlay_shown: bool) -> String {
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

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(75.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            make_hud_button(
                parent,
                &ButtonVariant::Toggle(
                    false,
                    String::from("Show solution"),
                    String::from("Hide solution"),
                ),
                HudAction::ToggleOverlay,
            );
            make_hud_button(
                parent,
                &ButtonVariant::Normal(String::from("Rando-maze")),
                HudAction::RandomiseSeed,
            );
        });
}

pub fn make_hud_button(builder: &mut ChildBuilder, variant: &ButtonVariant, action: HudAction) {
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

    let button_text = match variant {
        ButtonVariant::Normal(text) => text,
        ButtonVariant::Toggle(initial_state, inactive_text, active_text) => {
            if *initial_state {
                active_text
            } else {
                inactive_text
            }
        }
    };

    builder
        .spawn((
            Button,
            button_node,
            BackgroundColor(NORMAL_BUTTON),
            variant.clone(),
            action,
        ))
        .with_children(|parent| {
            parent.spawn((Text::new(button_text), button_text_font));
        });
}

// This system handles changing all buttons color based on mouse interaction
pub fn update_button_state(
    mut interaction_query: Query<
        (&Interaction, &mut ButtonVariant, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut button_variant, mut background_color) in &mut interaction_query {
        let variant = button_variant.clone();
        *background_color = match *interaction {
            Interaction::Pressed => match variant {
                ButtonVariant::Normal(_) => HOVERED_BUTTON.into(),
                ButtonVariant::Toggle(state, _, _) => {
                    if state {
                        // button is switching from active to inactive.
                        HOVERED_BUTTON.into()
                    } else {
                        // button is switching from inactive to active.
                        HOVERED_PRESSED_BUTTON.into()
                    }
                }
            },
            Interaction::Hovered => match variant {
                ButtonVariant::Normal(_) => HOVERED_BUTTON.into(),
                ButtonVariant::Toggle(state, _, _) => {
                    if state {
                        // button is active.
                        HOVERED_PRESSED_BUTTON.into()
                    } else {
                        // button is inactive.
                        HOVERED_BUTTON.into()
                    }
                }
            },
            Interaction::None => match variant {
                ButtonVariant::Normal(_) => NORMAL_BUTTON.into(),
                ButtonVariant::Toggle(state, _, _) => {
                    if state {
                        // button is active.
                        PRESSED_BUTTON.into()
                    } else {
                        // button is inactive.
                        NORMAL_BUTTON.into()
                    }
                }
            },
        };

        if let ButtonVariant::Toggle(state, inactive_text, active_text) = variant {
            if *interaction == Interaction::Pressed {
                *button_variant = ButtonVariant::Toggle(!state, inactive_text, active_text);
            }
        }
    }
}

pub fn hud_action(
    interaction_query: Query<
        (&Interaction, &HudAction, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut overlay_state: ResMut<OverlayState>,
) {
    for (interaction, hud_action, children) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match hud_action {
                HudAction::ToggleOverlay => {
                    overlay_state.0 = !overlay_state.0;
                }
                HudAction::RandomiseSeed => {}
                HudAction::ChangeMazeType(maze_type) => {}
            }
        }
    }
}
