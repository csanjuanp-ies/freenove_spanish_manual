#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use nrf52833_hal::{gpio, pac, Timer};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Iniciando programa");


    let peripherals = pac::Peripherals::take().unwrap();
    rprintln!("Periféricos adquiridos");
    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let mut timer = Timer::new(peripherals.TIMER0);
    let mut led = p0.p0_02.into_push_pull_output(gpio::Level::Low);

    loop {
        timer.delay_ms(1000_u32);
        rprintln!("Encendiendo");
        led.set_high().unwrap();
        timer.delay_ms(1000_u32);
        rprintln!("Apagando");
        led.set_low().unwrap();
    }
}