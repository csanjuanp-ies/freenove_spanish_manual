#![no_main]
#![no_std]

use cortex_m::prelude::_embedded_hal_adc_OneShot;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin};
use microbit::Board;
use nrf52833_hal::{Saadc, Timer};
use nrf52833_hal::saadc::SaadcConfig;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Inicialización .... ");
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut saadc_config = SaadcConfig::default();
    saadc_config.resolution = nrf52833_hal::saadc::Resolution::_10BIT;  // En este caso muestreamos a 10 bits
    let mut adc = Saadc::new(board.ADC, saadc_config);
    let mut pin_x = board.edge.e02;
    let mut pin_y = board.edge.e01;
    let mut pin_z = board.edge.e00.into_pullup_input();

    rprintln!("Inicialización completada");
    loop {
        let val_x = adc.read(&mut pin_x).unwrap();
        let val_y = adc.read(&mut pin_y).unwrap();
        let val_z = pin_z.is_low().unwrap() as bool;

        rprintln!("({},{},{})", val_x, val_y, val_z);
        timer.delay_ms(200_u32);
    }
}
