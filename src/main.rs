#![no_main]
#![no_std]

use embedded_hal::digital::v2::OutputPin;
use nrf52840_hal as hal;
use hal::gpio::Level;
use rtt_target::{rprintln, rtt_init_print};

use embedded_hal::prelude::*;
use asm_delay::AsmDelay;

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let mut led = port0.p0_22.into_push_pull_output(Level::Low);

    rprintln!("Blinky button demo starting");
    led.set_high().unwrap();

    let mut delay = AsmDelay::new(asm_delay::bitrate::U32BitrateExt::mhz(64));

    let mut odd = true;

    loop {
        if odd {
            rprintln!("off");
            odd = false;
            led.set_low().unwrap();
        } else {
            rprintln!("on");
            odd = true;
            led.set_high().unwrap();
        }
        delay.delay_ms(1000u32);
    }
}
