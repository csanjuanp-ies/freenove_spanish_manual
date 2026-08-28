#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::{twim, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{ Lsm303agr, AccelMode, AccelOutputDataRate};
use microbit::display::blocking::Display;

use utils::mapping;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut timer0 = Timer::new(board.TIMER0);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    let mut leds = [[0u8; 5]; 5];
    let mut display = Display::new(board.display_pins);

    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer0,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz10,
        )
        .unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    rprintln!("Iniciando bucle principal");
    loop {
        while !sensor.mag_status().unwrap().xyz_new_data() {  //bucle de espera activa
            timer0.delay_ms(1u32);
        }
        let (x, y, _) = sensor.acceleration().unwrap().xyz_mg();
        let x = mapping(x) as usize;
        let y = mapping(y) as usize;

        rprintln!("x: {}, y: {}", x, y);
        leds[y][x] = 255u8;
        display.show(&mut timer0, leds, 50);
        leds[y][x] = 0u8;
        display.show(&mut timer0, leds, 50);
    }
}

mod utils {
    pub fn mapping(value: i32) -> i32 {
        let mut value_ret = value;

        // limitar el valor a un rango de -400 a 400
        if value < -400 { value_ret-=400; }
        else if value > 400 { value_ret = 400; }

        // mapear el valor a un rango de 0 a 4
        value_ret = (value_ret + 400) / 200;

        if value_ret < 0 { value_ret = 0; }
        else if value_ret > 4 { value_ret = 4; }

        value_ret
    }
}
