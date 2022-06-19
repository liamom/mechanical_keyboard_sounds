use soloud::*;
use crate::sounds;

fn make_sound(sound: &[u8]) -> audio::Wav {
    let mut wav = audio::Wav::default();
    wav.load_mem(sound.to_vec()).unwrap();
    return wav
}

pub struct SoundPair<'a> {
    pub up: &'a Wav,
    pub down: &'a Wav,
}

impl <'a> SoundPair<'a> {
    fn new(up:&'a Wav, down:&'a Wav) -> Self {
        SoundPair {up, down}
    }
}

pub struct SoundContainer {
    pub letter_down: audio::Wav,
    pub letter_up: audio::Wav,
    pub space_down: audio::Wav,
    pub space_up: audio::Wav,
    pub enter_up: audio::Wav,
    pub enter_down: audio::Wav,
    pub backspace_up: audio::Wav,
    pub backspace_down: audio::Wav,
    pub honk: audio::Wav,
}

impl SoundContainer {
    pub fn new() -> Self {
        SoundContainer {
            letter_down: make_sound(sounds::LETTER_DOWN),
            letter_up: make_sound(sounds::LETTER_UP),
            space_down: make_sound(sounds::SPACE_DOWN),
            space_up: make_sound(sounds::SPACE_UP),
            enter_up: make_sound(sounds::ENTER_UP),
            enter_down: make_sound(sounds::ENTER_DOWN),
            backspace_up: make_sound(sounds::BACKSPACE_UP),
            backspace_down: make_sound(sounds::BACKSPACE_DOWN),
            honk: make_sound(sounds::HONK),
        }
    }

    pub fn get_sound(&self, code: u64) -> SoundPair {
        match code {
            32 => SoundPair::new(&self.space_up,&self.space_down),
            13 => SoundPair::new(&self.enter_up, &self.enter_down),
            8 => SoundPair::new(&self.backspace_up, &self.backspace_down),
            _ => SoundPair::new(&self.letter_up,&self.letter_down),
        }
    }
}