#![no_main]
#![no_std]

use core::f32::consts;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use libm::{atan2f, sqrtf};

use microbit::{
    hal::{twim, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{mode, Lsm303agr, MagMode, MagOutputDataRate, AccelMode, AccelOutputDataRate};
use microbit::display::blocking::Display;
use mag_cal::calc_calibration; //  Nuestra función de calibración

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut timer0 = Timer::new(board.TIMER0);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    // Inicialización del sensor y calibración
    sensor.init().unwrap();
    sensor
        .set_mag_mode_and_odr(
            &mut timer0,
            MagMode::HighResolution,
            MagOutputDataRate::Hz10,
        )
        .unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer0,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz10,
        )
        .unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();
    let _ = calc_calibration(&mut sensor, &mut display, &mut timer0);

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data() {  //bucle de espera activa
            timer0.delay_ms(1u32);
        }
        let (x, y, z) = sensor.magnetic_field().unwrap().xyz_nt();
        let (x, y, z) = (x as f32, y as f32, z as f32);
        rprintln!("Vector Magnetic Field: x {} y {} z {}", x, y, z);
        let magnitude = sqrtf(x * x + y * y + z * z);
        rprintln!("Valor del vector: {}", magnitude / 100.0);
        let theta = atan2f(y, x);
        rprintln!("Ángulo en el plano xy: {} deg", theta * 180.0 / consts::PI);
    }
}