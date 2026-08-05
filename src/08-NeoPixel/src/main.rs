#![no_main]
#![no_std]

use smart_leds::RGB8;
use smart_leds_trait::SmartLedsWrite;
use ws2812_nrf52833_pwm::Ws2812;

use smart_leds::{brightness};

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use rtt_target::{rprint, rprintln};
use crate::color::hsl_rgb;

#[entry]
fn main() -> ! {

    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
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
    timer.delay_ms(3000);

    rprintln!("starting loop");
    loop {
        rprintln!("\nBucle");
        let mut cur_leds: [RGB8; 8] = Default::default();
        for value in (0..360).step_by(5){
            rprint!(".");
            let mut value_in = value;
            for i in 0..8{
                value_in=value_in+i*45;
                if value_in > 360 {
                    value_in = value_in % 360;
                }
                let (red,green,blue)=hsl_rgb(value_in);
                cur_leds[i as usize] = RGB8::new(red,green,blue);
            }
            ws2812.write(brightness(cur_leds.iter().cloned(), 50)).unwrap();
            timer.delay_ms(50_u32);
        }
    }
}

mod color {
    pub fn hsl_rgb(grados_in: u16) -> (u8, u8, u8) {
        let mut grados:f32 = grados_in as f32 / 360.0 * 255.0;
        let red:f32;
        let green:f32;
        let blue:f32;

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
        (red as u8, green as u8, blue as u8)
    }
}