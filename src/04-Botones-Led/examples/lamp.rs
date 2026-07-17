#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use microbit::Board;
use nrf52833_hal::{Timer};
use nrf52833_hal::gpio::Level;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Iniciando programa");
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut led_p0 = board.edge.e00.into_pulldown_input();
    let mut led_p1 = board.edge.e01.into_push_pull_output(Level::Low);

    loop {
        let status:bool = false;
        if led_p0.is_low().unwrap() {
            timer.delay_ms(10_u32);
            if led_p0.is_low().unwrap(){
                status = !status;
                if status {
                    led_p1.set_high().unwrap();
                } else {
                    led_p1.set_low().unwrap();
                }
                while led_p0.is_low().unwrap(){
                    timer.delay_ms(10_u32);
                }
            }
        }
    }
}

