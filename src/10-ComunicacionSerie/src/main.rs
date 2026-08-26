#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::hal::uarte::{self, Baudrate, Parity};
use nrf52833_hal::Timer;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };
    let mut timer = Timer::new(board.TIMER0);

    let mut counter: u8 = 65;  // 'A' in ASCII
    serial.write(counter).unwrap();
    serial.flush().unwrap();


    loop {
        rprintln!("Counter: {}\r\n", counter);
        counter += 1;
        if counter == 127 { // 127 ascii only
            counter = 65; // next loop will be 'A'
        }
        serial.write(counter).unwrap();
        serial.flush().unwrap();
        timer.delay_ms(1000_u32);
    }
}