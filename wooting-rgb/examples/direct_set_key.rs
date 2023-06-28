use std::{thread::sleep, time::Duration};

use wooting_rgb::{Key, RgbKeyboard};

pub const ALL_KEYS: &'static [Key] = &[
    Key::Escape,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::F11,
    Key::F12,
    Key::PrintScreen,
    Key::Pause,
    Key::ScrollLock,
    Key::A1,
    Key::A2,
    Key::A3,
    Key::Mode,
    Key::Tilde,
    Key::One,
    Key::Two,
    Key::Three,
    Key::Four,
    Key::Five,
    Key::Six,
    Key::Seven,
    Key::Eight,
    Key::Nine,
    Key::Zero,
    Key::Dash,
    Key::Equals,
    Key::Backspace,
    Key::Insert,
    Key::Home,
    Key::PageUp,
    Key::NumLock,
    Key::NumDivide,
    Key::NumMultiply,
    Key::NumSubtract,
    Key::Tab,
    Key::Q,
    Key::W,
    Key::E,
    Key::R,
    Key::T,
    Key::Y,
    Key::U,
    Key::I,
    Key::O,
    Key::P,
    Key::LeftBracket,
    Key::RightBracket,
    Key::Backslash,
    Key::Delete,
    Key::End,
    Key::PageDown,
    Key::NumSeven,
    Key::NumEight,
    Key::NumNine,
    Key::NumAddition,
    Key::CapsLock,
    Key::A,
    Key::S,
    Key::D,
    Key::F,
    Key::G,
    Key::H,
    Key::J,
    Key::K,
    Key::L,
    Key::SemiColon,
    Key::Apostrophe,
    Key::ISO1,
    Key::Return,
    Key::NumFour,
    Key::NumFive,
    Key::NumSix,
    Key::LeftShift,
    Key::ISO2,
    Key::Z,
    Key::X,
    Key::C,
    Key::V,
    Key::B,
    Key::N,
    Key::M,
    Key::Comma,
    Key::Period,
    Key::ForwardSlash,
    Key::RightShift,
    Key::UpArrow,
    Key::NumOne,
    Key::NumTwo,
    Key::NumThree,
    Key::NumReturn,
    Key::LeftControl,
    Key::LeftMod,
    Key::LeftAlt,
    Key::Space,
    Key::RightAlt,
    Key::RightMod,
    Key::Fn,
    Key::RightControl,
    Key::LeftArrow,
    Key::DownArrow,
    Key::RightArrow,
    Key::NumZero,
    Key::NumDelete,
];

fn main() {
    println!(
        "Keyboard connected? {}",
        wooting_rgb::is_wooting_keyboard_connected()
    );

    let mut keyboard = RgbKeyboard::default();

    for key in ALL_KEYS {
        println!("Setting {} to white!", key);
        let _ = keyboard.direct_set_key(*key, 255, 255, 255);
        sleep(Duration::from_millis(500));
    }

    println!("Finished!");
}