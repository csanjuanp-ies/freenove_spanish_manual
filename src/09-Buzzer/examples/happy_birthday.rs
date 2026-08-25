#![no_main]
#![no_std]

use embedded_hal::delay::DelayNs;
use nrf52833_hal::{pwm, Timer};
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::Board;
use panic_halt as _;
use nrf52833_hal::gpio::{Level, Pin};
use nrf52833_hal::pwm::{Channel, Pwm};
use nrf52833_hal::time::Hertz;

#[entry]
fn main() -> ! {
    // Definición de las frecuencias de las notas en Hz
    const C4: u32 = 440;
    const D4: u32 = 550;
    const E4: u32 = 660;
    const F4: u32 = 770;
    const G4: u32 = 880;
    const A4: u32 = 990;
    const A_S4: u32 = 1045;
    const B4: u32 = 1100;
    const C5: u32 = 1210;
    const MULT:u32 = 125;
    const PAUSA: u32 = 0; // Silencio entre notas

    const MELODIA: [(u32, u32); 28] = [
        // Frase 1: Happy birthday to you
        (C4, 3*MULT),   // N(c4:3)
        (C4, 1*MULT),   // N(c:1)
        (D4, 4*MULT),   // N(d:4)
        (C4, 4*MULT),   // N(c:4)
        (F4, 4*MULT),   // N(f) -> asume igual que la anterior (:4)
        (E4, 8*MULT),  // N(e:8)
        (PAUSA, 6*MULT), // N(pausa)
        // Frase 2: Happy birthday to you
        (C4, 3*MULT),   // N(c:3)
        (C4, 1*MULT),   // N(c:1)
        (D4, 4*MULT),   // N(d:4)
        (C4, 4*MULT),   // N(c:4)
        (G4, 4*MULT),   // N(g) -> asume igual que la anterior (:4)
        (F4, 8*MULT),  // N(f:8)asume igual que la anterior (:4)
        (PAUSA, 8*MULT), // N(pausa)
        // Frase 3: Happy birthday dear ...
        (C4, 3*MULT),   // N(c:3)
        (C4, 1*MULT),   // N(c:1)
        (C5, 4*MULT),   // N(c5:4)
        (A4, 4*MULT),   // N(a) -> asume igual que la anterior (:4)
        (F4, 4*MULT),   // N(f)  -> asume igual que la anterior (:4)
        (E4, 8*MULT),   // N(e:8)
        (D4, 8*MULT),  // N(d)  -> asume igual que la anterior (:8)
        (PAUSA, 8*MULT), // N(pausa)
        // Frase 4:
        (A_S4, 3*MULT),  // N(a#_:3)
        (A_S4, 1*MULT),  // N(a#:1)
        (A4, 4*MULT),   // N(a:4)
        (F4, 4*MULT),   // N(f)  -> asume igual que la anterior (:4)
        (G4, 4*MULT),   // N(g)  -> asume igual que la anterior (:4)
        (F4, 8*MULT),  // N(f:8)
    ];

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut pin = board.edge.e00.into_push_pull_output(Level::Low);

    pin.set_high().unwrap();
    timer.delay_ms(1000_u32);

    let pwm = Pwm::new(board.PWM0);
    // Asignar el Pin P0.02 - RING0 - P0 al canal 0
    pwm.set_output_pin(Channel::C0, Pin::from(pin)).set_period(Hertz(440));

    pwm.enable();
    for &(frecuencia, duracion) in MELODIA.iter() {
        if frecuencia == PAUSA {
            pwm.set_duty_on_common(0);
        } else {  // tocar la nota
            pwm.set_period(Hertz(frecuencia));
            let max_duty = pwm.max_duty();
            pwm.set_duty_on_common( max_duty / 2);
        }
        // Mantener la nota sonando
        timer.delay_ms(duracion);

        // Pequeño silencio de 30 ms entre notas para que no se mezclen
        pwm.set_duty_on_common(0);
        timer.delay_ms(30);
    }

    // Apagar el altavoz al terminar la canción
    pwm.set_duty_on_common(0);
    pwm.disable();

    loop {
        cortex_m::asm::wfi();
    }
}