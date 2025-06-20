#![no_std]
#![no_main]

use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    main,
    rng::Rng,
    time::{self, Duration},
    timer::timg::TimerGroup,
};
use esp_hal::gpio::{Level, Output};
use esp_println::println;
use esp_wifi::{
    esp_now::{BROADCAST_ADDRESS, PeerInfo},
    init,
};
use embedded_hal::delay::DelayNs;
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);
    
    // Different GPIO pins for different chips
    #[cfg(feature = "esp32s3")]
    let mut led = Output::new(peripherals.GPIO21, Level::Low, Default::default());
    
    #[cfg(feature = "esp32c6")]
    let mut led = Output::new(peripherals.GPIO8, Level::Low, Default::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let mut delay = Delay::new();

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

    println!("esp-now version {}", esp_now.version().unwrap());

    esp_now.set_channel(11).unwrap();

    let mut next_send_time = time::Instant::now() + Duration::from_secs(5);
    loop {
        led.set_high();
        delay.delay_ms(500u32);
        led.set_low();
        delay.delay_ms(500u32);

        let r = esp_now.receive();
        if let Some(r) = r {
            println!("Received {:?}", r);

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
                println!("Send hello to peer status: {:?}", status);
            }
        }

        if time::Instant::now() >= next_send_time {
            next_send_time = time::Instant::now() + Duration::from_secs(5);
            println!("Send");
            let status = esp_now
                .send(&BROADCAST_ADDRESS, b"0123456789")
                .unwrap()
                .wait();
            println!("Send broadcast status: {:?}", status)
        }
    }
}