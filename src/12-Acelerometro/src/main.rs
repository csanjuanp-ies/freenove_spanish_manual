#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use libm::{atan2f, sqrtf};

use microbit::{
    hal::{twim, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{ Lsm303agr, AccelMode, AccelOutputDataRate};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut timer0 = Timer::new(board.TIMER0);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    // Inicialización del sensor y calibración
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer0,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz10,
        )
        .unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data() {  //bucle de espera activa
            timer0.delay_ms(1u32);
        }
        let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
        let (x, y, z) = (x as f32, y as f32, z as f32);
        let magnitude = sqrtf(x * x + y * y + z * z);
        rprintln!("Vector Acceleration: x {} y {} z {}", x, y, z);
        rprintln!("Valor del vector: {}", magnitude / 100.0);
    }
}