fn main() {
    let mut synthesizer = Synth::new();
    let num_samples = 88_000;
    let sample_rate = 44_000.0;
    let samples = synthesizer.generate_samples(num_samples, sample_rate);

    synthesizer.add_waveform(Box::new(SineWave));
    synthesizer.add_waveform(Box::new(SquareWave));
    synthesizer.add_waveform(Box::new(SawtoothWave));

    // Print the first 10 samples for demonstration
    for (i, sample) in samples.iter().take(100).enumerate() {
        println!("Sample {}: {}", i, sample);
    }
}

trait Waveform {
    fn sample(&self, t: f32) -> f32;
    // add frequency here?
}

struct SineWave;
struct SquareWave;
// PulseWave;
// case of square, or square a case of pulse?
struct SawtoothWave;
// additive sawtooth
// struct TriangeWave;

impl Waveform for SineWave {
    fn sample(&self, t: f32) -> f32 {
        // 2pi radians = circumference
        (t * 2.0 * std::f32::consts::PI).sin()
    }
}

impl Waveform for SquareWave {
    fn sample(&self, t: f32) -> f32 {
        if SineWave.sample(t) > 0.0 {
            1.0
        } else if SineWave.sample(t) == 0.0 {
            0.0
        } else {
            -1.0
        }
    }
}

impl Waveform for SawtoothWave {
    fn sample(&self, t: f32) -> f32 {
        // very nearly TTOMO
        2.0 * (t - t.floor()) - 1.0
    }
}

struct Synth {
    waveforms: Vec<Box<dyn Waveform>>,
}

impl Synth {
    fn new() -> Self {
        Synth {
            waveforms: Vec::new(),
        }
    }

    fn add_waveform(&mut self, waveform: Box<dyn Waveform>) {
        self.waveforms.push(waveform)
    }

    fn generate_samples(&self, num_samples: usize, samplerate: f32) -> Vec<f32> {
        let mut internal_samples = vec![0.0; num_samples];
        //zip for non usize enumerate()
        for (i, sam) in internal_samples.iter_mut().enumerate() {
            let t = i as f32 / samplerate;
            for waveform in &self.waveforms {
                // measure sample for waveform, then divide result by number of waveforms to mix and avoid clipping
                *sam += waveform.sample(t) / self.waveforms.len() as f32;
            }
            // *sam /= self.waveforms.len() as f32; // average (mix) samples
        }
        internal_samples
    }
}
