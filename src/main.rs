#![no_std]
#![no_main]

use hal::timer::{self, Timer};
use nrf52840_dk_bsp::{Board, hal::{self, gpio::Level}, prelude::*};

use rtic::app;
// use hal::{gpio::Level, prelude::{InputPin, OutputPin}};

// pick a panicking behavior
use panic_rtt_target as _;
use rtt_target::rprintln; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support

// #[cfg(feature = "52840")]
// use nrf52840_hal as hal;

#[app(device = crate::hal::pac, peripherals = false)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {
        rtt_target::rtt_init_print!();
        rprintln!("Starting target.");
        
        // Cortex-M peripherals
        // let _core: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        // let _device: hal::pac::Peripherals = cx.device;
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        rprintln!("idle.");

        let mut nrf52 = Board::take().unwrap();

        let mut timer = Timer::new(nrf52.TIMER0);

        rprintln!("Blinky button demo starting");
        // Alternately flash the red and blue leds
        loop {
            nrf52.leds.led_2.enable();
            delay(&mut timer, 250_000); // 250ms
            nrf52.leds.led_2.disable();
            delay(&mut timer, 1_000_000); // 1s
        }

        // let p = hal::pac::Peripherals::take().unwrap();

        // let port0 = hal::gpio::p0::Parts::new(p.P0);
        // let button = port0.p0_11.into_pullup_input();
        // let mut led = port0.p0_31.into_push_pull_output(Level::Low);

        // rprintln!("Blinky button demo starting");
        // loop {
        //     if button.is_high().unwrap() {
        //         led.set_high().unwrap();
        //     } else {
        //         led.set_low().unwrap();
        //     }
        // }
    }
};

fn delay<T>(timer: &mut Timer<T>, cycles: u32)
where
    T: timer::Instance,
{
    timer.start(cycles);
    let _ = nb::block!(timer.wait());
}
