use crate::signal::Signal;
use anyhow::{Error, Result};
use cpal::{
  traits::{DeviceTrait, HostTrait, StreamTrait},
  FromSample, Sample, SizedSample,
};

pub fn begin(signal: Box<dyn Signal>) -> Result<()> {
  let host = cpal::default_host();

  let device = host
    .default_output_device()
    .expect("failed to find output device");
  //println!("Output device: {}", device.name()?);

  let config = device.default_output_config().unwrap();
  //println!("Default output config: {:?}", config);

  match config.sample_format() {
    cpal::SampleFormat::I8 => run::<i8>(&device, &config.into(), signal),
    cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), signal),
    // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into()),
    cpal::SampleFormat::I32 => run::<i32>(&device, &config.into(), signal),
    // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into()),
    cpal::SampleFormat::I64 => run::<i64>(&device, &config.into(), signal),
    cpal::SampleFormat::U8 => run::<u8>(&device, &config.into(), signal),
    cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), signal),
    // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into()),
    cpal::SampleFormat::U32 => run::<u32>(&device, &config.into(), signal),
    // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into()),
    cpal::SampleFormat::U64 => run::<u64>(&device, &config.into(), signal),
    cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), signal),
    cpal::SampleFormat::F64 => run::<f64>(&device, &config.into(), signal),
    sample_format => panic!("Unsupported sample format '{sample_format}'"),
  }
}

pub fn run<T>(
  device: &cpal::Device,
  config: &cpal::StreamConfig,
  mut signal: Box<dyn Signal>,
) -> Result<()>
where
  T: SizedSample + FromSample<f32>,
{
  let sample_rate = config.sample_rate.0 as f64;
  let channels = config.channels as usize;

  let mut sample_counter = 0f64;
  let mut next_value = move || {
    sample_counter = sample_counter + 1.0;
    signal.sample(sample_counter / sample_rate) as f32
  };

  let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

  let stream = device.build_output_stream(
    config,
    move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
      write_data(data, channels, &mut next_value)
    },
    err_fn,
    None,
  )?;
  stream.play()?;

  loop {
    std::thread::sleep(std::time::Duration::from_millis(100));
  }

  Ok(())
}

fn write_data<T>(
  output: &mut [T],
  channels: usize,
  next_sample: &mut dyn FnMut() -> f32,
) where
  T: Sample + FromSample<f32>,
{
  for frame in output.chunks_mut(channels) {
    let value: T = T::from_sample(next_sample());
    for sample in frame.iter_mut() {
      *sample = value;
    }
  }
}
