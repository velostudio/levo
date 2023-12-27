use bevy::prelude::{
    default, BuildChildren, ButtonBundle, Camera2dBundle, Color, Commands, NodeBundle,
};
use bevy::ui::{AlignItems, Display, FlexDirection, Style, UiRect};
use bevy_cosmic_edit::*;

#[derive(bevy::prelude::Component)]
pub struct Portal;

pub fn bevy_color_to_cosmic(color: bevy::prelude::Color) -> CosmicColor {
    CosmicColor::rgba(
        (color.r() * 255.) as u8,
        (color.g() * 255.) as u8,
        (color.b() * 255.) as u8,
        (color.a() * 255.) as u8,
    )
}

pub fn string_to_bevy_color(str: String) -> bevy::prelude::Color {
    match str.as_str() {
        "white" => Color::WHITE,
        "black" => Color::BLACK,
        "red" => Color::RED,
        "blue" => Color::BLUE,
        "royal_purple" => Color::hex("#8C49A3").unwrap(),
        _ => Color::BLACK,
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                width: bevy::prelude::Val::Percent(100.),
                height: bevy::prelude::Val::Percent(100.),
                display: Display::Flex,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .id();
    let attrs =
        AttrsOwned::new(Attrs::new().color(bevy_color_to_cosmic(Color::hex("4d4d4d").unwrap())));
    let placeholder_attrs =
        AttrsOwned::new(Attrs::new().color(bevy_color_to_cosmic(Color::hex("#e6e6e6").unwrap())));
    let editor = commands
        .spawn((
            CosmicEditBundle {
                attrs: CosmicAttrs(attrs.clone()),
                metrics: CosmicMetrics {
                    font_size: 18.,
                    line_height: 1.2 * 18.,
                    ..default()
                },
                max_lines: CosmicMaxLines(1),
                text_setter: CosmicText::OneStyle("localhost".to_string()),
                text_position: CosmicTextPosition::Left { padding: 20 },
                mode: CosmicMode::InfiniteLine,
                ..default()
            },
            CosmicEditPlaceholderBundle {
                text_setter: PlaceholderText(CosmicText::OneStyle("Enter host...".into())),
                attrs: PlaceholderAttrs(placeholder_attrs.clone()),
            },
        ))
        .id();
    let edit = commands
        .spawn(ButtonBundle {
            border_color: Color::hex("#ededed").unwrap().into(),
            style: Style {
                border: UiRect::all(bevy::prelude::Val::Px(3.)),
                width: bevy::prelude::Val::Percent(30.),
                height: bevy::prelude::Val::Px(40.),
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        })
        .insert(CosmicSource(editor))
        .id();
    commands.insert_resource(Focus(Some(editor)));
    let portal = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: bevy::prelude::Val::Percent(90.),
                    height: bevy::prelude::Val::Percent(90.),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            Portal,
        ))
        .id();
    commands.entity(root).add_child(edit);
    commands.entity(root).add_child(portal);
}
