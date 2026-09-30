#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use i2c_character_display::{CharacterDisplayPCF8574T, LcdDisplayType};
use microbit::{
    hal::{twim, Timer},
    Board,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut delay_i2c = Timer::new(board.TIMER0);
    let mut delay_app = Timer::new(board.TIMER1);
    let numbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    // Configurar los pines de la MB2 destinados al I2C (P19 y P20) externos
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_external.into(), twim::Frequency::K100) };
    // Inicializar la pantalla LCD vía PCF8574 (Por defecto busca la dirección estándar 0x27 o 0x3F)
    let mut lcd = CharacterDisplayPCF8574T::new(i2c, LcdDisplayType::Lcd16x2, &mut delay_i2c);

    if let Err(_e) = lcd.init() {
        panic!("Error initializing LCD:");
    }

    let _ = lcd.backlight(true);
    let _ = lcd.clear();
    let _ =lcd.home();
    let _ = delay_app.delay_ms(1000);
    let _ = lcd.print("Hola Microbit V2");
    let _ = lcd.set_cursor(0, 1);
    let _ = delay_app.delay_ms(1000);
    let _ = lcd.print("Rust Embedded!");

    loop {
        for number in 0..=9 {
            let _ = lcd.set_cursor(15, 1);
            let _ = lcd.print(numbers[number as usize]);
            let _ = delay_app.delay_ms(1000);
        }
    }
}
