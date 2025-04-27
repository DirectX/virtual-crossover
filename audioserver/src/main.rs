use std::{fs::File, io::BufReader, time::Duration};
use rodio::{OutputStream, Sink, Source};
use minimp3::{Decoder, Frame};
use biquad::{Biquad, Coefficients, DirectForm1, ToHertz, Type, Q_BUTTERWORTH_F32};

struct FilteredPannedSource {
    samples: Vec<f32>, // interleaved stereo samples
    index: usize,
    sample_rate: u32,
}

impl Iterator for FilteredPannedSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.samples.len() {
            None
        } else {
            let sample = self.samples[self.index];
            self.index += 1;
            Some(sample)
        }
    }
}

impl Source for FilteredPannedSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.samples.len() / 2)
    }

    fn channels(&self) -> u16 {
        2 // stereo
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f64(
            self.samples.len() as f64 / self.sample_rate as f64 / 2.0,
        ))
    }
}

fn main() {
    // Load MP3 file
    println!("Loading mpg");
    let file = File::open("./audioserver/test.mp3").expect("failed to open mp3");
    println!("Loaded");
    let mut decoder = Decoder::new(BufReader::new(file));
    println!("Decoder created");

    let sample_rate = 44100;

    // Setup filters
    let filter_coeffs = Coefficients::<f32>::from_params(
        Type::BandPass,
        sample_rate.hz(),
        1000.hz(),
        Q_BUTTERWORTH_F32,
    ).unwrap();
    println!("Filter coeffs: {:?}", filter_coeffs);

    let mut left_filter = DirectForm1::<f32>::new(filter_coeffs);
    let mut right_filter = DirectForm1::<f32>::new(filter_coeffs);
    println!("Filters created");

    println!("Decoder");

    let mut interleaved_samples = Vec::new();

    println!("Decoder: {:?}", decoder.next_frame());

    // Decode and filter
    while let Ok(Frame { data, channels, .. }) = decoder.next_frame() {
        println!("frame: {} bytes, {} channels", data.len(), channels);

        for i in 0..(data.len() / channels) {
            let left = data[i * channels] as f32 / 32768.0;
            let right = if channels > 1 {
                data[i * channels + 1] as f32 / 32768.0
            } else {
                left
            };

            // Apply band-pass filtering
            let filtered_left = left_filter.run(left);
            let filtered_right = right_filter.run(right);

            // Example: pan to **right channel** only
            interleaved_samples.push(filtered_left);
            interleaved_samples.push(filtered_right);
        }
    }

    // Set up rodio output
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = FilteredPannedSource {
        samples: interleaved_samples,
        index: 0,
        sample_rate,
    };

    sink.append(source);
    sink.sleep_until_end();
}
