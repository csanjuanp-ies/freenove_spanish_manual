#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::Board;
use nrf52833_hal::gpio::{Level, Pin};
use nrf52833_hal::pwm::{Channel, Pwm};
use nrf52833_hal::Timer;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    use color::{write_analog, map};
    const RED: Channel = Channel::C0;
    const GREEN: Channel = Channel::C1;
    const BLUE: Channel = Channel::C2;

    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let led_azul = board.edge.e02.into_push_pull_output(Level::Low);
    let led_verde = board.edge.e01.into_push_pull_output(Level::Low);
    let led_rojo = board.edge.e00.into_push_pull_output(Level::Low);
    let pwm = Pwm::new(board.PWM0);

    pwm.set_output_pin(RED, Pin::from(led_rojo));
    pwm.set_output_pin(GREEN, Pin::from(led_verde));
    pwm.set_output_pin(BLUE, Pin::from(led_azul));


    loop {
        for grado in 0..360 {
            let (red, green, blue) = color::hsl_rgb(grado);
            write_analog(&pwm, RED, map(red));
            write_analog(&pwm, GREEN, map(green));
            write_analog(&pwm, BLUE, map(blue));
            timer.delay_ms(100_u32);
            rprintln!("Grado: {}, Red: {}, Green: {}, Blue: {}", grado, red, green, blue);
        }
    }
}

mod color {
    pub use nrf52833_hal::pwm::{Channel, Pwm};
    use nrf52833_pac::PWM0;

    pub fn map(value_in:u16) -> u16 {
        let value : f32 = value_in as f32;
        const MAX_DUTY:f32 = 2u32.pow(15) as f32;
        const MAX_RGB:f32 = 255f32;

        if value  > MAX_RGB {
            return 0;
        }

        (MAX_DUTY * ((1f32 -(MAX_RGB - value)  / MAX_RGB ))) as u16
    }

    pub fn write_analog(pwm: &Pwm<PWM0>, canal: Channel, value: u16) {
        pwm.set_duty_on(canal, value);
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