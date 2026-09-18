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

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Inicialización .... ");
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // Configurando la matriz de led para leer la luz ambiental
    let mut led_matrix = board.display_pins;
    // Limpiamos la matriz de led
    let mut row1 = led_matrix.row1.into_push_pull_output(microbit::hal::gpio::Level::Low);
    let mut col1 = led_matrix.col1.into_push_pull_output(microbit::hal::gpio::Level::Low);
    timer.delay_ms(1_u32);
    // Reverse bias the integrated LED (Row HIGH, Col LOW)
    row1.set_high().unwrap();
    timer.delay_ms(2_u32);
    // Analog input
    let mut saadc_config = SaadcConfig::default();
    saadc_config.resolution = nrf52833_hal::saadc::Resolution::_8BIT;  // En este caso muestreamos a 8 bits
    let mut adc = Saadc::new(board.ADC, saadc_config);
    let mut analog_pin = col1.into_floating_input();

    rprintln!("Inicialización completada");
    loop {
        let valor_crudo = adc.read(&mut analog_pin).unwrap();
        rprintln!("Valor crudo: {}", valor_crudo);
        timer.delay_ms(200_u32);
    }
}
