#![no_main]
#![no_std]

use cortex_m::prelude::_embedded_hal_adc_OneShot;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use microbit::{Board, hal::saadc::{Saadc, SaadcConfig}};
use nrf52833_hal::Timer;
use nrf52833_hal::gpio::{Level, Pin};
use nrf52833_hal::pwm::{Channel, Pwm};

use crate::utils::{map, write_analog, hsl_rgb};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Inicialización .... ");

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);


    // Read analog config in edge P3 (P0.31/AIN7 --> Col Leds 3)
    let saadc_config = SaadcConfig::default();
    let mut adc = Saadc::new(board.ADC, saadc_config);
    // Apagar el pin P3 y desconectarlo de la pantalla con el into_floating_input 
    let mut read_analog_pin = board.display_pins.col3.into_floating_input();

    // Write analog config in P0, P1, P2 for RGBLed
    const RED: Channel = Channel::C0;
    const GREEN: Channel = Channel::C1;
    const BLUE: Channel = Channel::C2;
    
    let led_azul = board.edge.e00.into_push_pull_output(Level::Low);
    let led_verde = board.edge.e01.into_push_pull_output(Level::Low);
    let led_rojo = board.edge.e02.into_push_pull_output(Level::Low);
    let pwm = Pwm::new(board.PWM0);

    pwm.set_output_pin(RED, Pin::from(led_rojo));
    pwm.set_output_pin(GREEN, Pin::from(led_verde));
    pwm.set_output_pin(BLUE, Pin::from(led_azul));



    rprintln!("... completada");
    
    loop {
        let valor_crudo = adc.read(&mut read_analog_pin).unwrap();
        rprintln!("Valor crudo: {}", valor_crudo);
        // 16384 --> 2^14 muestras de la entrada analógica
        // El valor -10 se consigue viendo el potenciómetro que tengamos, variará de uno a otro.
        // Dicho valor es obtiene con el potenciómetro en su posición mínima
        let valor_mapeado = map(valor_crudo, -10, 16384, 0, 360);
        rprintln!("Valor mapeado: {}", valor_mapeado);
        let (red, green, blue) = hsl_rgb(valor_mapeado as u16);
        rprintln!("Valor RGB: {}, {}, {}", red, green, blue);

        write_analog(&pwm, RED, red);
        write_analog(&pwm, GREEN, green);
        write_analog(&pwm, BLUE, blue);
        timer.delay_ms(200_u32);
    }
}

mod utils{
    use nrf52833_hal::pwm::{Channel, Pwm};
    use nrf52833_pac::PWM0;

    /// Convierte un valor de un rango de entrada a otro rango de salida.
    pub fn map(value: i16, from_low: i16, from_high: i16, to_low: i16, to_high: i16) -> i16 {
        let input_span = (from_high - from_low) as i32;
        let output_span = (to_high - to_low) as i32;
        let scaled = (value - from_low) as i32 * output_span;

        (scaled / input_span + to_low as i32) as i16
    }

    pub fn hsl_rgb(grados_in: u16) -> (u16, u16, u16) {
        let mut grados:f32 = (grados_in as f32 / 360.0 * 255.0);
        let mut red:f32;
        let mut green:f32;
        let mut blue:f32;

        if grados < 85.0 {
            red = 255.0 - grados * 3.0;
            green = grados * 3.0;
            blue = 0.0;
        }
        else if grados < 170.0 {
            grados = grados - 85.0;
            red = 0.0;
            green = 255.0 - grados * 3.0;
            blue = grados * 3.0;
        }
        else {
            grados = grados - 170.0;
            red = grados * 3.0;
            green = 0.0;
            blue = 255.0 - grados * 3.0;
        }
        (red as u16, green as u16, blue as u16)
    }

    pub fn write_analog(pwm: &Pwm<PWM0>, canal: Channel, value: u16) {
        pwm.set_duty_on(canal, value);
    }
}
