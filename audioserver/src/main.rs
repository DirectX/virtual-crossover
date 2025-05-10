use anyhow::Result;
use rodio::cpal::traits::HostTrait;
use rodio::{cpal, Decoder, Device, DeviceTrait, OutputStream, Sink, Source};
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::time::Duration;

struct AudioFilter {
    fir_coefficients: Vec<f32>,
}

impl AudioFilter {
    fn new(coefficients: Vec<f32>) -> Self {
        AudioFilter { fir_coefficients: coefficients }
    }

    fn apply(&self, input: &[f32]) -> Vec<f32> {
        let mut output = Vec::new();
        for i in 0..input.len() {
            let mut sum = 0.0;
            for j in 0..self.fir_coefficients.len() {
                if i >= j {
                    sum += input[i - j] * self.fir_coefficients[j];
                }
            }
            output.push(sum);
        }
        output
    }
}

struct AudioPlayer {
    sink: Sink,
    _stream: OutputStream,
    _stream_handle: rodio::OutputStreamHandle,
}

impl AudioPlayer {
    fn new() -> Result<Self> {
        // Use default device
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        
        Ok(AudioPlayer {
            sink,
            _stream: stream,
            _stream_handle: stream_handle,
        })
    }

    fn new_with_cpal_device(device: &cpal::Device) -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_from_device(&device)?;
        let sink = Sink::try_new(&stream_handle)?;
        
        Ok(AudioPlayer {
            sink,
            _stream: stream,
            _stream_handle: stream_handle,
        })
    }

    // List all available audio devices
    fn list_devices() -> Vec<cpal::Device> {
        let host = cpal::default_host();
        host.output_devices()
            .expect("Failed to get audio devices")
            .collect()
    }

    // Get device name
    fn get_device_name(device: &Device) -> String {
        device.name()
            .unwrap_or_else(|_| "Unknown Device".to_string())
    }

    fn play_file(&self, path: &str) -> Result<()> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file)?;
        
        // Convert i16 samples to f32 and apply low-pass filter
        let converted_source = source.convert_samples::<f32>();
        let filtered_source = converted_source;
        self.sink.append(filtered_source);
        self.sink.play();
        
        Ok(())
    }

    fn apply_low_pass(&self, path: &str) -> Result<()> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file)?;

        // // Define 6-order FIR filter coefficients
        // // These coefficients should be adjusted based on the desired filter response
        // let fir_coefficients = [0.1, 0.2, 0.3, 0.3, 0.2, 0.1]; // Example coefficients

        // // Apply the FIR filter to the source
        // let filtered_source = rodio::source::Zero; // ::source::Source::filter(
        //     converted_source,
        //     move |samples: &mut [f32], _: &cpal::SampleFormat| {
        //         // Buffer for previous samples (for FIR calculation)
        //         let mut buffer = vec![0.0; fir_coefficients.len()];

        //         for sample in samples.iter_mut() {
        //             // Shift buffer values
        //             for i in (1..buffer.len()).rev() {
        //                 buffer[i] = buffer[i-1];
        //             }
        //             buffer[0] = *sample;

        //             // Apply FIR filter
        //             let mut filtered_value = 0.0;
        //             for (i, coeff) in fir_coefficients.iter().enumerate() {
        //                 filtered_value += buffer[i] * coeff;
        //             }

        //             *sample = filtered_value;
        //         }
        //     },
        // );
        
        // Convert i16 samples to f32 and apply low-pass filter
        let converted_source = source.convert_samples::<f32>();
        let filtered_source = converted_source.low_pass(200);
        self.sink.append(filtered_source);
        self.sink.play();
        
        Ok(())
    }

    fn pause(&self) {
        self.sink.pause();
    }

    fn resume(&self) {
        self.sink.play();
    }

    fn stop(&self) {
        self.sink.stop();
    }

    fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    // Basic high-pass filter implementation
    fn apply_high_pass(&self, path: &str, cutoff_frequency: u32) -> Result<()> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file)?;
        let converted_source = source.convert_samples::<f32>();
        let filtered_source = converted_source.high_pass(cutoff_frequency);
        self.sink.append(filtered_source);
        self.sink.play();
        Ok(())
    }
}

fn select_device() -> Result<Device> {
    let devices = AudioPlayer::list_devices();
    
    // println!("Available audio devices:");
    // for (idx, device) in devices.iter().enumerate() {
    //     println!("{}: {}", idx, AudioPlayer::get_device_name(device));
    // }

    // print!("Select device number: ");
    // io::stdout().flush()?;
    
    // let mut input = String::new();
    // io::stdin().read_line(&mut input)?;
    
    // let device_idx = input.trim().parse::<usize>()?;

    let device_idx = 0;
    devices.get(device_idx)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Invalid device index"))
}

fn main() -> Result<()> {
    // // List all available audio devices
    // println!("Available audio devices:");
    // let devices = AudioPlayer::list_devices();
    // for (idx, device) in devices.iter().enumerate() {
    //     println!("{}: {}", idx, AudioPlayer::get_device_name(device));
    // }

    // Example: Use the first available device (if any)
    let player = if let Ok(device) = select_device().as_ref() {
        println!("Using device: {}", AudioPlayer::get_device_name(device));
        AudioPlayer::new_with_cpal_device(device)?
    } else {
        println!("No devices found, using default");
        AudioPlayer::new()?
    };
    
    // // Example usage
    // println!("Playing audio file...");
    // player.play_file("./test.mp3")?;
    
    // // Set volume (0.0 to 1.0)
    player.set_volume(0.7);
    
    // // Wait for a few seconds
    // std::thread::sleep(Duration::from_secs(5));
    
    // // Pause playback
    // println!("Pausing...");
    // player.pause();
    // std::thread::sleep(Duration::from_secs(2));
    
    // // Resume playback
    // println!("Resuming...");
    // player.resume();
    // std::thread::sleep(Duration::from_secs(5));
    
    println!("Applying high-pass filter...");
    player.play_file("./audioserver/test.mp3")?;
    std::thread::sleep(Duration::from_secs(27));
    player.stop();

    // // Apply high-pass filter
    // println!("Applying high-pass filter...");
    // player.apply_high_pass("./audioserver/test.mp3", 200)?;
    // std::thread::sleep(Duration::from_secs(10));
    // player.stop();

    // println!("Playing audio file...");
    // player.apply_low_pass("./audioserver/test.mp3")?;
    // std::thread::sleep(Duration::from_secs(10));
    
    // Stop playback
    println!("Stopping...");
    player.stop();

    Ok(())
}
