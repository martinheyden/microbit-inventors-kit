#![no_main]
#![no_std]

use cortex_m_rt::entry;
use hal::Saadc;
use hal::saadc::SaadcConfig;
use nrf52833_hal as hal;
use hal::{delay::Delay, pac, prelude::*};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

// Successive Approximation Analog to Digital Conversion (SAADC)
// https://docs.rs/nrf52833-hal/latest/nrf52833_hal/saadc/index.html

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peripherals = pac::Peripherals::take().unwrap();
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cortex_peripherals.SYST);

    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);

    let saadc_config = SaadcConfig::default();
    let mut saadc = Saadc::new(peripherals.SAADC, saadc_config);
    let mut saadc_pin = port0.p0_02;

    loop {
        delay.delay_ms(1000_u32);
        let read = saadc.read(&mut saadc_pin);
        rprintln!("Reading {} V", convert_value(read.unwrap(), 3.0f32));
    }

}


fn convert_value(val: i16, max: f32) -> f32 {
    (val as f32)/(i16::max_value() as f32) * max
}


