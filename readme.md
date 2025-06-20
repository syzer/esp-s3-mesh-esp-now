# ESP-NOW Blinky

A simple ESP-NOW wireless communication example that works on both ESP32-S3 and ESP32-C6 chips. The project demonstrates:

- LED blinking every 500ms
- ESP-NOW broadcast messages every 5 seconds
- Automatic peer discovery and response
- Cross-chip compatibility (S3 ↔ C6)

## Hardware Support

- **ESP32-S3**: Uses GPIO21 for LED, Xtensa architecture
- **ESP32-C6**: Uses GPIO8 for LED, RISC-V architecture

## Prerequisites

1. Install the ESP Rust toolchain:
   ```bash
   cargo install espflash
   cargo install espup
   espup install
   ```

2. Install Just command runner (optional but recommended):
   ```bash
   cargo install just
   ```

## Quick Start

### Using Just Commands (Recommended)

```bash
# List all available commands
just --list

# Build and flash ESP32-S3
just flash-s3

# Build and flash ESP32-C6  
just flash-c6
```

### Manual Commands

**ESP32-S3:**
```bash
cargo run --features esp32s3 --target xtensa-esp32s3-none-elf
```

**ESP32-C6:**
```bash
cargo build --no-default-features --features esp32c6 --target riscv32imac-esp-espidf
espflash flash --monitor --chip esp32c6 --log-format defmt target/riscv32imac-esp-espidf/debug/esp_now_blinky
```

## Just Commands Reference

### Build Commands
- `just build-s3` - Build for ESP32-S3
- `just build-c6` - Build for ESP32-C6  
- `just build-all` - Build for both chips
- `just release-s3` - Release build for S3
- `just release-c6` - Release build for C6
- `just release-all` - Release builds for both

### Flash Commands
- `just flash-s3` - Flash debug build to S3
- `just flash-c6` - Flash debug build to C6
- `just flash-release-s3` - Flash release build to S3
- `just flash-release-c6` - Flash release build to C6
- `just flash-auto` - Auto-detect chip and flash

### Development Commands
- `just check-s3` - Quick syntax check for S3
- `just check-c6` - Quick syntax check for C6
- `just check-all` - Check both targets
- `just clean` - Clean build artifacts

### Debugging Commands
- `just monitor-s3` - Monitor serial output from S3
- `just monitor-c6` - Monitor serial output from C6
- `just info-s3` - Show S3 chip information
- `just info-c6` - Show C6 chip information
- `just erase-s3` - Erase S3 flash
- `just erase-c6` - Erase C6 flash

## How it Works

1. **LED Blinking**: The LED toggles every 500ms using a blocking delay
2. **ESP-NOW Broadcasting**: Every 5 seconds, sends a broadcast message "0123456789"
3. **Peer Discovery**: When receiving a broadcast, automatically adds the sender as a peer
4. **Peer Response**: Responds to known peers with "Hello Peer" message

## Troubleshooting

### Connection Issues
- Ensure USB cable supports data (not just power)
- Try putting ESP32 in download mode: Hold BOOT, press RESET, release BOOT
- Use `just info-s3` or `just info-c6` to verify chip connection

### Build Issues
- Run `just clean` if builds fail after switching targets
- Ensure you have the correct toolchain: `espup install`

### ESP-NOW Not Working
- Both devices must be on the same WiFi channel (channel 11 by default)
- Check serial output with `just monitor-s3` or `just monitor-c6`

## Project Structure

```
├── src/bin/main.rs      # Main application code
├── Cargo.toml           # Dependencies with feature flags
├── .cargo/config.toml   # Target configuration
├── justfile             # Build commands
└── readme.md           # This file
```
