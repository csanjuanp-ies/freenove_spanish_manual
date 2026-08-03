#![no_main]
#![no_std]

use embedded_hal::delay::DelayNs;
use nrf52833_hal::{pwm, Timer};
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::Board;
use panic_halt as _;
use nrf52833_hal::gpio::{Level, Pin};
use nrf52833_hal::pwm::{Channel, Pwm};

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut led = board.edge.e00.into_push_pull_output(Level::Low);
    let duty_values = [ 0, 4000, 8000, 12000, 16000, 20000, 24000, 28000, 32000 ];

    led.set_high().unwrap();
    timer.delay_ms(1000_u32);

    let pwm = Pwm::new(board.PWM0);
    pwm.set_output_pin(Channel::C0, Pin::from(led)); // Asignar el Pin P0.02 - RING0 - P0 al canal 0
    let max_duty = pwm.max_duty();

    loop {
        // Efecto "Fade In": Incrementa el brillo
        for duty in duty_values {
            pwm.set_duty_on_common(duty);
            timer.delay_ms(200_u32);
        }

        pwm.set_duty_on_common(max_duty);
        timer.delay_ms(500_u32);

        // Efecto "Fade Out": Decrementa el brillo
        for duty in duty_values.into_iter().rev() {
            pwm.set_duty_on_common(duty);
            timer.delay_ms(200_u32);
        }
    }
}