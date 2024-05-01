fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[repr(C)]
enum PitchStep {
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

mod libmusc {

    use crate::*;
    use std::ffi::{c_char, CString};

    #[no_mangle]
    pub extern "C" fn add(left: usize, right: usize) -> usize {
        left + right
    }

    #[no_mangle]
    pub extern "C" fn add9(left: usize, right: usize) -> usize {
        left + right
    }

    #[no_mangle]
    pub extern "C" fn PitchStep_ToNumber(step: PitchStep) -> u8 {
        return step.to_number();
    }

    #[no_mangle]
    pub extern "C" fn PitchStep_ToString(step: PitchStep) -> *const c_char{
        let s1 = step.to_string();
        let s2 = CString::new(s1).unwrap();
        return s2.into_raw();
    }
}
