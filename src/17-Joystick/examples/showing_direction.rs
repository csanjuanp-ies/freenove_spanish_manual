#![no_main]
#![no_std]

use cortex_m::prelude::_embedded_hal_adc_OneShot;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin};
use microbit::Board;
use microbit::display::blocking::Display;
use nrf52833_hal::{Saadc, Timer};
use nrf52833_hal::saadc::SaadcConfig;
use crate::utils::{show_arrow, Direction};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Inicialización .... ");
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);


    let mut saadc_config = SaadcConfig::default();
    saadc_config.resolution = nrf52833_hal::saadc::Resolution::_10BIT;  // En este caso muestreamos a 10 bits
    let mut adc = Saadc::new(board.ADC, saadc_config);
    let mut pin_x = board.edge.e02;
    let mut pin_y = board.edge.e01;
    let mut pin_z = board.edge.e00.into_pullup_input();

    rprintln!("Inicialización completada");
    loop {
        let val_x = adc.read(&mut pin_x).unwrap();
        let val_y = adc.read(&mut pin_y).unwrap();
        let val_z = pin_z.is_low().unwrap() as bool;

        let direction = match (val_x, val_y, val_z) {
            (_, _, true) => { Direction::CERO },
            (x, y, _) if x < 450 && y < 450 => Direction::SUPERIOR_IZQUIERDA,
            (x, y, _) if x > 650 && y < 450 => Direction::SUPERIOR_DERECHA,
            (x, y, _) if x < 450 && y > 650 => Direction::INFERIOR_IZQUIERDA,
            (x, y, _) if x > 650 && y > 650 => Direction::INFERIOR_DERECHA,
            (x, _, _) if x < 450 => Direction::IZQUIERDA,
            (x, _, _) if x > 650 => Direction::DERECHA,
            (_, y, _) if y < 450 => Direction::ARRIBA,
            (_, y, _) if y > 650 => Direction::ABAJO,
            _ => Direction::VACIA,
        };

        show_arrow(&mut display, &mut timer, Direction::VACIA, 100_u32);
        show_arrow(&mut display, &mut timer, direction, 200_u32);
        timer.delay_ms(200_u32);
    }
}



mod utils{
    use microbit::display::blocking::Display;
    use nrf52833_hal::{Timer};

    pub enum Direction {
        VACIA =0,
        ABAJO =1,
        ARRIBA =2,
        IZQUIERDA =3,
        DERECHA =4,
        INFERIOR_IZQUIERDA =5,
        SUPERIOR_IZQUIERDA =6,
        INFERIOR_DERECHA =7,
        SUPERIOR_DERECHA =8,
        CERO = 9
    }
    pub fn show_arrow(
        display: &mut Display,
        timer: &mut Timer<nrf52833_pac::TIMER0>,
        direction: Direction,
        deleay_ms: u32
    ) {
        const DIGITS: [[[u8; 5]; 5]; 10] = [
            [  // vacía
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            [  // derecha
                [0, 0, 1, 0, 0],
                [0, 1, 0, 0, 0],
                [1, 1, 1, 1, 1],
                [0, 1, 0, 0, 0],
                [0, 0, 1, 0, 0],
            ],
            [  // izquierda
                [0, 0, 1, 0, 0],
                [0, 0, 0, 1, 0],
                [1, 1, 1, 1, 1],
                [0, 0, 0, 1, 0],
                [0, 0, 1, 0, 0],
            ],
            [  // arriba
                [0, 0, 1, 0, 0],
                [0, 1, 1, 1, 0],
                [1, 0, 1, 0, 1],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
            ],
            [  // abajo
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
                [1, 0, 1, 0, 1],
                [0, 1, 1, 1, 0],
                [0, 0, 1, 0, 0],
            ],
            [  // superior izquierda
                [1, 1, 1, 1, 0],
                [1, 1, 0, 0, 0],
                [1, 0, 1, 0, 0],
                [1, 0, 0, 1, 0],
                [0, 0, 0, 0, 1],
            ],
            [  // superior derecha
                [0, 1, 1, 1, 1],
                [0, 0, 0, 1, 1],
                [0, 0, 1, 0, 1],
                [0, 1, 0, 0, 1],
                [1, 0, 0, 0, 0],
            ],
            [  // inferior izquierda
                [0, 0, 0, 0, 1],
                [1, 0, 0, 1, 0],
                [1, 0, 1, 0, 0],
                [1, 1, 0, 0, 0],
                [1, 1, 1, 1, 0],
            ],
            [  // inferior derecha
                [1, 0, 0, 0, 0],
                [0, 1, 0, 0, 1],
                [0, 0, 1, 0, 1],
                [0, 0, 0, 1, 1],
                [0, 1, 1, 1, 1],
            ],
            [  // cero
                [1, 1, 1, 1, 1],
                [1, 0, 0, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
            ]

        ];
        display.show(timer, DIGITS[direction as usize], deleay_ms);
    }

}
