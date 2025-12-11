#![no_std]
#![no_main]

use panic_probe as _;
use defmt_rtt as _;
use nrf_mpsl as _;

use cortex_m_rt::entry;
use embassy_nrf::{self as nrf, twim, bind_interrupts};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

bind_interrupts!(struct TwimIrqs {
    TWISPI0 => twim::InterruptHandler<nrf::peripherals::TWISPI0>;
});

#[entry]
fn main() -> ! {
    let p = nrf::init(Default::default());

    // ---------------- I2C pins ----------------
    let scl = p.P1_14;
    let sda = p.P1_15;

    // ---------------- TWIM/I2C ----------------
    let mut config = twim::Config::default();
    config.frequency = twim::Frequency::K400;

    static mut DMA_BUF: [u8; 128] = [0; 128];

    let i2c = unsafe {
        twim::Twim::new(
            p.TWISPI0,
            TwimIrqs,
            sda,
            scl,
            config,
            &mut DMA_BUF,
        )
    };

    // ---------------- SSD1306 display ----------------
    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();

    display.init().unwrap();

    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("RMK Display OK", Point::new(0, 0), style)
        .draw(&mut display)
        .unwrap();
    display.flush().unwrap();

    // пока просто спим, клавиатурную логику добавим позже
    loop {
        cortex_m::asm::wfi();
    }
}

// ------------------- ЗАГЛУШКИ ПРЕРЫВАНИЙ -------------------
// nrf52840-pac/embassy ожидают эти символы из вектора прерываний.
// Раз мы не запускаем сейчас BLE/SoftDevice, можно дать пустые обработчики.

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn CLOCK_POWER() {
    // пусто или можно поставить breakpoint
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn UARTE0() {
    // пусто
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn TWISPI1() {
    // пусто
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn AAR_CCM() {
    // пусто
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn EGU0_SWI0() {
    // пусто
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn EGU1_SWI1() {
    // пусто
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn EGU2_SWI2() {
    // пусто
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn EGU3_SWI3() {
    // пусто
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn EGU4_SWI4() {
    // пусто
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn EGU5_SWI5() {
    // пусто
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn SPI2() {
    // пусто
}
