use std::collections::VecDeque;
use std::iter::Filter;
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Devices, FromSample, Sample};
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

fn test_output() {
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

enum IODevice {
    Input,
    Output,
}

fn main() {
    let host = cpal::default_host();
    let input_device_names: Vec<_> = host
        .input_devices()
        .unwrap()
        .filter_map(|d| d.name().ok())
        .collect::<Vec<_>>();
    println!("input devices: {:?}", input_device_names);
    let output_device_names = host
        .output_devices()
        .unwrap()
        .filter_map(|d| d.name().ok())
        .collect::<Vec<_>>();
    println!("output devices: {:?}", output_device_names);
    let mut input_devices = host.input_devices().unwrap();

    let input_device = match host.input_devices().unwrap().find(|device| {
        device
            .name()
            .map(|device_name| device_name == String::from("USB Audio")) // usb aux, guitar etc
            .unwrap_or(false)
    }) {
        Some(device) => device,
        None => host.default_input_device().expect("no default device"),
    };

    let output_device = match host.output_devices().unwrap().find(|device| {
        device
            .name()
            .map(|device_name| device_name == String::from("Hyper Nova")) // headphones
            .unwrap_or(false)
    }) {
        Some(device) => device,
        None => host.default_input_device().expect("no default device"),
    };
    println!("current input device: {:?}", input_device.name().unwrap());
    println!("current output device: {:?}", output_device.name().unwrap());

    let mut supported_input_configs_range = input_device
        .supported_input_configs()
        .expect("error while querying configs");

    let supported_input_config = supported_input_configs_range
        .next()
        .expect("no supported config?")
        .with_max_sample_rate();

    let input_config = supported_input_config.config();

    let mut supported_output_configs_range = output_device
        .supported_output_configs()
        .expect("error while querying configs");

    let supported_output_config = supported_output_configs_range
        .next()
        .expect("no supported config?")
        .with_max_sample_rate();

    let output_config = supported_output_config.config();

    println!("default input sample rate: {:?}", input_config.sample_rate);
    println!("default input channels: {:?}", input_config.channels);
    println!("default input buffer size: {:?}", input_config.buffer_size);

    let audio_buffer = Arc::new(Mutex::new(VecDeque::<f32>::new()));
    let buffer_clone = audio_buffer.clone();
    let input_stream = input_device
        .build_input_stream(
            &input_config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buffer = buffer_clone.lock().unwrap();
                for &sample in data {
                    buffer.push_back(sample);
                    if buffer.len() > 8192 {
                        buffer.pop_front();
                    }
                }
            },
            {
                move |err| {
                    //react to error;
                    println!("input stream error: {:?}", err);
                }
            },
            None, //blocking, alt -> Some(Duration)=timeout
        )
        .unwrap();
    let output_stream = output_device
        .build_output_stream(
            &output_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut buffer = audio_buffer.lock().unwrap();
                for sample in data.iter_mut() {
                    *sample = buffer.pop_front().unwrap_or(0.0);
                }
            },
            {
                move |err| {
                    println!("output stream error: {:?}", err);
                }
            },
            None, //blocking, alt -> Some(Duration)=timeout
        )
        .unwrap();

    input_stream.play().unwrap();
    output_stream.play().unwrap();
    println!("playing sound press enter to stop");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
