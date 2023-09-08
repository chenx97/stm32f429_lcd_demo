#![no_main]
#![no_std]

use embedded_hal::prelude::*;
use panic_halt as _;

use cortex_m_rt::entry;

use cortex_m::peripheral::Peripherals;
use stm32f4xx_hal::pac;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::rcc::RccExt;
use stm32f4xx_hal::time::Hertz;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        let gpiog = p.GPIOG.split();
        let gpioa = p.GPIOA.split();

        // (Re-)configure PG13 and PG14 (green LED & red LED) as outputs
        let mut green_led = gpiog.pg13.into_push_pull_output();
        let mut red_led = gpiog.pg14.into_push_pull_output();

        // Configure PA0 as input (user button)
        let button = gpioa.pa0.into_pull_down_input();

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 180 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(Hertz::MHz(180)).freeze();

        // Get delay provider
        let mut delay = cp.SYST.delay(&clocks);

        loop {
            // Toggle green LED constantly
            green_led.toggle();

            // Toggle red LED only if user button is pressed
            if button.is_high() == true {
                red_led.toggle();
            }

            // Delay a second
            delay.delay_ms(1000_u16);
        }
    }

    loop {}
}
