use bevy::prelude::{
    default, AssetServer, BuildChildren, ButtonBundle, Camera2dBundle, Color, Commands, NodeBundle,
    Res, TextBundle,
};
use bevy::text::{Text, TextStyle};
use bevy::ui::{AlignItems, Display, FlexDirection, Style, UiRect};
use bevy_cosmic_edit::*;

#[derive(bevy::prelude::Component)]
pub struct Portal;

#[derive(bevy::prelude::Component)]
pub struct AddressBar;

#[derive(bevy::prelude::Component)]
pub struct RefreshButton;

#[derive(bevy::prelude::Component)]
pub struct MainCamera;

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

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon_font = asset_server.load("fonts/MaterialIcons-Regular-subset.ttf");
    commands.spawn((Camera2dBundle::default(), MainCamera));
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
                text_setter: CosmicText::OneStyle("velo-studio.xyz/rust.wasm".to_string()),
                text_position: CosmicTextPosition::Left { padding: 20 },
                mode: CosmicMode::InfiniteLine,
                ..default()
            },
            CosmicEditPlaceholderBundle {
                text_setter: PlaceholderText(CosmicText::OneStyle("Enter URL...".into())),
                attrs: PlaceholderAttrs(placeholder_attrs.clone()),
            },
            AddressBar,
        ))
        .id();
    let edit = commands
        .spawn((
            ButtonBundle {
                border_color: Color::hex("#ededed").unwrap().into(),
                style: Style {
                    border: UiRect::all(bevy::prelude::Val::Px(3.)),
                    width: bevy::prelude::Val::Percent(100.),
                    height: bevy::prelude::Val::Px(40.),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            CosmicSource(editor),
        ))
        .id();
    commands.insert_resource(Focus(Some(editor)));
    let portal = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: bevy::prelude::Val::Percent(100.),
                    height: bevy::prelude::Val::Percent(100.),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            Portal,
        ))
        .id();
    let refresh_button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::NONE.into(),
                ..default()
            },
            RefreshButton,
        ))
        .id();
    let icon = commands
        .spawn(TextBundle {
            text: Text::from_section(
                "\u{E5D5}".to_string(),
                TextStyle {
                    font: icon_font.into(),
                    font_size: 42.0,
                    color: Color::GRAY.into(),
                },
            ),
            ..default()
        })
        .id();
    commands.entity(refresh_button).add_child(icon);
    let panel = commands
        .spawn(NodeBundle {
            style: Style {
                width: bevy::ui::Val::Percent(50.),
                ..default()
            },
            ..default()
        })
        .id();

    commands.entity(panel).add_child(edit);
    commands.entity(panel).add_child(refresh_button);

    commands.entity(root).add_child(panel);
    commands.entity(root).add_child(portal);
}
