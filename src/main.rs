mod sounds;
mod kb_listener;
mod sound_container;

use std::error::Error;
use crate::kb_listener::EventType;
use std::collections::HashMap;
use rand::thread_rng;
use rand::Rng;
use soloud::*;
use crate::sound_container::SoundContainer;


fn main() -> Result<(), Box<dyn Error>> {
    println!("Mechanical Keyboard Simulator");
    ctrlc::set_handler(|| {
        println!("exiting");
    })
    .expect("Error setting Ctrl-C handler");

    let mut rng = thread_rng();

    let sl = Soloud::default()?;

    let mut is_key_held = HashMap::new();

    let sounds = SoundContainer::new();

    let mut num_of_hits = 0_u64;

    let receiver = kb_listener::handle_input_events();

    while let Ok(value) = receiver.recv() {
        println!("value: {:?}", value);
        match value.event_type {
            EventType::UP | EventType::SYSUP => {
                let old_val = is_key_held.insert(value.value, false);
                if old_val.map_or(false, |v| v) {
                    sl.play(sounds.get_sound(value.value).up);
                }
            }
            EventType::DOWN | EventType::SYSDOWN => {
                let old_val = is_key_held.insert(value.value, true);
                if !old_val.map_or(false, |v| v) {
                    num_of_hits = num_of_hits + 1;

                    if num_of_hits > 40 && rng.gen_range(0..40) == 0 {
                        sl.play(&sounds.honk);
                    } else {
                        sl.play(sounds.get_sound(value.value).down);
                    }
                }
            }
        }
    }


    Ok(())
}
