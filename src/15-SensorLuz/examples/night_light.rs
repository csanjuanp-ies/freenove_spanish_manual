#![no_main]
#![no_std]

use cortex_m::prelude::_embedded_hal_adc_OneShot;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use microbit::Board;
use nrf52833_hal::saadc::SaadcConfig;
use nrf52833_hal::{Saadc, Timer};
use nrf52833_hal::gpio::Level;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Inicialización .... ");
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut saadc_config = SaadcConfig::default();
    saadc_config.resolution = nrf52833_hal::saadc::Resolution::_10BIT;  // En este caso muestreamos a 10 bits
    let mut adc = Saadc::new(board.ADC, saadc_config);
    let mut analog_pin = board.edge.e00;

    let mut led = board.edge.e01.into_push_pull_output(Level::Low);

    rprintln!("Inicialización completada");
    loop {
        let valor_crudo = adc.read(&mut analog_pin).unwrap();
        if valor_crudo < 400 {
            rprintln!("Luz detectada: {} (Apagando LED)", valor_crudo);
            led.set_low().unwrap();
        } else {
            rprintln!("Oscuridad detectada: {} (Encendiendo LED)", valor_crudo);
            led.set_high().unwrap();
        }
        timer.delay_ms(200_u32);
    }
}