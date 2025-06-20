use defmt::debug;
use embassy_time::{Duration, Timer};

#[cfg(feature = "esp32c6")]
use esp_hal_smartled::{smart_led_buffer, SmartLedsAdapter};
#[cfg(feature = "esp32c6")]
use smart_leds::{RGB8, SmartLedsWrite};
#[cfg(feature = "esp32c6")]
use smart_leds::hsv::{Hsv, hsv2rgb};
#[cfg(feature = "esp32c6")]
use esp_hal::rmt::{TxChannel, TxChannelCreator};
#[cfg(feature = "esp32c6")]
use esp_hal::gpio::OutputPin;

#[cfg(feature = "esp32s3")]
use esp_hal::gpio::Output;

#[cfg(feature = "esp32s3")]
/// Unified LED API for ESP32-S3 (GPIO LED)
pub struct Led {
    gpio: Option<Output<'static>>,
}

#[cfg(feature = "esp32c6")]
/// Unified LED API for ESP32-C6 (WS2812 LED)  
pub struct Led<TX> 
where
    TX: TxChannel,
{
    ws2812: Option<SmartLedsAdapter<TX, 25>>,
    hue: u8,
}

#[cfg(feature = "esp32s3")]
impl Led {
    /// Create a new LED instance for ESP32-S3 (GPIO)
    pub fn new_gpio(pin: Output<'static>) -> Self {
        Self {
            gpio: Some(pin),
        }
    }
}

#[cfg(feature = "esp32c6")]
impl<TX> Led<TX> 
where
    TX: TxChannel,
{
    /// Create a new LED instance for ESP32-C6 (WS2812)
    pub fn new_ws2812<C, O>(channel: C, pin: O) -> Self
    where
        C: TxChannelCreator<'static, TX>,
        O: OutputPin + 'static,
    {
        let led_adapter = SmartLedsAdapter::new(channel, pin, smart_led_buffer!(1));
        Self {
            ws2812: Some(led_adapter),
            hue: 0,
        }
    }
}
#[cfg(feature = "esp32s3")]
impl Led {
    /// Set LED color/brightness. For GPIO LED, brightness > 0 = on, 0 = off.
    pub fn set_color(&mut self, brightness: u8) -> Result<(), ()> {
        if let Some(gpio) = &mut self.gpio {
            if brightness > 0 {
                gpio.set_high();
            } else {
                gpio.set_low();
            }
            return Ok(());
        }
        Err(())
    }

    /// Cycle LED color/state with logging
    pub async fn cycle_color(&mut self, brightness: u8) {
        if self.gpio.is_some() {
            if brightness > 0 {
                debug!("GPIO LED ON");
                let _ = self.set_color(brightness);
            } else {
                debug!("GPIO LED OFF");
                let _ = self.set_color(0);
            }
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[cfg(feature = "esp32c6")]
impl<TX> Led<TX> 
where
    TX: TxChannel,
{
    /// Set LED color/brightness using WS2812.
    pub fn set_color(&mut self, brightness: u8) -> Result<(), ()> {
        if let Some(ws2812) = &mut self.ws2812 {
            if brightness > 0 {
                // Generate HSV color with cycling hue
                let hsv = Hsv {
                    hue: self.hue,
                    sat: 255,  // Full saturation for vibrant colors
                    val: brightness,  // Use brightness as value
                };
                let rgb = hsv2rgb(hsv);
                
                // Advance hue for next call
                self.hue = self.hue.wrapping_add(15);
                
                match ws2812.write([rgb].iter().cloned()) {
                    Ok(_) => return Ok(()),
                    Err(_) => return Err(()),
                }
            } else {
                // Turn off LED
                let black = RGB8::new(0, 0, 0);
                match ws2812.write([black].iter().cloned()) {
                    Ok(_) => return Ok(()),
                    Err(_) => return Err(()),
                }
            }
        }
        Err(())
    }

    /// Cycle LED color/state with logging
    pub async fn cycle_color(&mut self, brightness: u8) {
        if self.ws2812.is_some() {
            if brightness > 0 {
                debug!("WS2812 LED - On");
                let _ = self.set_color(brightness);
            } else {
                debug!("WS2812 LED - Off");
                let _ = self.set_color(0);
            }
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}
