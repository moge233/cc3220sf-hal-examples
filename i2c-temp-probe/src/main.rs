#![no_std]
#![no_main]


#[macro_use(block)]
extern crate nb;
extern crate panic_halt;
extern crate cc3220sf_hal;

use cortex_m_rt::entry;

use cc3220sf_hal::pac;
use cc3220sf_hal::gpio::*;
use cc3220sf_hal::prcm::mcu_init;
use cc3220sf_hal::prelude::*;
use cc3220sf_hal::i2c::*;
use cc3220sf_hal::uart::*;
use cc3220sf_hal::timer::*;


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

    // Take CC3220SF peripherals from the PAC
    let device_peripherals = pac::Peripherals::take().unwrap();

    // GPIO ports
    let gpio0 = device_peripherals.GPIOA0.split();
    let gpio1 = device_peripherals.GPIOA1.split();

    // Timer setup
    let mut timer = Timer::timera0(device_peripherals.TIMERA0,
                                   Ticks(0x02625A00));

    // I2C setup
    let sda = gpio1.gpio_11.into_alternate_function();
    let scl = gpio1.gpio_10.into_alternate_function();

    let mut i2c = I2c::i2ca0(device_peripherals.I2CA0, (scl, sda),
                             I2cSpeed::Standard);

    let slave_addr = 0x66 as u8;
    let write_buffer : [u8; 1] = ['r' as u8];
    let mut read_buffer: [u8; 40] = [0; 40];

    // UART setup
    let tx = gpio0.gpio_01.into_alternate_function();
    let rx = gpio0.gpio_02.into_alternate_function();
    let serial = Serial::uarta0(device_peripherals.UARTA0, (tx, rx),
                                BaudRate(115200), DataLength::Eight,
                                Parity::None, StopBits::One);
    let (mut tx, _) = serial.split(); // Ignore the RX pin

    loop { 
        // Read the temperature
        // The Atlas temperature probe used has a processing
        // delay of 300ms so we use the timer to allow it to process
        i2c.write(slave_addr, &write_buffer).unwrap();
        block!(timer.wait()).unwrap();
        i2c.read(slave_addr, &mut read_buffer).unwrap();
        block!(timer.wait()).unwrap();
      
        // The first byte is a status byte
        // '1' = successful read
        if read_buffer[0] == ('1' as u8) {
            let data: &[u8] = &read_buffer[1..40];
            for c in data.iter() {
                block!(tx.write(*c)).unwrap();
            }
        }

        // Newline and carriage return
        block!(tx.write(0xA)).unwrap();
        block!(tx.write(0xD)).unwrap();

        read_buffer.set_all(0);
    }
}
