#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    board::Board,
    hal::{
        timer::Timer,
    },
};
use panic_halt as _;
use crate::utils::ShiftRegister;


#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // DS Pin of 74HC595(Pin14)
    let data_pin = board.edge.e00.into_push_pull_output(microbit::hal::gpio::Level::Low).degrade();
    // ST_CP Pin of 74HC595(Pin12)
    let latch_pin = board.edge.e01.into_push_pull_output(microbit::hal::gpio::Level::Low).degrade();
    // SH_CP Pin of 74HC595(Pin11)
    let clock_pin = board.edge.e02.into_push_pull_output(microbit::hal::gpio::Level::Low).degrade();

    let mut shift_register = ShiftRegister::new(data_pin, clock_pin, latch_pin);
    loop {
        // Pattern: Moving single LED from left to right
        for i in 0..8 {
            shift_register.write_byte(1 << i);
            timer.delay_ms(500u32);
        }
    }
}

mod utils{
    use embedded_hal::digital::OutputPin;
    use nrf52833_hal::gpio::{Output, Pin, PushPull};

    // A helper struct to wrap our shift register pins
    pub struct ShiftRegister {
        data: Pin<Output<PushPull>>,
        clock: Pin<Output<PushPull>>,
        latch: Pin<Output<PushPull>>,
    }

    impl ShiftRegister {
        pub fn new(data: Pin<Output<PushPull>>, clock: Pin<Output<PushPull>>, latch: Pin<Output<PushPull>>) -> Self {
            let mut sr = ShiftRegister { data, clock, latch };
            let _ = sr.clock.set_low();
            let _ = sr.latch.set_low();
            sr
        }

        /// Shifts out an 8-bit value (MSB first) and latches it to the outputs
        pub fn write_byte(&mut self, value: u8) {
            // 1. Shift all 8 bits sequentially
            for i in (0..8).rev() {
                // Determine if the specific bit is 1 or 0
                if (value & (1 << i)) != 0 {
                    let _ = self.data.set_high();
                } else {
                    let _ = self.data.set_low();
                }
                // Pulse the Shift Clock (SRCLK) High then Low
                let _ = self.clock.set_high();
                let _ = self.clock.set_low();
            }

            // 2. Pulse Latch Clock (RCLK) to push the shifted bits to the output pins
            let _ = self.latch.set_high();
            let _ = self.latch.set_low();
        }
    }
}

