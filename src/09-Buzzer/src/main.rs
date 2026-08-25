#![no_main]
#![no_std]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use microbit::Board;
use nrf52833_hal::gpio::{Level, Pin};
use nrf52833_hal::pwm::{Channel, Pwm};
use nrf52833_hal::Timer;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Iniciando programa");
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut led = board.edge.e00.into_push_pull_output(Level::Low);

    loop {
        for _ in 0..4 {
            rprintln!("Encendiendo");
            led.set_high().unwrap();
            timer.delay_ms(100_u32);
            rprintln!("Apagando");
            led.set_low().unwrap();
            timer.delay_ms(100_u32);
        }
        timer.delay_ms(500_u32);
    }

}