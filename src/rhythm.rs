pub trait ToRatio {
    fn to_ratio(&self) -> f64;
}

#[derive(Clone, PartialEq, Eq, Debug)]

pub struct TimeSignature {
    numerator: u16,
    denominator: u16,
}

impl TimeSignature {
    pub fn new(numerator: u16, denominator: u16) -> Option<Self> {
        if numerator == 0 || denominator == 0 {
            return None;
        }

        Some(Self {
            numerator,
            denominator,
        })
    }

    pub fn numerator(&self) -> u16 {
        self.numerator
    }

    pub fn denominator(&self) -> u16 {
        self.denominator
    }
}

impl ToRatio for TimeSignature {
    fn to_ratio(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tempo {
    tempo_bpm: f64,
}

impl Tempo {
    pub fn new(tempo_bpm: f64) -> Option<Self> {
        if tempo_bpm < 0.0 {
            return None;
        }

        Some(Self { tempo_bpm })
    }

    pub fn bpm(&self) -> f64 {
        self.tempo_bpm
    }

    pub fn bps(&self) -> f64 {
        self.tempo_bpm / 60.0
    }

    pub fn one_beat_seconds(&self) -> f64 {
        1.0 / self.bps()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BeatAssignment {
    duration: Duration,
}

impl BeatAssignment {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }

    pub fn beat_duration(&self) -> &Duration {
        &self.duration
    }

    pub fn beats_in_duration(&self, duration: &Duration) -> f64 {
        let beat_assignment_ratio = self.beat_duration().to_ratio();
        let duration_ratio = duration.to_ratio();
        duration_ratio / beat_assignment_ratio
    }
}

impl ToRatio for BeatAssignment {
    fn to_ratio(&self) -> f64 {
        self.duration.to_ratio()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Duration {
    signature: TimeSignature,
}

impl Duration {
    pub fn new(numerator: u16, denominator: u16) -> Option<Self> {
        Some(Self {
            signature: TimeSignature::new(numerator, denominator)?,
        })
    }

    pub fn numerator(&self) -> u16 {
        self.signature.numerator()
    }

    pub fn denominator(&self) -> u16 {
        self.signature.denominator()
    }
}

impl ToRatio for Duration {
    fn to_ratio(&self) -> f64 {
        self.signature.to_ratio()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Rhythm {
    tempo: Tempo,
    beat_assignment: BeatAssignment,
}

impl Rhythm {
    pub fn new(tempo: Tempo, beat_assignment: BeatAssignment) -> Self {
        Self {
            tempo,
            beat_assignment,
        }
    }

    pub fn tempo(&self) -> &Tempo {
        &self.tempo
    }

    pub fn beat_assignment(&self) -> &BeatAssignment {
        &self.beat_assignment
    }

    pub fn seconds_from_duration(&self, duration: &Duration) -> f64 {
        let beats_in_duration = self.beat_assignment().beats_in_duration(duration); // TODO:
        beats_in_duration * self.tempo.one_beat_seconds()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Metre {
    rhythm: Rhythm,
    time_signature: TimeSignature,
}

impl Metre {
    pub fn new(beat: Rhythm, time_signature: TimeSignature) -> Self {
        Self {
            rhythm: beat,
            time_signature,
        }
    }

    pub fn beat(&self) -> &Rhythm {
        &self.rhythm
    }

    pub fn time_signature(&self) -> &TimeSignature {
        &self.time_signature
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tempo_bpm_bps() {
        let tempo1 = Tempo::new(120.0).unwrap();
        assert!((tempo1.bpm() - 120.00).abs() < 0.05);
        assert!((tempo1.bps() - 2.00).abs() < 0.05);
    }
}
