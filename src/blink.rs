#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

// Halt when the program panics.
extern crate panic_halt;

// Includes.
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32l4xx_hal as hal;
use hal::prelude::*;
// use cortex_m_semihosting::{hprintln};

#[entry]
fn main() -> ! {
    // Set up SysTick peripheral.
    let cmp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cmp.SYST;
    syst.set_clock_source( SystClkSource::Core );
    // ~1ms period; STM32F3 resets to 8MHz internal oscillator.
    syst.set_reload( 4_000_000 );
    syst.enable_counter();

    // Set up GPIO pin E8 (LED #4)
    let p = stm32l4xx_hal::pac::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut gpioe = p.GPIOB.split( &mut rcc.ahb2 );
    let mut ld4 = gpioe.pb3.into_push_pull_output( &mut gpioe.moder, &mut gpioe.otyper );

    loop {
        while !syst.has_wrapped() {};
        let _ = ld4.set_high();

        // hprintln!("Hello, world!").unwrap();

        while !syst.has_wrapped() {};
        let _ = ld4.set_low();
    }
}