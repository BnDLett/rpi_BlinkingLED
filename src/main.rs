use std::thread;
use std::time::Duration;
use rppal::gpio::Gpio;
use shutdown_hooks;

const GPIO_LED: u8 = 29;

extern "C" fn exit() {
    let mut pin = Gpio::new().expect("REASON").get(GPIO_LED).expect("REASON")
        .into_output();
    pin.set_low();
}

fn main() {
    let mut pin = Gpio::new().expect("REASON").get(GPIO_LED).expect("REASON")
        .into_output();

    loop {
        pin.toggle();
        thread::sleep(Duration::from_millis(1000));
    }
}

// Before use, do understand that you need to disable your RPi's 3B+/Zero 2 control of the status
// LED. Not doing so will result in the LED being affected by other processes, such as
// reading/writing to the SD card or being static.
