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
use embassy_time::{Duration, Timer};
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    gpio::Io,
    rng::Rng,
    timer::systimer::SystemTimer,
    timer::timg::TimerGroup,
};
use static_cell::StaticCell;

#[cfg(feature = "esp32s3")]
use esp_hal::gpio::{Level, Output};

#[cfg(feature = "esp32c6")]
use esp_hal::{rmt::Rmt, time::Rate};

use esp_now_blinky::Led;
use esp_wifi::{
    esp_now::{BROADCAST_ADDRESS, PeerInfo, EspNow},
    init,
};
use embassy_sync::{
    blocking_mutex::raw::NoopRawMutex,
    mutex::Mutex,
};

extern crate alloc;

// Macro to generate LED task for different chip types
macro_rules! create_led_task {
    ($led_type:ty) => {
        #[embassy_executor::task]
        async fn led_task(mut led: $led_type) {
            #[cfg(feature = "esp32s3")]
            info!("ESP32-S3 LED task started");
            #[cfg(feature = "esp32c6")]
            info!("ESP32-C6 LED task started");
            
            loop {
                led.cycle_color(50).await; // Medium brightness / ON
                led.cycle_color(0).await;  // Off
            }
        }
    };
}

// Generate LED task for the appropriate chip
#[cfg(feature = "esp32s3")]
create_led_task!(Led);

#[cfg(feature = "esp32c6")]
create_led_task!(Led<esp_hal::rmt::Channel<esp_hal::Blocking, 0>>);

// Embassy task for ESP-NOW receiving and peer management
#[embassy_executor::task]
async fn esp_now_receive_task(esp_now: &'static Mutex<NoopRawMutex, EspNow<'static>>) {
    info!("ESP-NOW receive task started");
    
    loop {
        // Check for received ESP-NOW messages
        let received = {
            let esp_now_guard = esp_now.lock().await;
            esp_now_guard.receive()
        };
        
        if let Some(r) = received {
            // Try to interpret the payload as UTF‑8 so we can print it nicely
            let payload = r.data();
            if let Ok(text) = core::str::from_utf8(payload) {
                info!("Received text \"{}\" from {:?}", text, r.info.src_address);
            } else {
                info!("Received bytes {:?} from {:?}", payload, r.info.src_address);
            }
            
            if r.info.dst_address == BROADCAST_ADDRESS {
                let mut esp_now_guard = esp_now.lock().await;
                if !esp_now_guard.peer_exists(&r.info.src_address) {
                    esp_now_guard
                        .add_peer(PeerInfo {
                            interface: esp_wifi::esp_now::EspNowWifiInterface::Sta,
                            peer_address: r.info.src_address,
                            lmk: None,
                            channel: None,
                            encrypt: false,
                        })
                        .expect("Failed to add ESP-NOW peer");
                }
                let status = esp_now_guard
                    .send(&r.info.src_address, b"Hello Peer")
                    .expect("Failed to send ESP-NOW message")
                    .wait();
                info!("Send hello to peer status: {:?}", status);
            }
        }
        
        // Small delay to prevent busy waiting
        Timer::after(Duration::from_millis(10)).await;
    }
}

// Embassy task for ESP-NOW periodic broadcasting
#[embassy_executor::task]
async fn esp_now_broadcast_task(esp_now: &'static Mutex<NoopRawMutex, EspNow<'static>>) {
    info!("ESP-NOW broadcast task started");
    
    loop {
        // Wait 5 seconds before sending next broadcast
        Timer::after(Duration::from_secs(5)).await;
        
        let status = {
            let mut esp_now_guard = esp_now.lock().await;
            esp_now_guard
                .send(&BROADCAST_ADDRESS, b"0123456789")
                .expect("Failed to send ESP-NOW broadcast")
                .wait()
        };
        info!("Send broadcast status: {:?}", status);
    }
}

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
    
    // Create unified LED API for different chips
    #[cfg(feature = "esp32s3")]
    let led = Led::new_gpio(Output::new(peripherals.GPIO21, Level::Low, Default::default()));
    
    #[cfg(feature = "esp32c6")]
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).expect("Failed to initialize RMT");
    
    #[cfg(feature = "esp32c6")]
    let led = Led::new_ws2812(rmt.channel0, peripherals.GPIO8);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    // Initialize Embassy with SystemTimer
    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    // Use static_cell to make esp_wifi_ctrl live for 'static
    static ESP_WIFI_CTRL: StaticCell<esp_wifi::EspWifiController> = StaticCell::new();
    let esp_wifi_ctrl = ESP_WIFI_CTRL.init(init(
        timg0.timer0,
        Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    ).expect("Failed to initialize ESP WiFi controller"));

    let wifi = peripherals.WIFI;
    let (mut controller, interfaces) = esp_wifi::wifi::new(esp_wifi_ctrl, wifi).expect("Failed to create WiFi controller and interfaces");
    controller.set_mode(esp_wifi::wifi::WifiMode::Sta).expect("Failed to set WiFi mode to Station");
    controller.start().expect("Failed to start WiFi controller");

    let esp_now = interfaces.esp_now;
    esp_now.set_channel(11).expect("Failed to set ESP-NOW channel");

    info!("esp-now version {:?}", esp_now.version().expect("Failed to get ESP-NOW version"));

    // Create a static mutex for sharing ESP-NOW between tasks
    static ESP_NOW_MUTEX: StaticCell<Mutex<NoopRawMutex, EspNow<'static>>> = StaticCell::new();
    let esp_now_static = ESP_NOW_MUTEX.init(Mutex::new(esp_now));

    // Spawn LED task
    #[cfg(feature = "esp32s3")]
    _spawner.spawn(led_task(led)).expect("Failed to spawn LED task");
    
    #[cfg(feature = "esp32c6")]
    _spawner.spawn(led_task(led)).expect("Failed to spawn LED task");

    // Spawn ESP-NOW tasks
    _spawner.spawn(esp_now_receive_task(esp_now_static)).expect("Failed to spawn ESP-NOW receive task");
    _spawner.spawn(esp_now_broadcast_task(esp_now_static)).expect("Failed to spawn ESP-NOW broadcast task");

    info!("All Embassy tasks spawned successfully!");
    
    // Main task can just wait or handle other coordination
    loop {
        Timer::after(Duration::from_secs(10)).await;
        info!("Main task heartbeat - Embassy tasks running...");
    }
}