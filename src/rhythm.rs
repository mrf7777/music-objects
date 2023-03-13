#[derive(Clone, PartialEq, Eq, Debug)]
struct TimeSignature {
    numerator: u8,
    denominator: u8,
}

impl TimeSignature {
    pub fn new(numerator: u8, denominator: u8) -> Option<TimeSignature> {
        if numerator == 0 || denominator == 0 {
            return None;
        }

        Some(Self {
            numerator,
            denominator,
        })
    }
}
