use crate::action::actions::Action;
use enigo::Direction::{Click, Press, Release};
use enigo::{Enigo, Key, Keyboard, Settings};
use log::{debug, warn};

#[derive(Debug)]
pub struct CustomMacroAction {
    pub keys_string: String,
}

impl Action for CustomMacroAction {
    fn execute(&self) {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        let parts: Vec<&str> = self.keys_string.split(" + ").collect();

        let mut modifiers = Vec::new();
        let mut main_key = None;

        // Tasten zuordnen
        for part in parts {
            match parse_key_string(part) {
                Some(key) => {
                    match key {
                        Key::Control | Key::Shift | Key::Alt | Key::Meta => modifiers.push(key),
                        _ => main_key = Some(key), // Alles andere ist die Haupttaste
                    }
                }
                None => warn!("Unbekannte Taste im Makro: {}", part),
            }
        }

        for mod_key in &modifiers {
            let _ = enigo.key(mod_key.clone(), Press);
        }

        if let Some(key) = main_key {
            let _ = enigo.key(key, Click);
        }

        for mod_key in modifiers.iter().rev() {
            let _ = enigo.key(mod_key.clone(), Release);
        }

        debug!("Makro ausgeführt: {}", self.keys_string);
    }
}

fn parse_key_string(s: &str) -> Option<Key> {
    match s {
        "Ctrl" => Some(Key::Control),
        "Shift" => Some(Key::Shift),
        "Alt" => Some(Key::Alt),
        "Win" => Some(Key::Meta),
        "SPACE" => Some(Key::Space),
        "ENTER" => Some(Key::Return),
        "ESCAPE" => Some(Key::Escape),
        "TAB" => Some(Key::Tab),
        "BACKSPACE" => Some(Key::Backspace),
        "ARROWUP" => Some(Key::UpArrow),
        "ARROWDOWN" => Some(Key::DownArrow),
        "ARROWLEFT" => Some(Key::LeftArrow),
        "ARROWRIGHT" => Some(Key::RightArrow),
        _ => {
            if s.len() == 1 {
                let c = s.chars().next().unwrap().to_ascii_lowercase();
                Some(Key::Unicode(c))
            } else {
                None
            }
        }
    }
}