use std::ops::RangeInclusive;

use rand::{rngs::ThreadRng, Rng};

use crate::mei::PitchName;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PitchNameWithOctave {
    pub pitch: PitchName,
    pub octave: u8,
}

impl PitchNameWithOctave {
    fn diatonic_value(self) -> u8 {
        self.pitch.diatonic_value() + self.octave * 7
    }
    fn from_diatonic_value(v: u8) -> Self {
        PitchNameWithOctave {
            pitch: PitchName::from_diatonic_value(v % 7),
            octave: v / 7,
        }
    }
}

pub fn random_pitch_with_octave(
    rng: &mut ThreadRng,
    range: &RangeInclusive<PitchNameWithOctave>,
) -> PitchNameWithOctave {
    let min = range.start().diatonic_value();
    let max = range.end().diatonic_value();
    let random_val = rng.random_range((min as u32)..(max as u32)) as u8;
    PitchNameWithOctave::from_diatonic_value(random_val)
}

pub const C2: PitchNameWithOctave = PitchNameWithOctave {
    pitch: PitchName::C,
    octave: 2,
};
pub const C4: PitchNameWithOctave = PitchNameWithOctave {
    pitch: PitchName::C,
    octave: 4,
};
pub const BASS_CLEFF_RANGE: RangeInclusive<PitchNameWithOctave> = C2..=C4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn specific_examples() {
        // supondo PitchName::C tem valor 0
        let c4 = PitchNameWithOctave {
            pitch: PitchName::from_diatonic_value(0),
            octave: 4,
        };
        assert_eq!(c4.diatonic_value(), 28);
        assert_eq!(PitchNameWithOctave::from_diatonic_value(28), c4);

        // supondo PitchName::F tem valor 3
        let f2 = PitchNameWithOctave {
            pitch: PitchName::from_diatonic_value(3),
            octave: 2,
        };
        assert_eq!(f2.diatonic_value(), 17);
        assert_eq!(PitchNameWithOctave::from_diatonic_value(17), f2);
    }
}
