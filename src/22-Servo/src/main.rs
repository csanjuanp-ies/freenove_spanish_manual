#![no_main]
#![no_std]


use cortex_m::prelude::_embedded_hal_adc_OneShot;
use cortex_m_rt::entry;

use microbit::Board;
use nrf52833_hal::gpio::{Level};  //Pin
use nrf52833_hal::Saadc;
use nrf52833_hal::saadc::SaadcConfig;
use panic_halt as _;
use nrf52833_hal::pwm::{Channel, Pwm};
use rtt_target::{rprintln, rtt_init_print};
use crate::utils::write_pwm0;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();

    // el potenciomentro en el P0
    let mut potentiometer = board.edge.e00;
    const UNO: Channel = Channel::C0;

    let mut saadc_config = SaadcConfig::default();
    saadc_config.resolution = nrf52833_hal::saadc::Resolution::_10BIT;  //1024 valores posibles
    let mut adc = Saadc::new(board.ADC, saadc_config);

    // un pin del motor al P1
    let pin_01 = board.edge.e01.into_push_pull_output(Level::Low).degrade();
    let pwm_pin_01 = Pwm::new(board.PWM0);
    pwm_pin_01.set_output_pin(UNO, pin_01);

    loop {
        let valor_crudo = adc.read(&mut potentiometer).unwrap();
        rprintln!("Valor crudo: {}", valor_crudo);
        let output_value = (((411.0 - valor_crudo as f32) / 411.0) * 1023.0) as i16;
        rprintln!("Output value: {}", output_value);
        write_pwm0(&pwm_pin_01, UNO, output_value);
        pwm_pin_01.set_period(nrf52833_hal::time::Hertz(20_000u32));
        
    }
}
mod utils{
    use nrf52833_hal::pwm::{Channel, Pwm};
    use nrf52833_pac::PWM0;
    const ADC_MULT: u16 = 1;

    pub fn write_pwm0(pwm: &Pwm<PWM0>, canal: Channel, value: i16) {
        pwm.set_duty_on(canal, u16::try_from(value).unwrap_or_default() * ADC_MULT);
    }

}
