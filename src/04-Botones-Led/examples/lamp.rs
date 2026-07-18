#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
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
    let mut status:bool = true;

    loop {

        if led_p0.is_low().unwrap() {
            rprintln!("Pulsado");
            timer.delay_ms(100_u32);
            if led_p0.is_low().unwrap(){
                if status {
                    rprintln!("On");
                    status = false;
                    led_p1.set_high().unwrap();
                } else {
                    rprintln!("Off");
                    status = true;
                    led_p1.set_low().unwrap();
                }
            }
            rprintln!("fin if");
            while led_p0.is_low().unwrap(){
                rprintln!("while");
                timer.delay_ms(100_u32);
            }
        }
    }
}

