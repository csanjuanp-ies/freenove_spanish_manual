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
use nrf52833_hal::gpio::{Level, Pin};
use nrf52833_hal::pwm::{Channel, Pwm};
use nrf52833_hal::Timer;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // Read analog config in P0
    let saadc_config = SaadcConfig::default();
    let mut adc = Saadc::new(board.ADC, saadc_config);
    let mut read_analog_pin = board.edge.e00;

    // Write analog config in P1
    const CERO: Channel = Channel::C0;
    let write_analog_pin = board.edge.e01.into_push_pull_output(Level::Low);
    let pwm = Pwm::new(board.PWM0);
    pwm.set_output_pin(CERO, Pin::from(write_analog_pin));

    rtt_init_print!();
    rprintln!("Inicialización completada");

    loop {
        let valor_crudo = adc.read(&mut read_analog_pin).unwrap();
        utils::write_analog(&pwm, CERO, valor_crudo);
        timer.delay_ms(200_u32);
    }
}

mod utils{
    use nrf52833_hal::pwm::{Channel, Pwm};
    use nrf52833_pac::PWM0;
    // La configuración por defecto hace 2^14 muestras
    const ADC_MULT: u16 = 2; // para una frecuencia máxima de 32768 Hz sobre las 2^14 muestras

    pub fn write_analog(pwm: &Pwm<PWM0>, canal: Channel, value: i16) {
        pwm.set_duty_on(canal, u16::try_from(value).unwrap_or_default() * ADC_MULT);
    }
}
