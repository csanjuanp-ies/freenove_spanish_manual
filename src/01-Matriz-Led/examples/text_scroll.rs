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
    let data = "0123";

    loop {
        for num in data.chars() {
            lib_cap::number_to_display(num.to_digit(10).unwrap() as u8, &mut light_heart_small);
            display.show(&mut timer, light_heart_small, 1000);
            for _ in 0..=5 {
                display.show(&mut timer, light_heart_small, 500);
                timer.delay_ms(10_u32);
                lib_cap::rotate_column_matrix(&mut light_heart_small);
            }
        }
    }
}