#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// use embassy_nrf::config::{Config, HfclkSource};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use nrf52840_dk_bsp::hal;
use rubble::beacon::Beacon;
use rubble::link::{ad_structure::AdStructure, MIN_PDU_BUF};
use rubble_nrf5x::radio::{BleRadio, PacketBuffer};
use rubble_nrf5x::utils::get_device_address;
// pick a panicking behavior
use {defmt_rtt as _, panic_probe as _};

static mut TX_BUF: PacketBuffer = [0; MIN_PDU_BUF];
static mut RX_BUF: PacketBuffer = [0; MIN_PDU_BUF];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // let mut config = Config::default();
    // config.hfclk_source = HfclkSource::ExternalXtal;
    // embassy_nrf::init(config);

    let nrf52 = hal::pac::Peripherals::take().unwrap();

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

    loop {
        beacon.broadcast(&mut radio);
        Timer::after(Duration::from_secs(1)).await;
    }
}
