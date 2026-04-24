use crate::action::actions::{ButtonEvent, EncoderEvent, HardwareTrigger};
use crate::protocol::PicoToHost;
use std::time::Instant;

// Diese Werte kannst du nach Gefühl anpassen
const LONG_PRESS_MS: u128 = 500;
const DOUBLE_PRESS_TIMEOUT_MS: u128 = 250;

pub struct InputTracker {
    button_press_times: [Option<Instant>; 16], // Annahme: max 16 Buttons
    button_last_release: [Option<Instant>; 16],
    encoder_pushed: [bool; 4],                 // Annahme: max 4 Encoder
}

impl InputTracker {
    pub fn new() -> Self {
        Self {
            button_press_times: [None; 16],
            button_last_release: [None; 16],
            encoder_pushed: [false; 4],
        }
    }

    // Wandelt das rohe Pico-Event in unser logisches System um
    pub fn process_event(&mut self, event: PicoToHost) -> Option<HardwareTrigger> {
        match event {
            PicoToHost::ButtonChanged { id, pressed } => {
                let id_usize = id as usize;

                // --- Ist es der Encoder-Button? ---
                // (Du musst wissen, welche IDs deine Encoder-Buttons haben.
                // Angenommen Encoder 0 hat Button ID 10)
                if id == 10 {
                    self.encoder_pushed[0] = pressed;
                    if !pressed {
                        // Der Push an sich wurde losgelassen
                        return Some(HardwareTrigger::Encoder {
                            id: 0,
                            event: EncoderEvent::PushPress
                        });
                    }
                    return None;
                }

                // --- Normale Buttons ---
                if pressed {
                    self.button_press_times[id_usize] = Some(Instant::now());

                    // Double Press Check
                    if let Some(last_release) = self.button_last_release[id_usize] {
                        if last_release.elapsed().as_millis() < DOUBLE_PRESS_TIMEOUT_MS {
                            // Es war ein Double Press!
                            self.button_press_times[id_usize] = None; // Reset
                            self.button_last_release[id_usize] = None;
                            return Some(HardwareTrigger::Button { id, event: ButtonEvent::DoublePress });
                        }
                    }
                    None
                } else {
                    // Button losgelassen -> Berechne Dauer
                    if let Some(press_time) = self.button_press_times[id_usize] {
                        let duration = press_time.elapsed().as_millis();
                        self.button_press_times[id_usize] = None;
                        self.button_last_release[id_usize] = Some(Instant::now()); // Für Double-Click timer

                        if duration >= LONG_PRESS_MS {
                            return Some(HardwareTrigger::Button { id, event: ButtonEvent::LongPress });
                        } else {
                            // Achtung: Um DoubleClick sauber zu erkennen, müsstest du eigentlich
                            // hier kurz warten, ob noch ein Klick kommt. Fürs Erste triggern
                            // wir ShortPress aber sofort.
                            return Some(HardwareTrigger::Button { id, event: ButtonEvent::ShortPress });
                        }
                    }
                    None
                }
            }
            PicoToHost::EncoderTurned { id, delta } => {
                let is_pushed = self.encoder_pushed[id as usize];

                let event = match (delta > 0, is_pushed) {
                    (true, false) => EncoderEvent::TurnRight,
                    (false, false) => EncoderEvent::TurnLeft,
                    (true, true) => EncoderEvent::PushTurnRight,
                    (false, true) => EncoderEvent::PushTurnLeft,
                };

                Some(HardwareTrigger::Encoder { id, event })
            }
            _ => None,
        }
    }
}