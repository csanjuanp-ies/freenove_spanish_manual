#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{StatefulOutputPin};
use microbit::Board;
use nrf52833_hal::{Timer};
use nrf52833_hal::gpio::{Level, Output, Pin, PushPull};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};


#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Iniciando programa");
    let board = Board::take().unwrap();
    let mut leds:[Pin<Output<PushPull>>; 10] = [
        board.edge.e00.degrade().into_push_pull_output(Level::Low),
        board.edge.e01.degrade().into_push_pull_output(Level::Low),
        board.edge.e02.degrade().into_push_pull_output(Level::Low),
        board.display_pins.col3.degrade().into_push_pull_output(Level::Low),
        board.display_pins.col1.degrade().into_push_pull_output(Level::Low),
        board.display_pins.col5.degrade().into_push_pull_output(Level::Low),
        board.display_pins.col4.degrade().into_push_pull_output(Level::Low),
        board.display_pins.col2.degrade().into_push_pull_output(Level::Low),
        board.edge.e09.degrade().into_push_pull_output(Level::Low),
        board.edge.e08.degrade().into_push_pull_output(Level::Low)
    ];
    let mut timer = Timer::new(board.TIMER0);

    loop {
        for led in leds.iter_mut() {
            led.toggle().ok();
            timer.delay_ms(500u32);
            led.toggle().ok();
        }
    }
}


