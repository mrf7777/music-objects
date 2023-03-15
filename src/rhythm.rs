#![deny(clippy::all, clippy::pedantic)]

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

    pub fn new_from_time_signature(time_signature: TimeSignature) -> Self {
        Self {
            signature: time_signature,
        }
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
        let beats_in_duration = self.beat_assignment().beats_in_duration(duration);
        beats_in_duration * self.tempo.one_beat_seconds()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Metre {
    rhythm: Rhythm,
    time_signature: TimeSignature,
}

impl Metre {
    pub fn new(rhythm: Rhythm, time_signature: TimeSignature) -> Self {
        Self {
            rhythm,
            time_signature,
        }
    }

    pub fn beat(&self) -> &Rhythm {
        &self.rhythm
    }

    pub fn time_signature(&self) -> &TimeSignature {
        &self.time_signature
    }

    pub fn bars_from_duration(&self, duration: &Duration) -> f64 {
        let bar_duration = Duration::new_from_time_signature(self.time_signature().clone());
        let seconds_per_bar = self.rhythm.seconds_from_duration(&bar_duration);
        self.rhythm.seconds_from_duration(duration) / seconds_per_bar
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

    #[test]
    fn rhythm_and_duration_seconds() {
        let tempo1 = Tempo::new(120.0).unwrap();
        let beat_assignment1 = BeatAssignment::new(Duration::new(1, 4).unwrap());
        let beat_assignment2 = BeatAssignment::new(Duration::new(2, 4).unwrap());
        let rhythm1 = Rhythm::new(tempo1.clone(), beat_assignment1.clone());
        let rhythm2 = Rhythm::new(tempo1.clone(), beat_assignment2.clone());

        assert!(
            (rhythm1.seconds_from_duration(beat_assignment1.beat_duration()) - 0.5).abs() < 0.05
        );
        assert!(
            (rhythm2.seconds_from_duration(beat_assignment2.beat_duration()) - 0.5).abs() < 0.05
        );

        let duration1 = Duration::new(1, 4).unwrap();
        let duration2 = Duration::new(2, 8).unwrap();
        let duration3 = Duration::new(2, 4).unwrap();
        let duration4 = Duration::new(9, 4).unwrap();
        let duration5 = Duration::new(1, 6).unwrap();

        assert!((rhythm1.seconds_from_duration(&duration1) - 0.5).abs() < 0.05);
        assert!((rhythm1.seconds_from_duration(&duration2) - 0.5).abs() < 0.05);
        assert!((rhythm1.seconds_from_duration(&duration3) - 1.0).abs() < 0.05);
        assert!((rhythm1.seconds_from_duration(&duration4) - 4.5).abs() < 0.05);
        assert!((rhythm1.seconds_from_duration(&duration5) - 0.33333).abs() < 0.05);
    }
}
