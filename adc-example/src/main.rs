#![no_std]
#![no_main]


extern crate panic_halt;

use cortex_m_rt::entry;

#[macro_use(block)]
extern crate nb;

use cc3220sf_hal::prelude::*;
use cc3220sf_hal::gpio::*;
use cc3220sf_hal::prcm::mcu_init;
use cc3220sf_hal::adc;
use cc3220sf_hal::timer::*;
use cc3220sf_hal::pac;



#[entry]
fn main() -> ! {

    mcu_init();


    let device_peripherals = pac::Peripherals::take().unwrap();

    let mut timer = Timer::timera0(device_peripherals.TIMERA0,
                                   Ticks(0x00989680));

    let gpio0 = device_peripherals.GPIOA0.split();
    
    let mut adc_pin = gpio0.gpio_03.into_alternate_function();
    let mut adc0 = adc::Adc::adc(device_peripherals.ADC,
                                 &mut adc_pin);

    loop { 
        let adc_data = adc0.convert(&mut adc_pin);

        // Convert ADC data to volts
        // The CC3220SF ADC has an internal reference voltage of
        // 1.467 Volts = 1467000 uVolts
        // The ADC has 12 bits of resolution (12^2 - 1 = 4095)
        let _adc_data_f: f32 = adc_data as f32 * (1467000f32 / 4095f32);

        block!(timer.wait()).unwrap();
    }
}
