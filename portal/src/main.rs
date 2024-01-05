use bevy::ecs::schedule::IntoSystemConfigs;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::MouseButton;
use bevy::prelude::{
    default, App, BuildChildren, ButtonBundle, Camera, Changed, Color, Commands,
    DespawnRecursiveExt, Entity, First, GlobalTransform, Input, KeyCode, PostUpdate, PreUpdate,
    Query, Res, ResMut, Resource, SpatialBundle, Startup, TextBundle, Transform, Update, Vec2,
    Vec3, Visibility, With,
};
use bevy::text::{Text, Text2dBundle, TextSection, TextStyle};
use bevy::time::Time;
use bevy::ui::{BackgroundColor, Interaction, Style};
use bevy::window::{CursorIcon, PrimaryWindow, Window};
use bevy::DefaultPlugins;
use bevy_cosmic_edit::*;

use bevy_prototype_lyon::prelude::{Fill, GeometryBuilder, PathBuilder, ShapeBundle, ShapePlugin};
use bevy_prototype_lyon::shapes::{Rectangle, RectangleOrigin};
use bevy_tokio_tasks::TokioTasksRuntime;
use brotli::Decompressor;
use clap::Parser;
use levo::portal::my_imports::Host;
use std::io::Read;
use std::path::{Path, PathBuf};
use url::Url;
use wasmtime::{component::*, StoreLimits, StoreLimitsBuilder};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::command::sync;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};

#[path = "ui.rs"]
mod ui;
pub use ui::*;

bindgen!({
    world: "my-world",
    path: "../spec",
    async: false,
});

/// Levo Portal
#[derive(Parser, Debug, Resource)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Allow read access to this path
    #[arg(short, long)]
    allow_read: Option<PathBuf>,
    /// Don't show the "chrome" (not yet implemented)
    #[arg(short, long)]
    bare: bool,
    /// Path to the WASM file to run directly (not yet implemented)
    #[arg(short, long)]
    run: Option<PathBuf>,
    /// URL to launch the portal with (not yet implemented)
    #[arg(short, long)]
    url: Option<String>,
    /// Path to the WASM file to watch, run and reload on changes (not yet implemented)
    #[arg(short, long)]
    watch: Option<PathBuf>,
}

#[derive(Debug)]
struct FillRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(PartialEq, Debug)]
struct Arc {
    x: f32,
    y: f32,
    radius: f32,
    sweep_angle: f32,
    x_rotation: f32,
}

#[derive(PartialEq, Debug)]
struct CubicBezierTo {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
}

#[derive(PartialEq, Debug)]
struct Link {
    url: String,
    text: String,
    x: f32,
    y: f32,
    size: f32,
}

#[derive(PartialEq, Debug)]
struct Label {
    text: String,
    x: f32,
    y: f32,
    size: f32,
    color: String,
}

#[derive(Debug)]
enum HostEvent {
    Arc(Arc),
    BeginPath,
    ClosePath,
    CubicBezierTo(CubicBezierTo),
    Fill,
    FillRect(FillRect),
    FillStyle(String),
    Label(Label),
    Link(Link),
    MoveTo((f32, f32)),
}

#[derive(Default)]
struct Inputs {
    keys_just_pressed: Vec<KeyCode>,
    keys_pressed: Vec<KeyCode>,
    keys_just_released: Vec<KeyCode>,
    mouse_buttons_just_pressed: Vec<MouseButton>,
    mouse_buttons_just_released: Vec<MouseButton>,
    mouse_buttons_pressed: Vec<MouseButton>,
    cursor_position: Option<Vec2>,
}

#[derive(Debug)]
struct Canvas {
    size: Vec2,
    position: Vec2,
}

struct MyCtx {
    table: Table,
    wasi: WasiCtx,
    queue: Vec<HostEvent>,
    delta_seconds: f32,
    limits: StoreLimits,
    inputs: Inputs,
    canvas: Canvas,
    allow_read: Option<PathBuf>,
}

impl WasiView for MyCtx {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

// #[async_trait::async_trait]
impl Host for MyCtx {
    fn print(&mut self, from_wasm: String) -> wasmtime::Result<()> {
        dbg!(from_wasm);
        Ok(())
    }

    fn fill_style(&mut self, color: String) -> wasmtime::Result<()> {
        self.queue.push(HostEvent::FillStyle(color));
        Ok(())
    }

    fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) -> wasmtime::Result<()> {
        self.queue.push(HostEvent::FillRect(FillRect {
            x: x - self.canvas.position.x,
            y: y - self.canvas.position.y,
            width,
            height,
        }));
        Ok(())
    }

    fn begin_path(&mut self) -> wasmtime::Result<()> {
        self.queue.push(HostEvent::BeginPath);
        Ok(())
    }

    fn arc(
        &mut self,
        x: f32,
        y: f32,
        radius: f32,
        sweep_angle: f32,
        x_rotation: f32,
    ) -> wasmtime::Result<()> {
        self.queue.push(HostEvent::Arc(Arc {
            x: x - self.canvas.position.x,
            y: y - self.canvas.position.y,
            radius,
            sweep_angle,
            x_rotation,
        }));
        Ok(())
    }

    fn close_path(&mut self) -> wasmtime::Result<()> {
        self.queue.push(HostEvent::ClosePath);
        Ok(())
    }

    fn fill(&mut self) -> wasmtime::Result<()> {
        self.queue.push(HostEvent::Fill);
        Ok(())
    }

    fn move_to(&mut self, x: f32, y: f32) -> wasmtime::Result<()> {
        self.queue.push(HostEvent::MoveTo((
            x - self.canvas.position.x,
            y - self.canvas.position.y,
        )));
        Ok(())
    }

    fn cubic_bezier_to(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> wasmtime::Result<()> {
        self.queue.push(HostEvent::CubicBezierTo(CubicBezierTo {
            x1: x1 - self.canvas.position.x,
            y1: y1 - self.canvas.position.y,
            x2: x2 - self.canvas.position.x,
            y2: y2 - self.canvas.position.y,
            x3: x3 - self.canvas.position.x,
            y3: y3 - self.canvas.position.y,
        }));
        Ok(())
    }

    fn link(
        &mut self,
        url: String,
        text: String,
        x: f32,
        y: f32,
        size: f32,
    ) -> wasmtime::Result<()> {
        self.queue.push(HostEvent::Link(Link {
            url,
            text,
            x: x - self.canvas.position.x,
            y: y - self.canvas.position.y,
            size,
        }));
        Ok(())
    }

    fn label(
        &mut self,
        text: String,
        x: f32,
        y: f32,
        size: f32,
        color: String,
    ) -> wasmtime::Result<()> {
        self.queue.push(HostEvent::Label(Label {
            text,
            x: x - self.canvas.position.x,
            y: y - self.canvas.position.y,
            size,
            color,
        }));
        Ok(())
    }

    fn delta_seconds(&mut self) -> wasmtime::Result<f32> {
        Ok(self.delta_seconds)
    }

    fn key_just_pressed(
        &mut self,
        key: levo::portal::my_imports::KeyCode,
    ) -> wasmtime::Result<bool> {
        Ok(self.inputs.keys_just_pressed.contains(&key.into()))
    }

    fn key_pressed(&mut self, key: levo::portal::my_imports::KeyCode) -> wasmtime::Result<bool> {
        Ok(self.inputs.keys_pressed.contains(&key.into()))
    }

    fn key_just_released(
        &mut self,
        key: levo::portal::my_imports::KeyCode,
    ) -> wasmtime::Result<bool> {
        Ok(self.inputs.keys_just_released.contains(&key.into()))
    }

    fn mouse_button_just_pressed(
        &mut self,
        btn: levo::portal::my_imports::MouseButton,
    ) -> wasmtime::Result<bool> {
        Ok(self.inputs.mouse_buttons_just_pressed.contains(&btn.into()))
    }

    fn mouse_button_just_released(
        &mut self,
        btn: levo::portal::my_imports::MouseButton,
    ) -> wasmtime::Result<bool> {
        Ok(self
            .inputs
            .mouse_buttons_just_released
            .contains(&btn.into()))
    }

    fn mouse_button_pressed(
        &mut self,
        btn: levo::portal::my_imports::MouseButton,
    ) -> wasmtime::Result<bool> {
        Ok(self.inputs.mouse_buttons_pressed.contains(&btn.into()))
    }

    fn cursor_position(&mut self) -> wasmtime::Result<Option<levo::portal::my_imports::Position>> {
        Ok(self.inputs.cursor_position.map(Into::into))
    }

    fn canvas_size(&mut self) -> wasmtime::Result<levo::portal::my_imports::Size> {
        Ok(levo::portal::my_imports::Size {
            width: self.canvas.size.x,
            height: self.canvas.size.y,
        })
    }

    fn read_file(&mut self, path: String) -> wasmtime::Result<Result<Vec<u8>, ()>> {
        if let Some(allow_read) = self.allow_read.as_ref() {
            let canonicalized_allow_read = match canonicalize_path(Path::new(allow_read)) {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Error canonicalizing allow_read path: {}", e.to_string());
                    return Ok(Err(()));
                }
            };

            let full_path = canonicalized_allow_read.join(&path);
            let canonicalized_full_path = match canonicalize_path(&full_path) {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Error canonicalizing full path: {}", e.to_string());
                    return Ok(Err(()));
                }
            };

            if is_path_within_allowed_directory(&canonicalized_allow_read, &canonicalized_full_path)
            {
                match std::fs::read(&canonicalized_full_path) {
                    Ok(content) => Ok(Ok(content)),
                    Err(e) => {
                        eprintln!("{}", e);
                        Ok(Err(()))
                    }
                }
            } else {
                eprintln!(
                    "Path is not within allowed directory. Allowed: {}. Path: {}",
                    &canonicalized_allow_read.display(),
                    &canonicalized_full_path.display()
                );
                Ok(Err(()))
            }
        } else {
            eprintln!("read_file is not allowed");
            Ok(Err(()))
        }
    }
}

impl From<KeyCode> for levo::portal::my_imports::KeyCode {
    fn from(value: KeyCode) -> Self {
        use KeyCode as Other;
        match value {
            Other::Key1 => Self::Key1,
            Other::Key2 => Self::Key2,
            Other::Key3 => Self::Key3,
            Other::Key4 => Self::Key4,
            Other::Key5 => Self::Key5,
            Other::Key6 => Self::Key6,
            Other::Key7 => Self::Key7,
            Other::Key8 => Self::Key8,
            Other::Key9 => Self::Key9,
            Other::Key0 => Self::Key0,
            Other::A => Self::A,
            Other::B => Self::B,
            Other::C => Self::C,
            Other::D => Self::D,
            Other::E => Self::E,
            Other::F => Self::F,
            Other::G => Self::G,
            Other::H => Self::H,
            Other::I => Self::I,
            Other::J => Self::J,
            Other::K => Self::K,
            Other::L => Self::L,
            Other::M => Self::M,
            Other::N => Self::N,
            Other::O => Self::O,
            Other::P => Self::P,
            Other::Q => Self::Q,
            Other::R => Self::R,
            Other::S => Self::S,
            Other::T => Self::T,
            Other::U => Self::U,
            Other::V => Self::V,
            Other::W => Self::W,
            Other::X => Self::X,
            Other::Y => Self::Y,
            Other::Z => Self::Z,
            Other::Escape => Self::Escape,
            Other::F1 => Self::F1,
            Other::F2 => Self::F2,
            Other::F3 => Self::F3,
            Other::F4 => Self::F4,
            Other::F5 => Self::F5,
            Other::F6 => Self::F6,
            Other::F7 => Self::F7,
            Other::F8 => Self::F8,
            Other::F9 => Self::F9,
            Other::F10 => Self::F10,
            Other::F11 => Self::F11,
            Other::F12 => Self::F12,
            Other::F13 => Self::F13,
            Other::F14 => Self::F14,
            Other::F15 => Self::F15,
            Other::F16 => Self::F16,
            Other::F17 => Self::F17,
            Other::F18 => Self::F18,
            Other::F19 => Self::F19,
            Other::F20 => Self::F20,
            Other::F21 => Self::F21,
            Other::F22 => Self::F22,
            Other::F23 => Self::F23,
            Other::F24 => Self::F24,
            Other::Snapshot => Self::Snapshot,
            Other::Scroll => Self::Scroll,
            Other::Pause => Self::Pause,
            Other::Insert => Self::Insert,
            Other::Home => Self::Home,
            Other::Delete => Self::Delete,
            Other::End => Self::End,
            Other::PageDown => Self::PageDown,
            Other::PageUp => Self::PageUp,
            Other::Left => Self::Left,
            Other::Up => Self::Up,
            Other::Right => Self::Right,
            Other::Down => Self::Down,
            Other::Back => Self::Back,
            Other::Return => Self::Return,
            Other::Space => Self::Space,
            Other::Compose => Self::Compose,
            Other::Caret => Self::Caret,
            Other::Numlock => Self::Numlock,
            Other::Numpad0 => Self::Numpad0,
            Other::Numpad1 => Self::Numpad1,
            Other::Numpad2 => Self::Numpad2,
            Other::Numpad3 => Self::Numpad3,
            Other::Numpad4 => Self::Numpad4,
            Other::Numpad5 => Self::Numpad5,
            Other::Numpad6 => Self::Numpad6,
            Other::Numpad7 => Self::Numpad7,
            Other::Numpad8 => Self::Numpad8,
            Other::Numpad9 => Self::Numpad9,
            Other::AbntC1 => Self::AbntC1,
            Other::AbntC2 => Self::AbntC2,
            Other::NumpadAdd => Self::NumpadAdd,
            Other::Apostrophe => Self::Apostrophe,
            Other::Apps => Self::Apps,
            Other::Asterisk => Self::Asterisk,
            Other::Plus => Self::Plus,
            Other::At => Self::At,
            Other::Ax => Self::Ax,
            Other::Backslash => Self::Backslash,
            Other::Calculator => Self::Calculator,
            Other::Capital => Self::Capital,
            Other::Colon => Self::Colon,
            Other::Comma => Self::Comma,
            Other::Convert => Self::Convert,
            Other::NumpadDecimal => Self::NumpadDecimal,
            Other::NumpadDivide => Self::NumpadDivide,
            Other::Equals => Self::Equals,
            Other::Grave => Self::Grave,
            Other::Kana => Self::Kana,
            Other::Kanji => Self::Kanji,
            Other::AltLeft => Self::AltLeft,
            Other::BracketLeft => Self::BracketLeft,
            Other::ControlLeft => Self::ControlLeft,
            Other::ShiftLeft => Self::ShiftLeft,
            Other::SuperLeft => Self::SuperLeft,
            Other::Mail => Self::Mail,
            Other::MediaSelect => Self::MediaSelect,
            Other::MediaStop => Self::MediaStop,
            Other::Minus => Self::Minus,
            Other::NumpadMultiply => Self::NumpadMultiply,
            Other::Mute => Self::Mute,
            Other::MyComputer => Self::MyComputer,
            Other::NavigateForward => Self::NavigateForward,
            Other::NavigateBackward => Self::NavigateBackward,
            Other::NextTrack => Self::NextTrack,
            Other::NoConvert => Self::NoConvert,
            Other::NumpadComma => Self::NumpadComma,
            Other::NumpadEnter => Self::NumpadEnter,
            Other::NumpadEquals => Self::NumpadEquals,
            Other::Oem102 => Self::Oem102,
            Other::Period => Self::Period,
            Other::PlayPause => Self::PlayPause,
            Other::Power => Self::Power,
            Other::PrevTrack => Self::PrevTrack,
            Other::AltRight => Self::AltRight,
            Other::BracketRight => Self::BracketRight,
            Other::ControlRight => Self::ControlRight,
            Other::ShiftRight => Self::ShiftRight,
            Other::SuperRight => Self::SuperRight,
            Other::Semicolon => Self::Semicolon,
            Other::Slash => Self::Slash,
            Other::Sleep => Self::Sleep,
            Other::Stop => Self::Stop,
            Other::NumpadSubtract => Self::NumpadSubtract,
            Other::Sysrq => Self::Sysrq,
            Other::Tab => Self::Tab,
            Other::Underline => Self::Underline,
            Other::Unlabeled => Self::Unlabeled,
            Other::VolumeDown => Self::VolumeDown,
            Other::VolumeUp => Self::VolumeUp,
            Other::Wake => Self::Wake,
            Other::WebBack => Self::WebBack,
            Other::WebFavorites => Self::WebFavorites,
            Other::WebForward => Self::WebForward,
            Other::WebHome => Self::WebHome,
            Other::WebRefresh => Self::WebRefresh,
            Other::WebSearch => Self::WebSearch,
            Other::WebStop => Self::WebStop,
            Other::Yen => Self::Yen,
            Other::Copy => Self::Copy,
            Other::Paste => Self::Paste,
            Other::Cut => Self::Cut,
        }
    }
}

impl From<levo::portal::my_imports::KeyCode> for KeyCode {
    fn from(value: levo::portal::my_imports::KeyCode) -> Self {
        use levo::portal::my_imports::KeyCode as Other;
        match value {
            Other::Key1 => Self::Key1,
            Other::Key2 => Self::Key2,
            Other::Key3 => Self::Key3,
            Other::Key4 => Self::Key4,
            Other::Key5 => Self::Key5,
            Other::Key6 => Self::Key6,
            Other::Key7 => Self::Key7,
            Other::Key8 => Self::Key8,
            Other::Key9 => Self::Key9,
            Other::Key0 => Self::Key0,
            Other::A => Self::A,
            Other::B => Self::B,
            Other::C => Self::C,
            Other::D => Self::D,
            Other::E => Self::E,
            Other::F => Self::F,
            Other::G => Self::G,
            Other::H => Self::H,
            Other::I => Self::I,
            Other::J => Self::J,
            Other::K => Self::K,
            Other::L => Self::L,
            Other::M => Self::M,
            Other::N => Self::N,
            Other::O => Self::O,
            Other::P => Self::P,
            Other::Q => Self::Q,
            Other::R => Self::R,
            Other::S => Self::S,
            Other::T => Self::T,
            Other::U => Self::U,
            Other::V => Self::V,
            Other::W => Self::W,
            Other::X => Self::X,
            Other::Y => Self::Y,
            Other::Z => Self::Z,
            Other::Escape => Self::Escape,
            Other::F1 => Self::F1,
            Other::F2 => Self::F2,
            Other::F3 => Self::F3,
            Other::F4 => Self::F4,
            Other::F5 => Self::F5,
            Other::F6 => Self::F6,
            Other::F7 => Self::F7,
            Other::F8 => Self::F8,
            Other::F9 => Self::F9,
            Other::F10 => Self::F10,
            Other::F11 => Self::F11,
            Other::F12 => Self::F12,
            Other::F13 => Self::F13,
            Other::F14 => Self::F14,
            Other::F15 => Self::F15,
            Other::F16 => Self::F16,
            Other::F17 => Self::F17,
            Other::F18 => Self::F18,
            Other::F19 => Self::F19,
            Other::F20 => Self::F20,
            Other::F21 => Self::F21,
            Other::F22 => Self::F22,
            Other::F23 => Self::F23,
            Other::F24 => Self::F24,
            Other::Snapshot => Self::Snapshot,
            Other::Scroll => Self::Scroll,
            Other::Pause => Self::Pause,
            Other::Insert => Self::Insert,
            Other::Home => Self::Home,
            Other::Delete => Self::Delete,
            Other::End => Self::End,
            Other::PageDown => Self::PageDown,
            Other::PageUp => Self::PageUp,
            Other::Left => Self::Left,
            Other::Up => Self::Up,
            Other::Right => Self::Right,
            Other::Down => Self::Down,
            Other::Back => Self::Back,
            Other::Return => Self::Return,
            Other::Space => Self::Space,
            Other::Compose => Self::Compose,
            Other::Caret => Self::Caret,
            Other::Numlock => Self::Numlock,
            Other::Numpad0 => Self::Numpad0,
            Other::Numpad1 => Self::Numpad1,
            Other::Numpad2 => Self::Numpad2,
            Other::Numpad3 => Self::Numpad3,
            Other::Numpad4 => Self::Numpad4,
            Other::Numpad5 => Self::Numpad5,
            Other::Numpad6 => Self::Numpad6,
            Other::Numpad7 => Self::Numpad7,
            Other::Numpad8 => Self::Numpad8,
            Other::Numpad9 => Self::Numpad9,
            Other::AbntC1 => Self::AbntC1,
            Other::AbntC2 => Self::AbntC2,
            Other::NumpadAdd => Self::NumpadAdd,
            Other::Apostrophe => Self::Apostrophe,
            Other::Apps => Self::Apps,
            Other::Asterisk => Self::Asterisk,
            Other::Plus => Self::Plus,
            Other::At => Self::At,
            Other::Ax => Self::Ax,
            Other::Backslash => Self::Backslash,
            Other::Calculator => Self::Calculator,
            Other::Capital => Self::Capital,
            Other::Colon => Self::Colon,
            Other::Comma => Self::Comma,
            Other::Convert => Self::Convert,
            Other::NumpadDecimal => Self::NumpadDecimal,
            Other::NumpadDivide => Self::NumpadDivide,
            Other::Equals => Self::Equals,
            Other::Grave => Self::Grave,
            Other::Kana => Self::Kana,
            Other::Kanji => Self::Kanji,
            Other::AltLeft => Self::AltLeft,
            Other::BracketLeft => Self::BracketLeft,
            Other::ControlLeft => Self::ControlLeft,
            Other::ShiftLeft => Self::ShiftLeft,
            Other::SuperLeft => Self::SuperLeft,
            Other::Mail => Self::Mail,
            Other::MediaSelect => Self::MediaSelect,
            Other::MediaStop => Self::MediaStop,
            Other::Minus => Self::Minus,
            Other::NumpadMultiply => Self::NumpadMultiply,
            Other::Mute => Self::Mute,
            Other::MyComputer => Self::MyComputer,
            Other::NavigateForward => Self::NavigateForward,
            Other::NavigateBackward => Self::NavigateBackward,
            Other::NextTrack => Self::NextTrack,
            Other::NoConvert => Self::NoConvert,
            Other::NumpadComma => Self::NumpadComma,
            Other::NumpadEnter => Self::NumpadEnter,
            Other::NumpadEquals => Self::NumpadEquals,
            Other::Oem102 => Self::Oem102,
            Other::Period => Self::Period,
            Other::PlayPause => Self::PlayPause,
            Other::Power => Self::Power,
            Other::PrevTrack => Self::PrevTrack,
            Other::AltRight => Self::AltRight,
            Other::BracketRight => Self::BracketRight,
            Other::ControlRight => Self::ControlRight,
            Other::ShiftRight => Self::ShiftRight,
            Other::SuperRight => Self::SuperRight,
            Other::Semicolon => Self::Semicolon,
            Other::Slash => Self::Slash,
            Other::Sleep => Self::Sleep,
            Other::Stop => Self::Stop,
            Other::NumpadSubtract => Self::NumpadSubtract,
            Other::Sysrq => Self::Sysrq,
            Other::Tab => Self::Tab,
            Other::Underline => Self::Underline,
            Other::Unlabeled => Self::Unlabeled,
            Other::VolumeDown => Self::VolumeDown,
            Other::VolumeUp => Self::VolumeUp,
            Other::Wake => Self::Wake,
            Other::WebBack => Self::WebBack,
            Other::WebFavorites => Self::WebFavorites,
            Other::WebForward => Self::WebForward,
            Other::WebHome => Self::WebHome,
            Other::WebRefresh => Self::WebRefresh,
            Other::WebSearch => Self::WebSearch,
            Other::WebStop => Self::WebStop,
            Other::Yen => Self::Yen,
            Other::Copy => Self::Copy,
            Other::Paste => Self::Paste,
            Other::Cut => Self::Cut,
        }
    }
}

impl From<levo::portal::my_imports::MouseButton> for MouseButton {
    fn from(value: levo::portal::my_imports::MouseButton) -> Self {
        use levo::portal::my_imports::MouseButton as Other;
        match value {
            Other::Left => Self::Left,
            Other::Right => Self::Right,
            Other::Middle => Self::Middle,
            Other::Other(inner) => Self::Other(inner),
        }
    }
}

impl From<MouseButton> for levo::portal::my_imports::MouseButton {
    fn from(value: MouseButton) -> Self {
        use MouseButton as Other;
        match value {
            Other::Left => Self::Left,
            Other::Right => Self::Right,
            Other::Middle => Self::Middle,
            Other::Other(inner) => Self::Other(inner),
        }
    }
}

impl From<levo::portal::my_imports::Position> for Vec2 {
    fn from(p: levo::portal::my_imports::Position) -> Self {
        Vec2::new(p.x, p.y)
    }
}

impl From<Vec2> for levo::portal::my_imports::Position {
    fn from(v: Vec2) -> Self {
        levo::portal::my_imports::Position { x: v.x, y: v.y }
    }
}

#[derive(Resource)]
struct WasmStore {
    store: Store<MyCtx>,
}

#[derive(Resource)]
struct WasmBindings {
    bindings: MyWorld,
    first_run: bool,
}

fn main() {
    let args = Args::parse();
    eprintln!("{:?}", &args);

    App::new()
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(args)
        .add_plugins(DefaultPlugins)
        .add_plugins(CosmicEditPlugin::default())
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup)
        .add_systems(First, clear_second_part)
        .add_systems(PreUpdate, clear_first_part)
        .add_systems(Update, handle_get_wasm)
        .add_systems(Update, run_wasm_setup.before(run_wasm_update))
        .add_systems(Update, run_wasm_update)
        .add_systems(Update, handle_guest_event)
        .add_systems(Update, handle_refresh)
        .add_systems(PostUpdate, handle_link)
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin {
            make_runtime: Box::new(|| {
                let mut runtime = tokio::runtime::Builder::new_multi_thread();
                runtime.enable_all();
                runtime.build().unwrap()
            }),
            ..bevy_tokio_tasks::TokioTasksPlugin::default()
        })
        .run();
}

#[derive(PartialEq)]
enum PathCommand {
    MoveTo((f32, f32)),
    CubicBezierTo(CubicBezierTo),
    Arc(Arc),
    Begin,
    Close,
}

#[derive(bevy::prelude::Component)]
struct GuestEntity;

#[derive(bevy::prelude::Component)]
struct DeadEntity;

#[derive(bevy::prelude::Component)]
struct GuestUrl(String);

fn clear_first_part(mut commands: Commands, guest_entites: Query<Entity, With<GuestEntity>>) {
    for entity in guest_entites.iter() {
        commands.entity(entity).remove::<Visibility>();
        commands.entity(entity).remove::<Transform>();
        commands.entity(entity).insert(DeadEntity);
    }
}

fn clear_second_part(mut commands: Commands, guest_entites: Query<Entity, With<DeadEntity>>) {
    for entity in guest_entites.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_guest_event(
    mut commands: Commands,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    canvas_q: Query<&bevy::ui::Node, With<Portal>>,
    wasm_store: Option<ResMut<WasmStore>>,
) {
    let Some(mut wasm_store) = wasm_store else {
        return;
    };
    let (camera, camera_transform) = camera_q.single();
    let canvas_node = canvas_q.single();
    let queue = &mut wasm_store.store.data_mut().queue;
    let mut current_fill = None;
    let mut current_path = Vec::new();
    for r in queue.drain(..) {
        match r {
            HostEvent::FillStyle(c_str) => {
                let c_val = string_to_bevy_color(c_str);
                current_fill = Some(Fill::color(c_val))
            }
            HostEvent::FillRect(FillRect {
                x,
                y,
                width,
                height,
            }) => {
                let rect = Rectangle {
                    extents: Vec2::new(width, height),
                    origin: RectangleOrigin::CustomCenter(Vec2::new(x, y)),
                };
                commands.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&rect),
                        ..default()
                    },
                    current_fill.unwrap_or(Fill::color(Color::RED)),
                    GuestEntity,
                ));
                current_fill = None;
            }
            HostEvent::BeginPath => {
                current_path.push(PathCommand::Begin);
            }
            HostEvent::Arc(arc) => {
                current_path.push(PathCommand::Arc(arc));
            }
            HostEvent::ClosePath => {
                current_path.push(PathCommand::Close);
            }
            HostEvent::Fill => {
                let first = current_path.get(0);

                if let Some(PathCommand::Begin) = first {
                    let mut path_builder = PathBuilder::new();
                    for command in current_path.drain(..).skip(1) {
                        match command {
                            PathCommand::Arc(Arc {
                                x,
                                y,
                                radius,
                                sweep_angle,
                                x_rotation,
                            }) => {
                                let width = canvas_node.size().x;
                                let height = canvas_node.size().y;
                                path_builder.move_to(Vec2::new(x - width / 2., y + height / 2.));
                                path_builder.arc(
                                    Vec2::new(x + radius - width / 2., y + radius + height / 2.),
                                    Vec2::new(radius, radius),
                                    sweep_angle,
                                    x_rotation,
                                );
                            }
                            PathCommand::Begin => {
                                dbg!("path already created");
                            }
                            PathCommand::Close => {
                                path_builder.close();
                            }
                            PathCommand::MoveTo((x, y)) => {
                                path_builder.move_to(Vec2::new(x, y));
                            }
                            PathCommand::CubicBezierTo(CubicBezierTo {
                                x1,
                                y1,
                                x2,
                                y2,
                                x3,
                                y3,
                            }) => {
                                path_builder.cubic_bezier_to(
                                    Vec2::new(x1, y1),
                                    Vec2::new(x2, y2),
                                    Vec2::new(x3, y3),
                                );
                            }
                        }
                    }
                    let path = path_builder.build();
                    commands.spawn((
                        ShapeBundle {
                            spatial: SpatialBundle {
                                transform: Transform::from_xyz(0., 0., 0.001),
                                ..default()
                            },
                            path,
                            ..default()
                        },
                        current_fill.unwrap_or(Fill::color(Color::RED)),
                        GuestEntity,
                    ));
                    current_fill = None;
                } else {
                    eprintln!("path should start from begin");
                }
            }
            HostEvent::MoveTo((x, y)) => {
                current_path.push(PathCommand::MoveTo((x, y)));
            }
            HostEvent::CubicBezierTo(cbt) => {
                current_path.push(PathCommand::CubicBezierTo(cbt));
            }
            HostEvent::Label(Label {
                text,
                x,
                y,
                size,
                color,
            }) => {
                commands.spawn((
                    Text2dBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                text,
                                TextStyle {
                                    font_size: size,
                                    color: string_to_bevy_color(color),
                                    ..default()
                                },
                            )],
                            ..default()
                        },
                        transform: Transform::from_xyz(x, y, 0.01),
                        ..default()
                    },
                    GuestEntity,
                ));
            }
            HostEvent::Link(Link {
                url,
                text,
                x,
                y,
                size,
            }) => {
                if let Some(pos) = camera.world_to_viewport(camera_transform, Vec3::new(x, y, 0.01))
                {
                    let button = commands
                        .spawn((
                            ButtonBundle {
                                focus_policy: bevy::ui::FocusPolicy::Pass,
                                background_color: Color::NONE.into(),
                                style: Style {
                                    position_type: bevy::ui::PositionType::Absolute,
                                    left: bevy::prelude::Val::Px(pos.x),
                                    top: bevy::prelude::Val::Px(pos.y),
                                    ..default()
                                },
                                ..default()
                            },
                            GuestUrl(url),
                            GuestEntity,
                        ))
                        .id();
                    let text = commands
                        .spawn((TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    text,
                                    TextStyle {
                                        font_size: size,
                                        color: Color::BLUE,
                                        ..default()
                                    },
                                )],
                                ..default()
                            },
                            ..default()
                        },))
                        .id();
                    commands.entity(button).add_child(text);
                }
            }
        }
    }
}

fn handle_refresh(
    text_input_q: Query<&CosmicEditor, With<AddressBar>>,
    mut refresh_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RefreshButton>),
    >,
    runtime: ResMut<TokioTasksRuntime>,
    canvas_q: Query<(&GlobalTransform, &bevy::ui::Node), With<Portal>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if windows.iter().len() == 0 {
        return;
    }
    let primary_window = windows.single();
    for (interaction, mut background_color) in refresh_q.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *background_color = Color::GRAY.with_a(0.3).into();
                let text = text_input_q.single().get_text();
                let (canvas_global_transform, canvas_node) = canvas_q.single();
                let (camera, camera_transform) = camera_q.single();
                let canvas_position = get_position(
                    canvas_global_transform,
                    &primary_window,
                    camera,
                    camera_transform,
                );
                if canvas_position.is_none() {
                    return;
                }
                let canvas = Canvas {
                    size: canvas_node.size(),
                    position: canvas_position.unwrap(),
                };
                runtime.spawn_background_task(move |ctx| async move {
                    match get_wasm(ctx, text.clone(), canvas).await {
                        Ok(_) => {}
                        Err(e) => eprintln!("failed to get wasm for '{text}': {e}"),
                    }
                });
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.with_a(0.3).into();
            }
            Interaction::None => *background_color = Color::NONE.into(),
        }
    }
}

fn handle_link(
    mut text_input_q: Query<(&CosmicEditor, &mut CosmicText), With<AddressBar>>,
    links_q: Query<(&Interaction, &GuestUrl), (Changed<Interaction>, With<GuestUrl>)>,
    runtime: ResMut<TokioTasksRuntime>,
    canvas_q: Query<(&GlobalTransform, &bevy::ui::Node), With<Portal>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if windows.iter().len() == 0 {
        return;
    }
    let mut primary_window = windows.single_mut();
    for (interaction, url) in links_q.iter() {
        match interaction {
            Interaction::Pressed => {
                primary_window.cursor.icon = CursorIcon::Hand;
                let mut text = url.0.clone();
                let (editor, mut text_setter) = text_input_q.single_mut();
                if let Ok(previous_url_value) = Url::parse(&make_url_valid(editor.get_text())) {
                    if let Ok(new_url) = previous_url_value.join(text.as_str()) {
                        text = new_url.to_string().replace("https://", "");
                    }
                }
                *text_setter = CosmicText::OneStyle(text.clone());
                let (canvas_global_transform, canvas_node) = canvas_q.single();
                let (camera, camera_transform) = camera_q.single();
                let canvas_position = get_position(
                    canvas_global_transform,
                    &primary_window,
                    camera,
                    camera_transform,
                );
                if canvas_position.is_none() {
                    return;
                }
                let canvas = Canvas {
                    size: canvas_node.size(),
                    position: canvas_position.unwrap(),
                };
                runtime.spawn_background_task(move |ctx| async move {
                    match get_wasm(ctx, text.clone(), canvas).await {
                        Ok(_) => {}
                        Err(e) => eprintln!("failed to get wasm for '{text}': {e}"),
                    }
                });
            }
            Interaction::Hovered => {
                primary_window.cursor.icon = CursorIcon::Hand;
            }
            Interaction::None => {
                primary_window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}

fn handle_get_wasm(
    editor_q: Query<&CosmicEditor, With<AddressBar>>,
    keys: Res<Input<KeyCode>>,
    runtime: ResMut<TokioTasksRuntime>,
    canvas_q: Query<(&GlobalTransform, &bevy::ui::Node), With<Portal>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if windows.iter().len() == 0 {
        return;
    }
    let primary_window = windows.single();
    #[cfg(target_os = "macos")]
    let command = keys.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);
    #[cfg(not(target_os = "macos"))]
    let command = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    if !keys.just_pressed(KeyCode::Return) && !(command && keys.just_pressed(KeyCode::R)) {
        return;
    }
    for editor in editor_q.iter() {
        let text = editor.get_text();
        let (canvas_global_transform, canvas_node) = canvas_q.single();
        let (camera, camera_transform) = camera_q.single();
        let canvas_position = get_position(
            canvas_global_transform,
            &primary_window,
            camera,
            camera_transform,
        );
        if canvas_position.is_none() {
            return;
        }
        let canvas = Canvas {
            size: canvas_node.size(),
            position: canvas_position.unwrap(),
        };
        runtime.spawn_background_task(move |ctx| async move {
            match get_wasm(ctx, text.clone(), canvas).await {
                Ok(_) => {}
                Err(e) => eprintln!("failed to get wasm for '{text}': {e}"),
            }
        });
    }
}

fn get_position(
    global_transform: &GlobalTransform,
    primary_window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    let world_position = global_transform.affine().translation;
    let point = Vec2::new(world_position.x, primary_window.height() - world_position.y);
    camera.viewport_to_world_2d(camera_transform, point)
}

fn run_wasm_update(
    wasm_instance: Option<ResMut<WasmBindings>>,
    wasm_store: Option<ResMut<WasmStore>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    canvas_q: Query<(&GlobalTransform, &bevy::ui::Node), With<Portal>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if windows.iter().len() == 0 {
        return;
    }
    let primary_window = windows.single();
    if let Some(wasm_resource) = wasm_instance {
        let mut store = wasm_store.unwrap();
        {
            let (canvas_global_transform, canvas_node) = canvas_q.single();
            let (camera, camera_transform) = camera_q.single();
            let canvas_position = get_position(
                canvas_global_transform,
                primary_window,
                camera,
                camera_transform,
            );
            let data = store.store.data_mut();

            data.delta_seconds = time.delta_seconds();

            data.inputs.keys_just_pressed.clear();
            data.inputs
                .keys_just_pressed
                .extend(keys.get_just_pressed());

            data.inputs.keys_pressed.clear();
            data.inputs.keys_pressed.extend(keys.get_pressed());

            data.inputs.keys_just_released.clear();
            data.inputs
                .keys_just_released
                .extend(keys.get_just_released());

            data.inputs.mouse_buttons_just_pressed.clear();
            data.inputs
                .mouse_buttons_just_pressed
                .extend(mouse_buttons.get_just_pressed());

            data.inputs.mouse_buttons_pressed.clear();
            data.inputs
                .mouse_buttons_pressed
                .extend(mouse_buttons.get_pressed());

            data.inputs.mouse_buttons_just_released.clear();
            data.inputs
                .mouse_buttons_just_released
                .extend(mouse_buttons.get_just_released());

            if let Some(pos) = canvas_position {
                data.canvas = Canvas {
                    size: canvas_node.size(),
                    position: pos,
                }
            }
            data.inputs.cursor_position = None;
            data.inputs.cursor_position = q_windows.get_single().ok().and_then(|w| {
                if let Some(p) = w.cursor_position() {
                    return Some(Vec2::new(
                        p.x - data.canvas.position.x,
                        p.y - data.canvas.position.y,
                    ));
                }
                None
            });
        }

        let _ = wasm_resource.bindings.call_update(&mut store.store);
    }
}

fn run_wasm_setup(
    wasm_instance: Option<ResMut<WasmBindings>>,
    wasm_store: Option<ResMut<WasmStore>>,
    args: Res<Args>,
) {
    if let Some(mut wasm_resource) = wasm_instance {
        if wasm_resource.first_run {
            wasm_resource.first_run = false;
            let mut store = wasm_store.unwrap();
            store.store.data_mut().allow_read = args.allow_read.clone();
            let _ = wasm_resource.bindings.call_setup(&mut store.store);
        }
    }
}

async fn get_wasm(
    mut ctx: bevy_tokio_tasks::TaskContext,
    url: String,
    canvas: Canvas,
) -> Result<(), Box<dyn std::error::Error>> {
    let valid_url = make_url_valid(url);
    let initial_buffer_size = 65536;
    let mut buffer = Vec::with_capacity(initial_buffer_size);

    #[cfg(feature = "webtransport")]
    {
        use url::Url;
        use wtransport::ClientConfig;
        use wtransport::Endpoint;

        let uri = Url::parse(valid_url.as_str()).expect("expected valid URL");
        let host = uri.host_str().expect("expected valid host");
        let path = uri.path();
        let config = ClientConfig::builder()
            .with_bind_default()
            .with_no_cert_validation() // TODO: don't do it on prod, use with_native_cers instead
            .enable_key_log()
            .build();
        if let Ok(connection) = Endpoint::client(config)
            .unwrap()
            .connect(format!("https://{}:4433{}", host, path))
            .await
        {
            let mut stream = connection.open_bi().await.unwrap().await?;
            stream.0.write_all(b"WASM").await?;

            loop {
                let mut chunk = vec![0; 65536];
                match stream.1.read(&mut chunk).await? {
                    Some(bytes_read) => {
                        buffer.extend_from_slice(&chunk[..bytes_read]);
                    }
                    None => break, // End of stream
                }
            }
        }
    }

    #[cfg(not(feature = "webtransport"))]
    {
        let response = reqwest::Client::builder()
            .build()?
            .get(&valid_url)
            .header("Accept-Encoding", "br")
            .send()
            .await?;
        let bytes = response.bytes().await?;
        buffer.extend_from_slice(&bytes);
    }

    // Decompress the received buffer using rust-brotli
    let mut decompressed_reader = Decompressor::new(buffer.as_slice(), 4096);
    let mut decoded_input = Vec::new();
    decompressed_reader.read_to_end(&mut decoded_input)?;

    // Set up Wasmtime components
    let mut config = Config::new();
    config.wasm_component_model(true).async_support(false);
    let engine = Engine::new(&config)?;
    let component = Component::new(&engine, decoded_input)?;

    // Set up Wasmtime linker
    let mut linker = Linker::new(&engine);
    sync::add_to_linker(&mut linker)?;
    let table = Table::new();
    let memory_size = 50 << 20; // 50 MB
    let wasi = WasiCtxBuilder::new().build();
    MyWorld::add_to_linker(&mut linker, |state: &mut MyCtx| state)?;
    // Set up Wasmtime store
    let mut store = Store::new(
        &engine,
        MyCtx {
            table,
            wasi,
            queue: Vec::new(),
            delta_seconds: 0.0,
            limits: StoreLimitsBuilder::new().memory_size(memory_size).build(),
            inputs: Default::default(),
            canvas,
            allow_read: None,
        },
    );
    store.limiter(|state| &mut state.limits);
    let (bindings, _) = MyWorld::instantiate(&mut store, &component, &linker)?;

    ctx.run_on_main_thread(move |ctx| {
        if let Some(mut wasm_resource) = ctx.world.get_resource_mut::<WasmBindings>() {
            wasm_resource.bindings = bindings;
            wasm_resource.first_run = true;
        } else {
            ctx.world.insert_resource(WasmBindings {
                bindings,
                first_run: true,
            })
        }
        if let Some(mut wasm_resource) = ctx.world.get_resource_mut::<WasmStore>() {
            wasm_resource.store = store;
        } else {
            ctx.world.insert_resource(WasmStore { store })
        }
    })
    .await;

    Ok(())
}

fn canonicalize_path(path: &Path) -> Result<PathBuf, String> {
    Path::new(path)
        .canonicalize()
        .map_err(|e| format!("Error canonicalizing path {}: {}", path.display(), e))
}

fn is_path_within_allowed_directory(allowed_path: &Path, target_path: &Path) -> bool {
    target_path.starts_with(allowed_path)
}

fn make_url_valid(url: String) -> String {
    if url.contains("http") {
        url
    } else {
        format!("https://{}", url)
    }
}
