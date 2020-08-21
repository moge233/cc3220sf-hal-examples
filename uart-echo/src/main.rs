#![no_std]
#![no_main]


#[macro_use(block)]
extern crate nb;
extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_hal::serial::{Read, Write};

use cc3220sf_hal::pac;
use cc3220sf_hal::gpio::*;
use cc3220sf_hal::prcm::mcu_init;
use cc3220sf_hal::uart::{Serial, BaudRate, Parity, DataLength, StopBits};


/************************************************** 
 * This trait was taken from 
 * https://stackoverflow.com/questions/49192205/
 * how-to-reset-all-array-elements
 **************************************************/
trait ArraySetAll {
    type Elem;
    fn set_all(&mut self, value: Self::Elem);
}

impl<T> ArraySetAll for [T] where T: Clone {
    type Elem = T;
    fn set_all(&mut self, value: T) {
        for e in self {
            *e = value.clone();
        }
    }
}


#[entry]
fn main() -> ! {

    mcu_init();

    let device_peripherals = pac::Peripherals::take().unwrap();

    let gpio0 = device_peripherals.GPIOA0.split();
    let tx = gpio0.gpio_01.into_alternate_function();
    let rx = gpio0.gpio_02.into_alternate_function();
    let serial = Serial::uarta0(device_peripherals.UARTA0, (tx, rx),
                                BaudRate(115200), DataLength::Eight,
                                Parity::None, StopBits::One);
    let (mut tx, mut rx) = serial.split();

    let mut buf: [u8; 80] = [0; 80];
    let mut index: usize = 0;

    loop { 
        // This will keep reading characters until the limit is reached
        // or a newline character is read
        // After this, the data that is read in will be echoed back out
        // and the buffer will be cleared
        while index < 80 {
            let ch = block!(rx.read()).unwrap();
            buf[index] = ch;
            index += 1;
            if ch == 0xD {
                break;
            }
        }

        for c in buf.iter() {
            if *c == 0xA {
                block!(tx.write(0xD)).unwrap();
            }
            block!(tx.write(*c)).unwrap();
        }

        index = 0;
        buf.set_all(0);
    }
}
