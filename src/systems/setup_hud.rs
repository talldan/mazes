use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 50.0;

fn get_maze_builder_name(maze_builder_type: MazeBuilderType) -> String {
    match maze_builder_type {
        MazeBuilderType::BinaryTree => String::from("Binary Tree"),
        MazeBuilderType::Sidewinder => String::from("Sidewinder"),
        MazeBuilderType::AldousBroder => String::from("Aldous Broder"),
        MazeBuilderType::Wilson => String::from("Wilson"),
    }
}

pub fn setup_hud(
    mut commands: Commands,
    overlay_state: Res<OverlayState>,
    maze_builder_type: Res<MazeBuilderType>,
) {
    commands
        .spawn(Node {
            width: Val::Vw(100.0),
            height: Val::Px(75.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
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

            make_dropdown(
                parent,
                vec![
                    (
                        get_maze_builder_name(MazeBuilderType::BinaryTree),
                        HudAction::ChangeMazeType(MazeBuilderType::BinaryTree),
                    ),
                    (
                        get_maze_builder_name(MazeBuilderType::Sidewinder),
                        HudAction::ChangeMazeType(MazeBuilderType::Sidewinder),
                    ),
                    (
                        get_maze_builder_name(MazeBuilderType::AldousBroder),
                        HudAction::ChangeMazeType(MazeBuilderType::AldousBroder),
                    ),
                    (
                        get_maze_builder_name(MazeBuilderType::Wilson),
                        HudAction::ChangeMazeType(MazeBuilderType::Wilson),
                    ),
                ],
                get_maze_builder_name(*maze_builder_type),
                String::from("Maze type"),
            );
        });
}

pub fn make_hud_button(
    builder: &mut ChildBuilder,
    variant: ButtonVariant,
    action: HudAction,
    text: String,
) -> Entity {
    let button_node = Node {
        width: Val::Px(BUTTON_WIDTH),
        height: Val::Px(BUTTON_HEIGHT),
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
        })
        .id()
}

fn make_dropdown(
    builder: &mut ChildBuilder,
    options: Vec<(String, HudAction)>,
    active_option: String,
    label: String,
) {
    // Parent everything within a 'Dropdown' node.
    builder
        .spawn((
            Dropdown,
            Node {
                width: Val::Px(BUTTON_WIDTH),
                height: Val::Px(BUTTON_HEIGHT),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip_x(),
                ..default()
            },
        ))
        .with_children(|dropdown| {
            // The 'Dropdown' contains an opener button.
            make_hud_button(
                dropdown,
                ButtonVariant::DropdownOpener(false),
                HudAction::None,
                label,
            );

            let option_count = options.len() as f32;

            // And a 'DropdownMenu', which is hidden by default.
            dropdown
                .spawn((
                    DropdownMenu,
                    RadioGroup,
                    Node {
                        top: Val::Px(BUTTON_HEIGHT + 10.0),
                        width: Val::Px(BUTTON_WIDTH),
                        height: Val::Px(option_count * BUTTON_HEIGHT),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    Visibility::Hidden,
                ))
                .with_children(|dropdown_menu| {
                    // The 'DropdownMenu' contains each option.
                    options.into_iter().for_each(|(option_text, action)| {
                        let is_active = active_option == option_text;
                        make_hud_button(
                            dropdown_menu,
                            ButtonVariant::Radio(is_active),
                            action,
                            option_text,
                        );
                    });
                });
        });
}

// This system handles changing all buttons color based on mouse interaction.
// TODO - radio and toggle buttons should probably be updated as a result of their
// state changing rather than only the mouse input. This would allow limiting of this
// query to only Changed<Interaction>.
pub fn update_button_colors(
    mut interaction_query: Query<
        (&Interaction, &ButtonVariant, &mut BackgroundColor),
        With<Button>,
    >,
) {
    for (interaction, button_variant, mut background_color) in &mut interaction_query {
        *background_color = match *interaction {
            Interaction::Pressed => match *button_variant {
                ButtonVariant::Normal => HOVERED_BUTTON.into(),
                ButtonVariant::Toggle(state)
                | ButtonVariant::Radio(state)
                | ButtonVariant::DropdownOpener(state) => {
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
                ButtonVariant::Toggle(state)
                | ButtonVariant::Radio(state)
                | ButtonVariant::DropdownOpener(state) => {
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
                ButtonVariant::Toggle(state)
                | ButtonVariant::Radio(state)
                | ButtonVariant::DropdownOpener(state) => {
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
    }
}

// This system handles changing all buttons color based on mouse interaction
pub fn update_toggle_button_state(
    mut interaction_query: Query<
        (&Interaction, &mut ButtonVariant),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut button_variant) in &mut interaction_query {
        if let ButtonVariant::Toggle(state) = *button_variant {
            if *interaction == Interaction::Pressed {
                // Toggle the state.
                *button_variant = ButtonVariant::Toggle(!state);
            }
        }
    }
}

// This system handles opening and closing dropdown menus.
pub fn update_dropdown_menu_open_state(
    mut interaction_query: Query<
        (&Interaction, &mut ButtonVariant, &Parent),
        (Changed<Interaction>, With<Button>),
    >,
    dropdown_children_query: Query<&Children, With<Dropdown>>,
    mut dropdown_menu_query: Query<&mut Visibility, With<DropdownMenu>>,
) {
    for (interaction, mut button_variant, parent) in &mut interaction_query {
        if let ButtonVariant::DropdownOpener(is_open) = *button_variant {
            if *interaction == Interaction::Pressed {
                let dropdown_children = dropdown_children_query.get(parent.get()).unwrap();
                let mut dropdown_menu_visibility =
                    dropdown_menu_query.get_mut(dropdown_children[1]).unwrap();

                // The `is_open` logic is reversed here, as the state hasn't changed
                // yet after the button press. That happens below.
                *dropdown_menu_visibility = if !is_open {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };

                // Toggle the state.
                *button_variant = ButtonVariant::DropdownOpener(!is_open);
                return;
            }
        }
    }
}

// This system handles toggling radio buttons.
pub fn update_radio_state(
    mut interaction_query: Query<(&Interaction, &mut ButtonVariant, &Parent), With<Button>>,
    radio_group_query: Query<&Children, With<RadioGroup>>,
) {
    // Find a currently unpressed radio button that has been pressed.
    let pressed_radio_button = interaction_query
        .iter()
        .find(|(interaction, button_variant, _)| {
            **interaction == Interaction::Pressed && **button_variant == ButtonVariant::Radio(false)
        });

    // If there isn't one, return early, there's nothing to do.
    if None == pressed_radio_button {
        return;
    }

    // Get the parent entity of the pressed button, this will be a RadioGroup.
    let (_, _, parent) = pressed_radio_button.unwrap();
    // Get the children of the radio group, each will be a 'ButtonVariant::Radio.
    let radio_group_children = radio_group_query.get(parent.get());

    // Loop through each button in the group and modify its state.
    // If the button was pressed, set it to active and set all others to inactive.
    if let Ok(radio_buttons) = radio_group_children {
        radio_buttons.iter().for_each(|button_entity| {
            let button = interaction_query.get_mut(*button_entity);

            if let Ok((interaction, mut button_variant, _)) = button {
                *button_variant = if *interaction == Interaction::Pressed {
                    ButtonVariant::Radio(true)
                } else {
                    ButtonVariant::Radio(false)
                }
            }
        })
    }
}
