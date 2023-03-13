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

    pub fn tempo_bpm(&self) -> f64 {
        self.tempo_bpm
    }

    pub fn tempo_bps(&self) -> f64 {
        self.tempo_bpm / 60.0
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
        assert!((tempo1.tempo_bpm() - 120.00).abs() < 0.05);
        assert!((tempo1.tempo_bps() - 2.00).abs() < 0.05);
    }
}
