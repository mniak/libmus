#![allow(dead_code)]
pub mod libmusc;

#[repr(C)]
pub enum PitchStep {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl PitchStep {
    fn to_number(&self) -> u8 {
        match self {
            PitchStep::C => 1,
            PitchStep::D => 2,
            PitchStep::E => 3,
            PitchStep::F => 4,
            PitchStep::G => 5,
            PitchStep::A => 6,
            PitchStep::B => 7,
        }
    }
    fn to_string(&self) -> &'static str {
        match self {
            PitchStep::C => "C",
            PitchStep::D => "D",
            PitchStep::E => "E",
            PitchStep::F => "F",
            PitchStep::G => "G",
            PitchStep::A => "A",
            PitchStep::B => "B",
        }
    }
}

#[repr(C)]
pub enum Alteration {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

#[repr(C)]
pub struct PitchClass {
    step: PitchStep,
    alteration: Alteration,
}

#[repr(C)]
pub struct Pitch {
    class: PitchClass,
    octave: u8,
}

#[repr(C)]
pub enum PerfectableIntervalSize {
    Unison,
    Fourth,
    Fifth,
    Octave,
}
impl PerfectableIntervalSize {
    fn invert(&self) -> PerfectableIntervalSize {
        match self {
            Self::Unison => Self::Octave,
            Self::Fourth => Self::Fifth,
            Self::Fifth => Self::Fourth,
            Self::Octave => Self::Unison,
        }
    }
}

#[repr(C)]
pub enum MajorableIntervalSize {
    Second,
    Third,
    Sixth,
    Seventh,
}
impl MajorableIntervalSize {
    fn invert(&self) -> MajorableIntervalSize {
        match self {
            Self::Second => Self::Seventh,
            Self::Third => Self::Sixth,
            Self::Sixth => Self::Third,
            Self::Seventh => Self::Second,
        }
    }
}

#[repr(C)]
pub enum PerfectableIntervalQuality {
    Diminished,
    Perfect,
    Augmented,
}
impl PerfectableIntervalQuality {
    fn invert(&self) -> PerfectableIntervalQuality {
        match self {
            Self::Augmented => Self::Diminished,
            Self::Diminished => Self::Augmented,
            Self::Perfect => Self::Perfect,
        }
    }
}

#[repr(C)]
pub enum MajorableIntervalQuality {
    Major,
    Minor,
}
impl MajorableIntervalQuality {
    fn invert(&self) -> MajorableIntervalQuality {
        match self {
            Self::Major => Self::Minor,
            Self::Minor => Self::Major,
        }
    }
}

#[repr(C)]
pub enum Interval {
    Perfectable {
        size: PerfectableIntervalSize,
        quality: PerfectableIntervalQuality,
    },
    Majorable {
        size: MajorableIntervalSize,
        quality: MajorableIntervalQuality,
    },
}

impl Interval {
    fn invert(&self) -> Interval {
        match self {
            Self::Perfectable { size, quality } => Self::Perfectable {
                size: size.invert(),
                quality: quality.invert(),
            },
            Self::Majorable { size, quality } => Self::Majorable {
                size: size.invert(),
                quality: quality.invert(),
            },
        }
    }
}
