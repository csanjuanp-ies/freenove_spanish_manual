#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::Board;
use microbit::display::blocking::Display;
use nrf52833_hal::{Timer};


#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Inicialización .... ");
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let temp_regs = board.TEMP;


    rprintln!("Inicialización completada");
    loop {
        let raw_value = read_temperature_raw(&temp_regs);
        // El hardware de nRF52 devuelve el valor multiplicado por 4 (en pasos de 0.25 °C)
        let temperature: f32 = raw_value as f32 / 4.0;
        rprintln!("Temperatura: {}°C", temperature);
        show_integer_con_desplazamiento(&mut display, &mut timer, temperature as i32, 250);
        timer.delay_ms(200_u32);
    }
}

fn read_temperature_raw(temp_regs: &nrf52833_pac::TEMP) -> i32 {
    temp_regs.tasks_start.write(|w| unsafe { w.bits(1) });
    while temp_regs.events_datardy.read().bits() == 0 {
        core::hint::spin_loop();
    }
    temp_regs.events_datardy.write(|w| unsafe { w.bits(0) });
    temp_regs.temp.read().bits() as i32
}

fn show_integer_con_desplazamiento(
    display: &mut Display,
    timer: &mut Timer<nrf52833_pac::TIMER0>,
    value: i32,
    deleay_ms: u32
) {
    const DIGITS: [[[u8; 5]; 5]; 11] = [
        [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ],
        [
            [0, 0, 1, 0, 0],
            [0, 1, 1, 0, 0],
            [1, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [1, 1, 1, 1, 1],
        ],
        [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [1, 1, 1, 1, 1],
        ],
        [
            [1, 1, 1, 1, 0],
            [0, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
        ],
        [
            [0, 0, 1, 1, 0],
            [0, 1, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 1, 0],
        ],
        [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
        ],
        [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ],
        [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ],
        [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ],
        [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ],
        [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
    ];

    let negative = value < 0;
    let mut number = value.unsigned_abs();
    let mut digits = [0usize; 10];
    let mut digit_count = 0;

    if number == 0 {
        digits[0] = 0;
        digit_count = 1;
    } else {
        while number > 0 {
            digits[digit_count] = (number % 10) as usize;
            number /= 10;
            digit_count += 1;
        }
    }

    let character_count = digit_count + usize::from(negative);
    let scroll_length = character_count * 6 + 5;

    for offset in 0..scroll_length {
        let mut frame = [[0u8; 5]; 5];

        for screen_column in 0..5 {
            let position = offset + screen_column;
            let character_index = position / 6;
            let column_in_character = position % 6;

            if character_index >= character_count || column_in_character >= 5 {
                continue;
            }

            let digit = if negative && character_index == 0 {
                10
            } else {
                let index = character_index - usize::from(negative);
                digits[digit_count - 1 - index]
            };

            for row in 0..5 {
                frame[row][screen_column] = DIGITS[digit][row][column_in_character];
            }
        }

        display.show(timer, frame, deleay_ms);
    }
}