use rodio::{source::Source, OutputStream, Sink};

pub struct AudioDriver {
    // found out i needed to keep this stream alive the hard way.
    // hours lost: 3
    stream: OutputStream,
    sink: Sink,
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioDriver {
    pub fn build() -> AudioDriver {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // had to look this one up!
        // this makes a beep
        let freq = 440.0;
        let sample_rate = 44100;
        let duration_secs = 1;
        let samples = Self::generate_square_wave(freq, duration_secs, sample_rate);

        AudioDriver {
            stream,
            sink,
            samples,
            sample_rate,
        }
    }

    pub fn play(&mut self) {
        let source = rodio::buffer::SamplesBuffer::new(1, self.sample_rate, self.samples.clone());
        self.sink.append(source.convert_samples::<f32>());
    }

    pub fn stop(&mut self) {
        self.sink.stop();
    }

    // square wave goes brrrrrr
    fn generate_square_wave(freq: f32, duration_secs: u64, sample_rate: u32) -> Vec<f32> {
        (0..sample_rate as u64 * duration_secs)
            .map(move |x| {
                let t = x as f32 / sample_rate as f32;
                if (t * freq * 2.0 * std::f32::consts::PI).sin() >= 0.0 {
                    0.8 // High part of the wave
                } else {
                    -0.8 // Low part of the wave
                }
            })
            .collect()
    }
}
