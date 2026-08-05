#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{hal::{
    gpio::Level,
    spi::{self, Spi},
    Timer,
}, Board};
use rtt_target::{rprint, rprintln, rtt_init_print};
use smart_leds::{brightness, RGB8, SmartLedsWrite};
use ws2812_spi::Ws2812;
use color as cls_spi;
use crate::cls_spi::hsl_rgb;

const NUM_LEDS: usize = 8;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mosi = board.edge.e00.into_push_pull_output(Level::Low).degrade();

    // Pines virtuales obligatorios para inicializar el bus SPI (no se conectan a nada físicamente)
    // Usamos pines que no colisionen con periféricos internos importantes: P1, P2
    let miso = board.edge.e01.into_floating_input().degrade();
    let sck = board.edge.e02.into_push_pull_output(Level::Low).degrade();

    // Configurar el bus SPI a 4MHz y MODE_1
    let spi_pins = spi::Pins {
        sck:  Some(sck),
        miso:  Some(miso),
        mosi: Some(mosi),
    };

    let spi = Spi::new(
        board.SPI0,
        spi_pins,
        spi::Frequency::M4,
        embedded_hal::spi::MODE_1,  // importante, si no funciona probar MODE_0
    );


    let mut ws2812 = Ws2812::new(spi);
    let leds = [  // estado inicial todo apagado
        RGB8::new(255, 0, 0),
        RGB8::new(255, 255, 0),
        RGB8::new(255, 0, 0),
        RGB8::new(255, 255, 0),
        RGB8::new(255, 0, 0),
        RGB8::new(255, 255, 0),
        RGB8::new(255, 0, 0),
        RGB8::new(255, 255, 0),
    ];

    ws2812.write(brightness(leds.iter().cloned(), 25)).unwrap();
    timer.delay_ms(3000);
    ws2812.write( brightness(leds.iter().cloned(), 5)).unwrap();
    timer.delay_ms(3000);


    loop {
        rprintln!("\nBucle");
        let mut cur_leds: [RGB8; NUM_LEDS] = Default::default();
        for value in (0..360).step_by(5){
            rprint!(".");
            let mut value_in = value;
            for i in 0..NUM_LEDS{
                value_in=value_in+i*45;
                if value_in > 360 {
                    value_in = value_in % 360;
                }
                let (red,green,blue)=hsl_rgb(value_in as u16);
                cur_leds[i] = RGB8::new(red,green,blue);
            }
            ws2812.write(brightness(cur_leds.iter().cloned(), 5)).unwrap();
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