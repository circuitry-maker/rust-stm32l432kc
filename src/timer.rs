#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32l4xx_hal as hal;

use crate::hal::interrupt;
use crate::hal::prelude::*;
use crate::hal::timer::{Event, Timer};
use crate::rt::entry;
use crate::rt::ExceptionFrame;
use cortex_m::peripheral::NVIC;

use cortex_m_semihosting::{hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Starting").unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);
    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);
    // let clocks = rcc.cfgr
    //     .sysclk(80.mhz())
    //     .pclk1(32.mhz())
    //     .pclk2(32.mhz())
    //     .freeze(&mut flash.acr, &mut pwr);

    unsafe { NVIC::unmask(hal::stm32::Interrupt::TIM7) };
    let mut timer = Timer::tim7(dp.TIM7, 1.mhz(), clocks, &mut rcc.apb1r1);
    timer.listen(Event::TimeOut);

    loop {
        continue;
    }
}

#[interrupt]
fn TIM7() {
    hprintln!("Hello, world!").unwrap();
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}