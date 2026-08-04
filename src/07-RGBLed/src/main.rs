#![no_main]
#![no_std]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use microbit::Board;
use nrf52833_hal::gpio::{Level, Pin};
use nrf52833_hal::pwm::{Channel, Pwm};
use panic_halt as _;

#[entry]
fn main() -> ! {
    use color::{write_analog,ColorIntensidad};
    const RED: Channel = Channel::C0;
    const GREEN: Channel = Channel::C1;
    const BLUE: Channel = Channel::C2;


    let board = Board::take().unwrap();
    let led_azul = board.edge.e00.into_push_pull_output(Level::Low);
    let led_verde = board.edge.e01.into_push_pull_output(Level::Low);
    let led_rojo = board.edge.e02.into_push_pull_output(Level::Low);
    let pwm = Pwm::new(board.PWM0);

    pwm.set_output_pin(RED, Pin::from(led_rojo));
    pwm.set_output_pin(GREEN, Pin::from(led_verde));
    pwm.set_output_pin(BLUE, Pin::from(led_azul));


    write_analog(&pwm, RED, ColorIntensidad::Encendido);
    write_analog(&pwm, GREEN, ColorIntensidad::Encendido);
    write_analog(&pwm, BLUE, ColorIntensidad::Encendido);

    loop {
        nop()
    }
}

mod color {
    pub use nrf52833_hal::pwm::{Channel, Pwm};
    use nrf52833_pac::PWM0;

    pub enum ColorIntensidad {
        Apagado = 0,
        UnCuarto = 8000,
        Medio = 16000,
        TresCuartos = 24000,
        Encendido = 32000
    }

    impl Into<u16> for ColorIntensidad {
        fn into(self) -> u16 {
            match self {
                ColorIntensidad::Apagado => 0,
                ColorIntensidad::UnCuarto => 8000,
                ColorIntensidad::Medio => 16000,
                ColorIntensidad::TresCuartos => 24000,
                ColorIntensidad::Encendido => 32000,
            }
        }
    }

    pub fn write_analog(pwm: &Pwm<PWM0>, canal: Channel, value: ColorIntensidad) {
        pwm.set_duty_on(canal, value.into());
    }
}
