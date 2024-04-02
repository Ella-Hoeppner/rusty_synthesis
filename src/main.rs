use signal::*;

mod output;
mod signal;

fn main() {
  let signal = Product::new(
    WithFrequency::new(ToUni::new(Cos::new()), 321.),
    WithFrequency::new(Wavetabled::new(Saw::new()), 440.),
  );
  output::begin(Box::new(signal)).unwrap();
}
