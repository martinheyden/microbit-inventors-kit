#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use nrf52833_hal as hal;
use hal::gpio::{Pin, Input, PullUp};
use hal::gpiote::{Gpiote, GpioteChannel};
use hal::{delay::Delay, pac, prelude::*};
use hal::pac::interrupt;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

static G_GPIOTE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));

// https://dev.to/apollolabsbin/stm32f4-embedded-rust-at-the-hal-gpio-interrupts-e5

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peripherals = pac::Peripherals::take().unwrap();
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cortex_peripherals.SYST);

    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let pin5 = port0.p0_14.into_pullup_input();
    let pin11 = port0.p0_23.into_pullup_input();

    let gpiote = hal::gpiote::Gpiote::new(peripherals.GPIOTE);

    let channel1 = gpiote.channel1();
    let pin5_degrade = pin5.degrade();
    create_interrupt_event(pin5_degrade, channel1);

    let channel2 = gpiote.channel2();
    let pin11_degrade = pin11.degrade();
    create_interrupt_event(pin11_degrade, channel2);

    unsafe {
        cortex_m::peripheral::NVIC::unmask(interrupt::GPIOTE);
    }

    cortex_m::interrupt::free(|cs| {
        G_GPIOTE.borrow(cs).replace(Some(gpiote));
    });

    loop {
        delay.delay_ms(1000_u32);
    }

}


fn create_interrupt_event(pin: Pin<Input<PullUp>>, channel: GpioteChannel) {
    let event = channel.input_pin(&pin);
    event.hi_to_lo();
    event.enable_interrupt();
}

#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        let mut gpiote = G_GPIOTE.borrow(cs).borrow_mut();
        let channel1 = gpiote.as_mut().unwrap().channel1();
        if channel1.is_event_triggered() {
            rprintln!("Button 1 pressed");
            channel1.reset_events();
        }
        let channel2 = gpiote.as_mut().unwrap().channel2();
        if channel2.is_event_triggered() {
            rprintln!("Button 2 Pressed");
            channel2.reset_events();
        }
    });

    return
}