#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

use hal::prelude::*;
use nrf52840_hal as hal;
use rubble::beacon::Beacon;
use rubble::link::{ad_structure::AdStructure, MIN_PDU_BUF};
use rubble_nrf5x::radio::{BleRadio, PacketBuffer};
use rubble_nrf5x::utils::get_device_address;

static mut TX_BUF: PacketBuffer = [0; MIN_PDU_BUF];
static mut RX_BUF: PacketBuffer = [0; MIN_PDU_BUF];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let nrf52 = hal::pac::Peripherals::take().unwrap();

    // Enable external HiFreq oscillator. This is needed for Bluetooth
    // to work.
    hal::clocks::Clocks::new(nrf52.CLOCK).enable_ext_hfosc();

    // Determine device address
    let device_address = get_device_address();

    // Rubble currently requires an RX buffer even though the radio is only used as a TX-only beacon.
    let mut radio = BleRadio::new(nrf52.RADIO, &nrf52.FICR, unsafe { &mut TX_BUF }, unsafe {
        &mut RX_BUF
    });

    let beacon = Beacon::new(
        device_address,
        &[AdStructure::CompleteLocalName("Rusty Beacon (nRF52)")],
    )
    .unwrap();

    // Set up a timer
    let mut timer = hal::timer::Timer::new(nrf52.TIMER0);

    loop {
        rprintln!("broadcast beacon");
        beacon.broadcast(&mut radio);
        timer.delay_ms(1000_u16);
    }
}
