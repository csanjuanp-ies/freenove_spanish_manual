#![no_main]
#![no_std]

use cortex_m::prelude::_embedded_hal_adc_OneShot;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use microbit::{Board, hal::saadc::{Saadc, SaadcConfig}};
use smart_leds::RGB8;
use smart_leds_trait::SmartLedsWrite;
use ws2812_nrf52833_pwm::Ws2812;
use smart_leds::{brightness};
use embedded_hal::delay::DelayNs;
use microbit::{hal::Timer};

use crate::utils::{map, hsl_rgb};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Inicialización .... ");

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // Configuración de la ruleta de LEds apagado de inicio. Gestión a través de P0
    let pin = board.edge.e00.degrade();
    let mut ws2812: Ws2812<{ 8 * 24 }, _> = Ws2812::new(board.PWM0, pin);
    let leds = [  // estado inicial todo apagado
        RGB8::new(0, 0, 0),
        RGB8::new(0, 0, 0),
        RGB8::new(0, 0, 0),
        RGB8::new(0, 0, 0),
        RGB8::new(0, 0, 0),
        RGB8::new(0, 0, 0),
        RGB8::new(0, 0, 0),
        RGB8::new(0, 0, 0),
    ];
    ws2812.write(brightness(leds.iter().cloned(), 50)).unwrap();


    // Read analog config in edge P1
    let saadc_config = SaadcConfig::default();
    let mut adc = Saadc::new(board.ADC, saadc_config);
    let mut read_analog_pin = board.edge.e01;

    rprintln!("... completada");

    let mut cur_leds: [RGB8; 8] = Default::default();
    loop {
        for val in 0..8 {  // Leemos 8 veces, una para cada led de la ruleta
            let valor_crudo = adc.read(&mut read_analog_pin).unwrap();
            let valor_mapeado = map(valor_crudo, -10, 16384, 0, 360);
            let (red, green, blue) = hsl_rgb(valor_mapeado as u16);
            cur_leds[val as usize] = RGB8::new(red as u8, green as u8, blue as u8);
        }

        // Lo mandamos a la ruleta
        ws2812.write(brightness(cur_leds.iter().cloned(), 50)).unwrap();
        timer.delay_ms(200_u32);
    }
}

mod utils{
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
}
