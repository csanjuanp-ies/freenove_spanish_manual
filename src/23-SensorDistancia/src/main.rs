#![no_main]
#![no_std]
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use microbit::hal::gpio::{Level};
use microbit::hal::Timer;
use microbit::Board;
use embedded_hal::digital::{InputPin};
use nrf52833_hal::twim;
use i2c_character_display::{CharacterDisplayPCF8574T, LcdDisplayType};


#[entry]
fn main() -> ! {
 rtt_init_print!();
    rprintln!("Inicializando HC-SR04 en MB2...");

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut delay_i2c = Timer::new(board.TIMER1);

    // Configurar los pines de la MB2 destinados al I2C (P19 y P20) externos
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_external.into(), twim::Frequency::K100) };
    // Inicializar la pantalla LCD vía PCF8574 (Por defecto busca la dirección estándar 0x27 o 0x3F)
    let mut lcd = CharacterDisplayPCF8574T::new(i2c, LcdDisplayType::Lcd16x2, &mut delay_i2c);

    if let Err(_e) = lcd.init() {
        panic!("Error initializing LCD:");
    }

    // Pines: Pin 1 (P0.03) para Trig y Pin 0 (P0.02) para Echo
    let mut trig = board.edge.e01.into_push_pull_output(Level::Low);
    let mut echo = board.edge.e00.into_floating_input();

    loop {
        // 1. Enviar pulso de disparo (Trig) de 10 microsegundos
        rprintln!("Fase 1: Enviando pulso de disparo");
        trig.set_low().expect("TODO: panic message");
        timer.delay_us(2_u32);

        rprintln!("echo is high: {}", echo.is_high().unwrap());
        trig.set_high().expect("TODO: panic message");
        timer.delay_us(15u32); // 15 microsegundos para asegurar el pulso
        trig.set_low().expect("TODO: panic message");
        rprintln!("echo is high: {}", echo.is_high().unwrap());

        // 2. Esperar a que el pin Echo pase a HIGH
        rprintln!("Fase 2: Esperando");
        while echo.is_low().unwrap() {}

        // 3. Medir la duración del pulso en HIGH
        // Reiniciamos o tomamos una marca de tiempo en microsegundos
        rprintln!("Fase 3: Midiendo");
        let start_time = timer.read();
        while echo.is_high().unwrap(){}
        let end_time = timer.read();
        let duration = end_time.wrapping_sub(start_time);

        rprintln!("Fase 4: calculando distancia {} us", duration);
        // 4. Calcular distancia en centímetros
        // Velocidad del sonido = 343 m/s => 0.0343 cm/s
        // Distancia = (Tiempo * 0.0343) / 2
        let distance_cm = (duration as f32 * 0.0343) / 2.0;

        // LCD Display
        let _ = lcd.backlight(true);
        let _ = lcd.clear();
        let _ =lcd.home();
        if distance_cm > 400.0 || distance_cm < 2.0 {
            rprintln!("Fuera de rango o error de lectura");
            let _ = lcd.print("Fuera de rango");
        } else {
            rprintln!("Distancia: {} cm", distance_cm);
            let _ = lcd.print("En distancia");
        }

        // Esperar al menos 60us antes de la próxima medición (recomendado)
        // timer.delay_us(60u32);
        timer.delay_ms(1000);

    }
}
