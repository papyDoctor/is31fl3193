//! IS31FL3193 RGB LED driver
//!
//! This crate provides a driver interface for the IS31FL3193 IC from LUMISSIL
//!
//! # Examples
//!
//! An example of implementation for the Raspberry Pi Pico can be found in examples folder

#![no_std]
mod command;
use command::Command;
use embedded_hal as hal;

/// All possible errors in this crate
#[derive(Clone, Debug)]
pub enum IS31FL3193Error<E> {
    /// I2C bus error
    I2c(E),
}

/// Hardware AD pin (pin 7) connection
pub enum ADPin {
    ToGND,
    ToVCC,
    ToSCL,
    ToSDA,
}

/// is31fl3193 driver.
#[derive(Copy, Clone, Debug)]
pub struct IS31FL3193<I2C> {
    i2c: I2C,
    address: u8,
}
impl<I2C, E> IS31FL3193<I2C>
where
    I2C: hal::blocking::i2c::Write<Error = E>,
{
    /// Create a new I2C interface with an address that
    /// corresponds to the AD pin hardware connection
    pub fn new(i2c: I2C, adpin: ADPin) -> IS31FL3193<I2C> {
        let address = match adpin {
            ADPin::ToGND => 0xD0 >> 1,
            ADPin::ToVCC => 0xD6 >> 1,
            ADPin::ToSCL => 0xD2 >> 1,
            ADPin::ToSDA => 0xD4 >> 1,
        };
        Self { i2c, address }
    }

    /// Consume the interface and return the underlying peripherial driver
    pub fn release(self) -> I2C {
        self.i2c
    }

    pub fn set_max_current(&mut self, intensity: Intensity) -> Result<(), IS31FL3193Error<E>> {
        self.send(Command::Current(intensity))
    }

    pub fn set_pwm(&mut self, led1: u8, led2: u8, led3: u8) -> Result<(), IS31FL3193Error<E>> {
        self.send(Command::PWMLed1(led1))?;
        self.send(Command::PWMLed2(led2))?;
        self.send(Command::PWMLed3(led3))?;
        self.send(Command::PWMUpdate)
    }

    pub fn set_pwm_led1(&mut self, led1: u8) -> Result<(), IS31FL3193Error<E>> {
        self.send(Command::PWMLed1(led1))?;
        self.send(Command::PWMUpdate)
    }

    pub fn set_pwm_led2(&mut self, led2: u8) -> Result<(), IS31FL3193Error<E>> {
        self.send(Command::PWMLed1(led2))?;
        self.send(Command::PWMUpdate)
    }

    pub fn set_pwm_led3(&mut self, led3: u8) -> Result<(), IS31FL3193Error<E>> {
        self.send(Command::PWMLed1(led3))?;
        self.send(Command::PWMUpdate)
    }

    pub fn set_timing(
        &mut self,
        led: Channel,
        t0: T0,
        t1: T1,
        t2: T2,
        t3: T3,
        t4: T4,
    ) -> Result<(), IS31FL3193Error<E>> {
        match led {
            Channel::Led1 => {
                self.send(Command::T0Led1(t0))?;
                self.send(Command::T1T2Led1(t1, t2))?;
                self.send(Command::T3T4Led1(t3, t4))?;
            }
            Channel::Led2 => {
                self.send(Command::T0Led2(t0))?;
                self.send(Command::T1T2Led2(t1, t2))?;
                self.send(Command::T3T4Led2(t3, t4))?;
            }
            Channel::Led3 => {
                self.send(Command::T0Led3(t0))?;
                self.send(Command::T1T2Led3(t1, t2))?;
                self.send(Command::T3T4Led3(t3, t4))?;
            }
        }
        self.send(Command::BreathTimingUpdate)
    }

    pub fn set_mode(&mut self, mode: Mode) -> Result<(), IS31FL3193Error<E>> {
        match mode {
            Mode::PWM => self.send(Command::LedMode(false)),
            Mode::Breath(breath_mode, marking) => {
                self.send(Command::LedMode(true))?;
                match breath_mode {
                    BreathingMode::Auto => match marking {
                        Marking::Off => {
                            self.send(Command::Breathing(false, false, false, Channel::Led1))
                        }
                        Marking::On(channel) => {
                            self.send(Command::Breathing(false, false, true, channel))
                        }
                    },
                    BreathingMode::OneCycle => match marking {
                        Marking::Off => {
                            self.send(Command::Breathing(true, true, false, Channel::Led1))
                        }
                        Marking::On(channel) => {
                            self.send(Command::Breathing(true, true, true, channel))
                        }
                    },
                    BreathingMode::RampToOn => match marking {
                        Marking::Off => {
                            self.send(Command::Breathing(true, false, false, Channel::Led1))
                        }
                        Marking::On(channel) => {
                            self.send(Command::Breathing(true, false, true, channel))
                        }
                    },
                }
            }
        }
    }

    pub fn shutdown(
        &mut self,
        enable_all_leds: bool,
        shutdown: bool,
    ) -> Result<(), IS31FL3193Error<E>> {
        self.send(Command::ShutDown(enable_all_leds, shutdown))
    }
    fn send(&mut self, cmd: Command) -> Result<(), IS31FL3193Error<E>> {
        self.i2c
            .write(self.address, &cmd.as_bytes())
            .map_err(IS31FL3193Error::I2c)
    }
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Intensity {
    MA5 = 2 << 2,
    MA10 = 1 << 2,
    MA17 = 4 << 2,
    MA30 = 3 << 2,
    MA42 = 0 << 2,
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone)]
pub enum Mode {
    PWM,
    Breath(BreathingMode, Marking),
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum BreathingMode {
    Auto,
    OneCycle,
    RampToOn,
}
#[derive(Copy, Clone)]
pub enum Marking {
    Off,
    On(Channel),
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Channel {
    Led1 = 0,
    Led2 = 1,
    Led3 = 2,
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum T0 {
    MS0 = 0,
    MS130 = 1,
    MS260 = 2,
    MS520 = 3,
    MS1040 = 4,
    MS2080 = 5,
    MS4160 = 6,
    MS8320 = 7,
    MS16640 = 8,
    MS33280 = 9,
    MS66560 = 10,
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum T1 {
    MS130 = 0 << 5,
    MS260 = 1 << 5,
    MS520 = 2 << 5,
    MS1040 = 3 << 5,
    MS2080 = 4 << 5,
    MS4160 = 5 << 5,
    MS8320 = 6 << 5,
    MS16640 = 7 << 5,
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum T2 {
    MS0 = 0 << 1,
    MS130 = 1 << 1,
    MS260 = 2 << 1,
    MS520 = 3 << 1,
    MS1040 = 4 << 1,
    MS2080 = 5 << 1,
    MS4160 = 6 << 1,
    MS8320 = 7 << 1,
    MS16640 = 8 << 1,
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum T3 {
    MS130 = 0,
    MS260 = 1 << 5,
    MS520 = 2 << 5,
    MS1040 = 3 << 5,
    MS2080 = 4 << 5,
    MS4160 = 5 << 5,
    MS8320 = 6 << 5,
    MS16640 = 7 << 5,
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum T4 {
    MS0 = 0 << 1,
    MS130 = 1 << 1,
    MS260 = 2 << 1,
    MS520 = 3 << 1,
    MS1040 = 4 << 1,
    MS2080 = 5 << 1,
    MS4160 = 6 << 1,
    MS8320 = 7 << 1,
    MS16640 = 8 << 1,
    MS33280 = 9 << 1,
    MS66560 = 10 << 1,
}
