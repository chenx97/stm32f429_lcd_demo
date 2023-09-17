#![no_main]
#![no_std]

mod image;
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use display_interface_spi::SPIInterface;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_hal::digital::v2::OutputPin;
use ili9341::{DisplaySize240x320, Ili9341};
use panic_halt as _;
use stm32f4xx_hal::gpio::{Pull, Speed};
use stm32f4xx_hal::pac;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::rcc::RccExt;
use stm32f4xx_hal::spi::{Mode, NoMiso, Phase, Polarity};
use stm32f4xx_hal::time::Hertz;

const DISCOVERY_SPI5_AF: u8 = 5;

#[derive(Default)]
struct DummyOutput;

impl OutputPin for DummyOutput {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    type Error = ();
}

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        let gpiod = p.GPIOD.split();
        let gpiof = p.GPIOF.split();

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 180 MHz (i.e. the maximum) and freeze it
        let sysclk = &mut rcc.cfgr.sysclk(Hertz::MHz(180)).freeze();

        // Get delay provider
        let systimer = cp.SYST;
        let mut delay = systimer.delay(&sysclk);

        let sck = gpiof
            .pf7
            .into_alternate::<DISCOVERY_SPI5_AF>()
            .internal_resistor(Pull::None)
            .speed(Speed::Medium);
        let miso = NoMiso::default();
        let mosi = gpiof
            .pf9
            .into_alternate::<DISCOVERY_SPI5_AF>()
            .internal_resistor(Pull::Up)
            .speed(Speed::Medium);

        let mode = Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        };
        let spi5 = p.SPI5.spi((sck, miso, mosi), mode, 42.MHz(), &sysclk);
        let lcd_wrx = gpiod.pd13.into_push_pull_output();
        let mut lcd_ncs = p.GPIOC.split().pc2.into_push_pull_output();
        lcd_ncs.set_low();
        lcd_ncs.set_high();
        let iface = SPIInterface::new(spi5, lcd_wrx, lcd_ncs);
        let reset = DummyOutput::default();
        let mut lcd = Ili9341::new(
            iface,
            reset,
            &mut delay,
            ili9341::Orientation::PortraitFlipped,
            DisplaySize240x320,
        )
        .unwrap();
        lcd.clear(Rgb565::WHITE).unwrap();
        embedded_graphics::image::ImageRawLE::<Rgb565>::new(&image::IMAGE, 240)
            .draw(&mut lcd)
            .unwrap();
    }

    loop {}
}
