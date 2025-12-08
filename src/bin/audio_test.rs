use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use rand::random_range;

fn write_test_audio<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo)
where
    T: Sample + FromSample<f32>,
{
    for sample in data.iter_mut() {
        let noise_value: f32 = random_range(-0.1..0.1);
        *sample = Sample::from_sample(noise_value);
    }
}

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?")
        .with_max_sample_rate();
    let config = supported_config.config();
    let stream = device
        .build_output_stream(
            &config,
            write_test_audio::<f32>,
            move |err| {
                //react to error;
                println!("white noise error: {:?}", err);
            },
            None, //blocking, alt -> Some(Duration)=timeout
        )
        .unwrap();
    stream.play().unwrap();
    println!("playing sound press enter to stop");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
}
