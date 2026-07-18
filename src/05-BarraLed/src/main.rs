#![no_main]
#![no_std]

// Versión más fácil
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{OutputPin};
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

    let mut led_p0 = board.edge.e00.into_push_pull_output(Level::Low);
    let mut led_p1 = board.edge.e01.into_push_pull_output(Level::Low);
    let mut led_p2 = board.edge.e02.into_push_pull_output(Level::Low);

    let mut led_p3 = board.display_pins.col3.into_push_pull_output(Level::Low);
    let mut led_p4 = board.display_pins.col1.into_push_pull_output(Level::Low);
    let mut led_p10 = board.display_pins.col5.into_push_pull_output(Level::Low);
    let mut led_p6 = board.display_pins.col4.into_push_pull_output(Level::Low);
    let mut led_p7 = board.display_pins.col2.into_push_pull_output(Level::Low);

    let mut led_p9 = board.edge.e09.into_push_pull_output(Level::Low);
    let mut led_p8 = board.edge.e08.into_push_pull_output(Level::Low);

    loop {
        led_p0.set_high().unwrap();
        timer.delay_ms(500_u32);
        led_p0.set_low().unwrap();
        led_p1.set_high().unwrap();
        timer.delay_ms(500_u32);
        led_p1.set_low().unwrap();
        led_p2.set_high().unwrap();
        timer.delay_ms(500_u32);
        led_p2.set_low().unwrap();
        led_p3.set_high().unwrap();
        timer.delay_ms(500_u32);
        led_p3.set_low().unwrap();
        led_p4.set_high().unwrap();
        timer.delay_ms(500_u32);
        led_p4.set_low().unwrap();
        led_p10.set_high().unwrap();
        timer.delay_ms(500_u32);
        led_p10.set_low().unwrap();
        led_p6.set_high().unwrap();
        timer.delay_ms(500_u32);
        led_p6.set_low().unwrap();
        led_p7.set_high().unwrap();
        timer.delay_ms(500_u32);
        led_p7.set_low().unwrap();
        led_p9.set_high().unwrap();
        timer.delay_ms(500_u32);
        led_p9.set_low().unwrap();
        led_p8.set_high().unwrap();
        timer.delay_ms(500_u32);
        led_p8.set_low().unwrap();
    }
}
