use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use fastrand;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn setup_hud(mut commands: Commands, overlay_state: Res<OverlayState>) {
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
                ButtonVariant::Normal,
                HudAction::RandomiseSeed,
                String::from("Rando-maze"),
            );
            make_hud_button(
                parent,
                ButtonVariant::Toggle(overlay_state.0),
                HudAction::ToggleOverlay,
                String::from("Show solution"),
            );
        });
}

pub fn make_hud_button(
    builder: &mut ChildBuilder,
    variant: ButtonVariant,
    action: HudAction,
    text: String,
) {
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

    builder
        .spawn((
            Button,
            button_node,
            BackgroundColor(NORMAL_BUTTON),
            variant,
            action,
        ))
        .with_children(|parent| {
            parent.spawn((Text::new(text), button_text_font));
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
        *background_color = match *interaction {
            Interaction::Pressed => match *button_variant {
                ButtonVariant::Normal => HOVERED_BUTTON.into(),
                ButtonVariant::Toggle(state) => {
                    if state {
                        // button is switching from active to inactive.
                        HOVERED_BUTTON.into()
                    } else {
                        // button is switching from inactive to active.
                        HOVERED_PRESSED_BUTTON.into()
                    }
                }
            },
            Interaction::Hovered => match *button_variant {
                ButtonVariant::Normal => HOVERED_BUTTON.into(),
                ButtonVariant::Toggle(state) => {
                    if state {
                        // button is active.
                        HOVERED_PRESSED_BUTTON.into()
                    } else {
                        // button is inactive.
                        HOVERED_BUTTON.into()
                    }
                }
            },
            Interaction::None => match *button_variant {
                ButtonVariant::Normal => NORMAL_BUTTON.into(),
                ButtonVariant::Toggle(state) => {
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

        if let ButtonVariant::Toggle(state) = *button_variant {
            if *interaction == Interaction::Pressed {
                // Toggle the state.
                *button_variant = ButtonVariant::Toggle(!state);
            }
        }
    }
}

pub fn hud_action(
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
            }
        }
    }
}
