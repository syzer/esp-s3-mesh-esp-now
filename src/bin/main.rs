#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

//! # ESP-NOW Blinky with Embassy Async
//!
//! This example demonstrates ESP-NOW wireless communication with async LED blinking
//! using Embassy async runtime on both ESP32-S3 and ESP32-C6 chips.
//!
//! ## Features:
//! - Embassy async runtime with SystemTimer for efficient timing
//! - Non-blocking LED control and ESP-NOW communication
//! - Cross-architecture support (Xtensa S3 and RISC-V C6)
//! - Automatic peer discovery and response
//! - Cooperative multitasking in single main loop
//!
//! ## Hardware:
//! - ESP32-S3: LED on GPIO21
//! - ESP32-C6: LED on GPIO8

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    gpio::{Io, Level, Output},
    rng::Rng,
    timer::systimer::SystemTimer,
    timer::timg::TimerGroup,
};
use esp_wifi::{
    esp_now::{BROADCAST_ADDRESS, PeerInfo},
    init,
};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Initialize GPIO for LED
    let _io = Io::new(peripherals.IO_MUX);
    
    // Different GPIO pins for different chips
    #[cfg(feature = "esp32s3")]
    let mut led = Output::new(peripherals.GPIO21, Level::Low, Default::default());
    
    #[cfg(feature = "esp32c6")]
    let mut led = Output::new(peripherals.GPIO8, Level::Low, Default::default());

    esp_alloc::heap_allocator!(size: 72 * 1024);

    // Initialize Embassy with SystemTimer
    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    let esp_wifi_ctrl = init(
        timg0.timer0,
        Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    let wifi = peripherals.WIFI;
    let (mut controller, interfaces) = esp_wifi::wifi::new(&esp_wifi_ctrl, wifi).unwrap();
    controller.set_mode(esp_wifi::wifi::WifiMode::Sta).unwrap();
    controller.start().unwrap();

    let mut esp_now = interfaces.esp_now;

    info!("esp-now version {:?}", esp_now.version().unwrap());

    esp_now.set_channel(11).unwrap();

    let mut next_send_time = Instant::now() + Duration::from_secs(5);

    // Combined LED blinking and ESP-NOW communication loop
    loop {
        // LED blinking with async timing
        info!("LED ON - Hello world!");
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        
        info!("LED OFF");
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;

        // Check for received ESP-NOW messages
        if let Some(r) = esp_now.receive() {
            info!("Received {:?}", r);

            if r.info.dst_address == BROADCAST_ADDRESS {
                if !esp_now.peer_exists(&r.info.src_address) {
                    esp_now
                        .add_peer(PeerInfo {
                            interface: esp_wifi::esp_now::EspNowWifiInterface::Sta,
                            peer_address: r.info.src_address,
                            lmk: None,
                            channel: None,
                            encrypt: false,
                        })
                        .unwrap();
                }
                let status = esp_now
                    .send(&r.info.src_address, b"Hello Peer")
                    .unwrap()
                    .wait();
                info!("Send hello to peer status: {:?}", status);
            }
        }

        // Send broadcast message every 5 seconds
        if Instant::now() >= next_send_time {
            next_send_time = Instant::now() + Duration::from_secs(5);
            info!("Send broadcast message");
            let status = esp_now
                .send(&BROADCAST_ADDRESS, b"0123456789")
                .unwrap()
                .wait();
            info!("Send broadcast status: {:?}", status);
        }
    }
}