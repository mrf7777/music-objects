#![deny(clippy::all, clippy::pedantic)]

#[derive(Clone, Debug)]
pub struct Ratio {
    numerator: u32,
    denominator: u32,
}

impl Ratio {
    #[must_use]
    pub fn new(numerator: u32, denominator: u32) -> Option<Self> {
        if numerator == 0 || denominator == 0 {
            return None;
        }

        Some(Self {
            numerator,
            denominator,
        })
    }

    #[must_use]
    pub fn numerator(&self) -> u32 {
        self.numerator
    }

    #[must_use]
    pub fn denominator(&self) -> u32 {
        self.denominator
    }

    #[must_use]
    pub fn to_f64(&self) -> f64 {
        f64::from(self.numerator()) / f64::from(self.denominator())
    }
}

impl PartialEq for Ratio {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Ratio {}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // numerator and denominator are non-zero
        let other_ratio = other.to_f64();
        let self_ratio = self.to_f64();

        if self_ratio < other_ratio {
            std::cmp::Ordering::Less
        } else if self_ratio > other_ratio {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimeSignature {
    ratio: Ratio,
}

impl TimeSignature {
    #[must_use]
    pub fn new(ratio: Ratio) -> Self {
        Self { ratio }
    }

    #[must_use]
    pub fn ratio(&self) -> &Ratio {
        &self.ratio
    }
}

#[derive(Clone, Copy, PartialEq, Debug, PartialOrd)]
pub struct Tempo {
    tempo_bpm: f64,
}

impl Tempo {
    #[must_use]
    pub fn new(tempo_bpm: f64) -> Option<Self> {
        if tempo_bpm < 0.0 {
            return None;
        }

        Some(Self { tempo_bpm })
    }

    #[must_use]
    pub fn bpm(&self) -> f64 {
        self.tempo_bpm
    }

    #[must_use]
    pub fn bps(&self) -> f64 {
        self.tempo_bpm / 60.0
    }

    #[must_use]
    pub fn one_beat_seconds(&self) -> f64 {
        1.0 / self.bps()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct BeatAssignment {
    duration: Duration,
}

impl BeatAssignment {
    #[must_use]
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }

    #[must_use]
    pub fn beat_duration(&self) -> &Duration {
        &self.duration
    }

    #[must_use]
    pub fn beats_in_duration(&self, duration: &Duration) -> f64 {
        let beat_assignment_ratio = self.beat_duration().ratio();
        let duration_ratio = duration.ratio();
        duration_ratio.to_f64() / beat_assignment_ratio.to_f64()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Duration {
    ratio: Ratio,
}

impl Duration {
    #[must_use]
    pub fn new(numerator: u32, denominator: u32) -> Option<Self> {
        Some(Self {
            ratio: Ratio::new(numerator, denominator)?,
        })
    }

    #[must_use]
    pub fn new_from_ratio(time_signature: Ratio) -> Self {
        Self {
            ratio: time_signature,
        }
    }

    #[must_use]
    pub fn numerator(&self) -> u32 {
        self.ratio.numerator()
    }

    #[must_use]
    pub fn denominator(&self) -> u32 {
        self.ratio.denominator()
    }

    #[must_use]
    pub fn ratio(&self) -> &Ratio {
        &self.ratio
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Rhythm {
    tempo: Tempo,
    beat_assignment: BeatAssignment,
}

impl Rhythm {
    #[must_use]
    pub fn new(tempo: Tempo, beat_assignment: BeatAssignment) -> Self {
        Self {
            tempo,
            beat_assignment,
        }
    }

    #[must_use]
    pub fn tempo(&self) -> &Tempo {
        &self.tempo
    }

    #[must_use]
    pub fn beat_assignment(&self) -> &BeatAssignment {
        &self.beat_assignment
    }

    #[must_use]
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
    #[must_use]
    pub fn new(rhythm: Rhythm, time_signature: TimeSignature) -> Self {
        Self {
            rhythm,
            time_signature,
        }
    }

    #[must_use]
    pub fn beat(&self) -> &Rhythm {
        &self.rhythm
    }

    #[must_use]
    pub fn time_signature(&self) -> &TimeSignature {
        &self.time_signature
    }

    #[must_use]
    pub fn bars_from_duration(&self, duration: &Duration) -> f64 {
        let bar_duration = Duration::new_from_ratio(self.time_signature().ratio().clone());
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
        let rhythm1 = Rhythm::new(tempo1, beat_assignment1.clone());
        let rhythm2 = Rhythm::new(tempo1, beat_assignment2.clone());

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

    #[test]
    fn rhythm_order() {
        let duration1 = Duration::new(1, 4);
        let duration2 = Duration::new(1, 3);
        let duration3 = Duration::new(100, 3);
        let duration4 = Duration::new(2, 8);
        let duration5 = Duration::new(2, 6);

        assert!(duration1 < duration2);
        assert!(duration2 > duration1);
        assert!(duration1 == duration1);
        assert!(duration1 != duration2);

        assert!(duration1 == duration4);
        assert!(duration2 == duration5);
        assert!(duration1 < duration5);
        assert!(duration5 > duration1);

        assert!(duration3 > duration1);
        assert!(duration3 > duration2);
        assert!(duration3 <= duration3);
        assert!(duration3 > duration4);
        assert!(duration3 > duration5);
    }
}
