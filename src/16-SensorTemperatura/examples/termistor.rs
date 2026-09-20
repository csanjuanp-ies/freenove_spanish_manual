#![no_main]
#![no_std]

use cortex_m::prelude::_embedded_hal_adc_OneShot;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use microbit::Board;
use microbit::display::blocking::Display;
use nrf52833_hal::saadc::SaadcConfig;
use nrf52833_hal::{Saadc, Timer};
use libm::logf;


#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Inicialización .... ");
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut saadc_config = SaadcConfig::default();
    saadc_config.resolution = nrf52833_hal::saadc::Resolution::_10BIT;  // En este caso muestreamos a 10 bits
    let mut adc = Saadc::new(board.ADC, saadc_config);
    let mut analog_pin = board.edge.e00;
    let mut display = Display::new(board.display_pins);


    rprintln!("Inicialización completada");
    loop {
        let valor_crudo = adc.read(&mut analog_pin).unwrap();
        rprintln!("Temperatura crudo: {}", valor_crudo);
        let v = valor_crudo as f32 * 3.3/1023.0;
        rprintln!("Temperatura voltaje: {}", v);
        let rt = v/(3.3-v)/10.0;
        rprintln!("Temperatura resistencia: {}", rt);
        let rt_log =  logf(1.0 + ((rt/10.0)/3950.0));
        rprintln!("Temperatura log: {}", rt_log);
        let temp_c = (
            1.0/
                (
                    1.0/
                    (273.15+25.0)  + (rt_log)
                )
        ) - 273.15;
        rprintln!("Temperatura calculada: {}", temp_c);

        show_integer_con_desplazamiento(&mut display, &mut timer, temp_c as i32, 200);
        timer.delay_ms(200_u32);
    }
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