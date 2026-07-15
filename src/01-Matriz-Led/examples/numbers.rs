#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

include!("lib_cap.rs");

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut light_heart_small = [[0; 5];5];
    let data = [1,2,0,5,1];

    loop {
        for num in data {
            lib_cap::number_to_display(num, &mut light_heart_small);
            display.show(&mut timer, light_heart_small, 1000);
            timer.delay_ms(500_u32);
        }
    }
}