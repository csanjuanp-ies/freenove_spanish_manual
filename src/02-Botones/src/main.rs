#![no_main]
#![no_std]

use cortex_m_rt::entry;
// use embedded_hal::delay::DelayNs;
use embedded_hal::digital::InputPin;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut btn_a = board.buttons.button_a;
    let mut btn_b = board.buttons.button_b;
    let mut timer = Timer::new(board.TIMER0);

    let derecha = [
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];
    let izquierda = [
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
        [0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
    ];

    loop {
        if btn_a.is_low().unwrap() {
        display.show(&mut timer, izquierda, 1000);
        } else if btn_b.is_low().unwrap() {
        display.show(&mut timer, derecha, 1000);
        }
    }
}