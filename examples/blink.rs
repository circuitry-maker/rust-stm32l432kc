//! Blinks an LED

#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32l4xx_hal as hal;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::rt::entry;
use crate::rt::ExceptionFrame;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    // Try a different clock configuration
    let clocks = rcc.cfgr.hclk(4.MHz()).freeze(&mut flash.acr, &mut pwr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut timer = Delay::new(cp.SYST, clocks);
    loop {
        timer.delay_ms(1000_u32);
        led.set_high();

        timer.delay_ms(1000_u32);
        led.set_low();
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
