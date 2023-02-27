//! IS31FL3193 commands

use crate::{Channel, Intensity, T0, T1, T2, T3, T4};

// Shamefully taken from https://github.com/jamwaffles/ssd1306

/// Commands
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub(crate) enum Command {
    /// First bool: All channels enable/disable
    /// Second bool: false: normal operation, true: software shutdown mode
    ShutDown(bool, bool),
    /// Breathing control
    /// First bool: Ramping mode enable
    /// Second bool: Hold time, false:T2, true:T4
    /// Third bool: Breathing mark enable
    /// Channel: Channel selection
    Breathing(bool, bool, bool, Channel),
    /// RGB mode, PWM(false) or breathing(true)
    LedMode(bool),
    /// LED Maximum current setting
    Current(Intensity),
    /// PWM settings
    PWMLed1(u8),
    PWMLed2(u8),
    PWMLed3(u8),
    /// Update PWMs settings to Leds
    PWMUpdate,
    /// T0:Time for breathing
    T0Led1(T0),
    T0Led2(T0),
    T0Led3(T0),
    /// T1,T2: Time for breathing
    T1T2Led1(T1, T2),
    T1T2Led2(T1, T2),
    T1T2Led3(T1, T2),
    /// T3,T4: Time for breathing
    T3T4Led1(T3, T4),
    T3T4Led2(T3, T4),
    T3T4Led3(T3, T4),
    /// Update T0,T1,T2,T3,T4 breathing times to LEDs
    BreathTimingUpdate,
    /// Power on/off LEDS
    /// /// First bool: LED1, second: LED2, Third: LED3
    Power(bool, bool, bool),
    /// Reset to default (power on default)
    Reset,
}

impl Command {
    /// Send command to is31fl3193
    pub(crate) fn as_bytes(self) -> [u8; 2] {
        // Transform command into a fixed size array of 7 u8 and the real length for sending
        use Command::*;
        match self {
            ShutDown(en, ssd) => [0x00, (en as u8) << 5 | ssd as u8],
            Breathing(rm, ht, bme, css) => [
                0x01,
                (rm as u8) << 5 | (ht as u8) << 4 | (bme as u8) << 2 | (css as u8),
            ],
            LedMode(breath) => [0x02, (breath as u8) << 5],
            Current(intensity) => [0x03, intensity as u8],
            PWMLed1(pwm) => [0x04, pwm],
            PWMLed2(pwm) => [0x05, pwm],
            PWMLed3(pwm) => [0x06, pwm],
            PWMUpdate => [0x07, 0],
            T0Led1(t0) => [0x0A, (t0 as u8) << 4],
            T0Led2(t0) => [0x0B, (t0 as u8) << 4],
            T0Led3(t0) => [0x0C, (t0 as u8) << 4],
            T1T2Led1(t1, t2) => [0x10, (t1 as u8) | t2 as u8],
            T1T2Led2(t1, t2) => [0x11, (t1 as u8) | t2 as u8],
            T1T2Led3(t1, t2) => [0x12, (t1 as u8) | t2 as u8],
            T3T4Led1(t3, t4) => [0x16, (t3 as u8) | t4 as u8],
            T3T4Led2(t3, t4) => [0x17, (t3 as u8) | t4 as u8],
            T3T4Led3(t3, t4) => [0x18, (t3 as u8) | t4 as u8],
            BreathTimingUpdate => [0x1C, 0],
            Power(led1, led2, led3) => [0x1D, (led1 as u8) | (led2 as u8) << 1 | (led3 as u8) << 2],
            Reset => [0x2F, 0],
        }
    }
}
