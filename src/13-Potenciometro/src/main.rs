#![no_main]
#![no_std]

use cortex_m::prelude::_embedded_hal_adc_OneShot;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::{
        saadc::{Saadc, SaadcConfig},
    },
    Board,
};
use nrf52833_hal::Timer;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let saadc_config = SaadcConfig::default();
    let mut adc = Saadc::new(board.ADC, saadc_config);
    let mut analog_pin = board.edge.e00;

    const SAMPLES_DEFAULT: u32 = 14;
    const FACTOR_CORRECCION_VOLTIOS:f32 = 0.33;

    rtt_init_print!();
    rprintln!("Inicialización completada");

    loop {
        let valor_crudo = adc.read(&mut analog_pin).unwrap() as f32;
        let voltaje = valor_crudo / (2u32.pow(SAMPLES_DEFAULT) as f32 * FACTOR_CORRECCION_VOLTIOS );
        rprintln!("Value: {} Volts {}", valor_crudo, voltaje);
        timer.delay_ms(1000_u32);
    }
}