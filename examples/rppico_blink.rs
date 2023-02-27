#![no_std]
#![no_main]
use fugit::RateExtU32;
use hal::pac;

use defmt_rtt as _;
use panic_probe as _;

use is31fl3193::*;
use rp2040_hal as hal;
use rp_pico::{
    entry,
    hal::{Clock, I2C},
    XOSC_CRYSTAL_FREQ,
};

#[entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    // Soft-reset does not release the hardware spinlocks
    // Release them now to avoid a deadlock after debug or watchdog reset
    unsafe {
        rp_pico::hal::sio::spinlock_reset();
    }
    let mut resets = pac.RESETS;

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut resets,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut resets);
    // ----------------------------------------------------------------
    // ----I2C0---- configuration
    // ----------------------------------------------------------------
    let i2c0 = I2C::i2c0(
        pac.I2C0,
        pins.gpio20.into_mode(),
        pins.gpio21.into_mode(),
        400.kHz(),
        &mut resets,
        clocks.peripheral_clock.freq(),
    );

    let mut led_drv = IS31FL3193::new(i2c0, ADPin::ToGND);

    //
    // 1, Example PWM
    //
    // Before exiting from shutdown, set the maximum current allowed
    led_drv.set_max_current(Intensity::MA5).unwrap();
    // Set the mode we want (PWM is default)
    led_drv.set_mode(Mode::PWM).unwrap();
    // Set PWM values
    led_drv.set_pwm(100, 0, 75).unwrap();
    // Enable LEDS and go out of shutdown mode
    led_drv.shutdown(true, false).unwrap();

    // //
    // // 2, Example Breath
    // //
    // // Before exiting from shutdown, set the maximum current allowed
    // led_drv.set_max_current(Intensity::MA5).unwrap();
    // // PWM setting is also necessary for breathing mode
    // led_drv.set_pwm(50, 50, 50).unwrap();
    // // Set the mode we want (PWM is default)
    // led_drv
    //     .set_mode(Mode::Breath(BreathingMode::Auto, Marking::Off))
    //     .unwrap();
    // // Enable LEDS and go out of shutdown mode
    // led_drv.shutdown(true, false).unwrap();

    // //
    // // 3, Example Breath with differents timing for each color
    // //
    // // Before exiting from shutdown, set the maximum current allowed
    // led_drv.set_max_current(Intensity::MA5).unwrap();
    // // Set custom timings
    // led_drv
    //     .set_timing(
    //         Channel::Led1,
    //         T0::MS260,
    //         T1::MS130,
    //         T2::MS130,
    //         T3::MS260,
    //         T4::MS130,
    //     )
    //     .unwrap();
    // led_drv
    //     .set_timing(
    //         Channel::Led2,
    //         T0::MS130,
    //         T1::MS130,
    //         T2::MS130,
    //         T3::MS520,
    //         T4::MS130,
    //     )
    //     .unwrap();
    // led_drv
    //     .set_timing(
    //         Channel::Led3,
    //         T0::MS130,
    //         T1::MS1040,
    //         T2::MS130,
    //         T3::MS130,
    //         T4::MS130,
    //     )
    //     .unwrap();
    // // PWM setting is also necessary for breathing mode
    // led_drv.set_pwm(50, 50, 50).unwrap();
    // // Set the mode we want (PWM is default)
    // led_drv
    //     .set_mode(Mode::Breath(BreathingMode::Auto, Marking::Off))
    //     .unwrap();
    // // Enable LEDS and go out of shutdown mode
    // led_drv.shutdown(true, false).unwrap();

    defmt::info!("Done, entering loop");
    loop {}
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
