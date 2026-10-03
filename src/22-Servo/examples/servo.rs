#![no_main]
#![no_std]

use cortex_m::prelude::_embedded_hal_adc_OneShot;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::Board;
use nrf52833_hal::gpio::{Level};
use nrf52833_hal::{pwm, Timer, Saadc};
use panic_halt as _;
use nrf52833_hal::pwm::{Channel, Pwm};
use nrf52833_hal::saadc::SaadcConfig;
use rtt_target::{rprintln, rtt_init_print};
use utils::{write_pwm0, map};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // el potenciómentro en el P0 (capítulo 14)
    let mut potentiometer = board.edge.e00;
    let mut saadc_config = SaadcConfig::default();
    saadc_config.resolution = nrf52833_hal::saadc::Resolution::_10BIT;  //1024 valores posibles
    let mut adc = Saadc::new(board.ADC, saadc_config);


    // un pin del motor al P1
    const UNO: Channel = Channel::C0;
    let pin_00 = board.edge.e01.into_push_pull_output(Level::Low).degrade();
    let pwm_pin_00 = Pwm::new(board.PWM0);
    pwm_pin_00.set_output_pin(UNO, pin_00);
    // Configuramos el PWM para 50Hz-->200 en duty será 0.5ms (0º) y 1200 en duty será 2.5ms (180º)
    pwm_pin_00.set_prescaler(pwm::Prescaler::Div32);  // 500_000 /max_duty = 50Hz
    pwm_pin_00.set_max_duty(10_000u16);
    pwm_pin_00.loop_inf();

    loop {
        let valor_crudo = adc.read(&mut potentiometer).unwrap();
        let map_value = map(valor_crudo, -1, 1023, 200, 1200);
        rprintln!("map_value: {}-{}", valor_crudo, map_value);
        write_pwm0(&pwm_pin_00, UNO, map_value);
        timer.delay_ms(1000_u32);
    }
}
mod utils{
    use nrf52833_hal::pwm::{Channel, Pwm};
    use nrf52833_pac::PWM0;
    pub fn write_pwm0(pwm: &Pwm<PWM0>, canal: Channel, value: u16) {
        pwm.set_duty_off(canal, value);
    }
    pub fn map(value: i16, from_low: i16, from_high: i16, to_low: i16, to_high: i16) -> u16 {
        let input_span = (from_high - from_low) as i32;
        let output_span = (to_high - to_low) as i32;
        let scaled = (value - from_low) as i32 * output_span;

        (scaled / input_span + to_low as i32) as u16
    }
}
