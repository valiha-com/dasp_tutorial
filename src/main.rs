use dasp_sample::{I24, Sample};
use dasp_signal::{self as signal, Signal};

fn main() {
    sine();
    sample()
}


fn sine() {
    // Generates a sine wave signal at 1hz to be sampled 4 times per second.
    let mut signal = signal::rate(4.0).const_hz(1.0).sine();
    assert_eq!(signal.next(), 0.0); // 1
    assert_eq!(signal.next(), 1.0); // 2
    signal.next(); // 3
    assert_eq!(signal.next(), -1.0); // 4
    signal.next(); // 5
    assert_eq!(signal.next(), 1.0); // 6
    assert_ne!(signal.next(), 0.0); // out
}

fn sample() {
    assert_eq!((-1.0).to_sample::<u8>(), 0);
    assert_eq!(0.0.to_sample::<u8>(), 128);
    assert_eq!(0i32.to_sample::<u32>(), 2_147_483_648);
    assert_eq!(I24::new(0).unwrap(), Sample::from_sample(0.0));
    assert_eq!(0.0, Sample::EQUILIBRIUM);
}