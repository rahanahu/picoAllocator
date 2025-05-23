use crate::LED_PIN;
use cortex_m::interrupt;
use embedded_hal::digital::{OutputPin, StatefulOutputPin};

pub fn led_on() {
    interrupt::free(|cs| {
        if let Some(pin) = LED_PIN.borrow(cs).borrow_mut().as_mut() {
            let _ = pin.set_high().unwrap();
        }
    });
}

pub fn led_off() {
    interrupt::free(|cs| {
        if let Some(pin) = LED_PIN.borrow(cs).borrow_mut().as_mut() {
            let _ = pin.set_low().unwrap();
        }
    });
}

pub fn led_toggle() {
    interrupt::free(|cs| {
        if let Some(pin) = LED_PIN.borrow(cs).borrow_mut().as_mut() {
            let _ = pin.toggle().unwrap();
        }
    });
}
