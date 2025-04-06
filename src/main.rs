use hound::{SampleFormat, WavSpec, WavWriter};
//use std::fs::File;

fn main() {
    let mut synthesizer = Synth::new();
    let num_samples = 88_200;
    let sample_rate = 44_100.0;
    // TODO make individual tests for diff waveforms instead of main
    synthesizer.add_waveform(Box::new(SineWave));
    //synthesizer.add_waveform(Box::new(SquareWave));
    //synthesizer.add_waveform(Box::new(SawtoothWave));
    let samples = synthesizer.generate_samples(num_samples, sample_rate);

    // Print the first 100 samples for demonstration
    for (i, sample) in samples.iter().take(44100).enumerate() {
        if i % 4000 == 0 {
            println!("Sample {}: {}", i, sample);
        }
    }

    /*
    // export
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create("test.wav", spec).unwrap();
    for s in samples {
        // multiply by 32767 to convert to 16 bit PCM
        let converted: i16 = (s * 32767 as f32).floor() as i16;
        writer
            .write_sample(converted)
            .expect("sample to write into writer")
    }
    writer.finalize().expect("writer to finalize");
    */
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
            // if less than 0
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
    // waveforms should include one or more oscillator (Waveform)
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
                *sam += (waveform.sample(t) / self.waveforms.len() as f32);
            }
        }
        internal_samples
    }

    /*
    fn write_file(readied_samples: Vec<f32>) {
        let mut output = File::create("rusty.wav");
        output.write
    }
    */
}
