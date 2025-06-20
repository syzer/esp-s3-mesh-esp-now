# ESP-NOW Blinky with Embassy

A modern async ESP-NOW wireless communication example using Embassy async runtime that works on both ESP32-S3 and ESP32-C6 chips. The project demonstrates:

- **Async LED blinking**: Non-blocking LED control using Embassy timers
- **ESP-NOW broadcasting**: Async message sending every 5 seconds  
- **Automatic peer discovery**: Non-blocking peer detection and response
- **Cross-chip compatibility**: S3 ↔ C6 communication
- **Embassy async runtime**: Efficient cooperative multitasking

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
cargo build --no-default-features --features esp32c6 --target riscv32imac-unknown-none-elf
espflash flash --monitor --chip esp32c6 --log-format defmt target/riscv32imac-unknown-none-elf/debug/esp_now_blinky
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
- `just validate` - Validate both builds work correctly
- `just clean` - Clean build artifacts

### Debugging Commands
- `just monitor-s3` - Monitor serial output from S3
- `just monitor-c6` - Monitor serial output from C6
- `just info-s3` - Show S3 chip information
- `just info-c6` - Show C6 chip information
- `just erase-s3` - Erase S3 flash
- `just erase-c6` - Erase C6 flash

## How it Works

1. **Embassy Async Runtime**: Uses SystemTimer for efficient non-blocking operations
2. **Async LED Blinking**: LED toggles every 500ms using `Timer::after()` without blocking
3. **ESP-NOW Broadcasting**: Every 5 seconds, sends a broadcast message "0123456789"
4. **Peer Discovery**: When receiving a broadcast, automatically adds the sender as a peer
5. **Peer Response**: Responds to known peers with "Hello Peer" message
6. **Cooperative Multitasking**: All operations run in a single main loop without blocking

## Testing Communication Between Devices

To test ESP-NOW communication between two devices:

1. **Flash two devices** (can be same or different chip types):
   ```bash
   # Device 1 (ESP32-S3)
   just flash-s3
   
   # Device 2 (ESP32-C6) 
   just flash-c6
   ```

2. **Monitor both devices** in separate terminals:
   ```bash
   # Terminal 1
   just monitor-s3
   
   # Terminal 2  
   just monitor-c6
   ```

3. **Expected behavior**:
   - Both LEDs should blink every 500ms
   - Every 5 seconds, each device broadcasts "0123456789"
   - When device A receives B's broadcast, it adds B as a peer and responds with "Hello Peer"
   - You should see messages like:
     ```
     esp-now version (1, 0)
     Send
     Send broadcast status: Success
     Received ReceivedData { data: [48, 49, 50, 51, 52, 53, 54, 55, 56, 57], info: ReceiveInfo { ... } }
     Send hello to peer status: Success
     ```

## Performance Notes

- **Build Speed**: Uses `-j 8` for parallel compilation (adjust based on your CPU cores)
- **ESP-NOW Range**: Typically 200-300 meters line-of-sight outdoors  
- **Channel**: Fixed to WiFi channel 11 for reliable communication
- **Memory Usage**: Heap allocator configured for 72KB (suitable for ESP-NOW operations)
- **Embassy Benefits**: 
  - Non-blocking operations improve responsiveness
  - Lower power consumption through efficient sleeping
  - Better resource utilization with cooperative scheduling
  - Easier concurrent programming with async/await patterns

## Embassy Async Architecture

This project uses Embassy's async runtime which provides several advantages:

- **Efficient Timing**: `Timer::after()` allows other tasks to run during delays
- **SystemTimer Integration**: Uses ESP32's built-in SystemTimer for precise timing
- **Zero-Cost Abstractions**: Async operations compile to efficient state machines
- **Single-threaded**: No need for complex locking or synchronization

## Build Optimization

To adjust parallel build jobs for your system:

1. **Edit justfile**: Change `-j 8` to `-j <your_cpu_cores>`
2. **Or set environment**: `export CARGO_BUILD_JOBS=<cores>`
3. **Check CPU cores**: `nproc` (Linux/macOS) or `echo $NUMBER_OF_PROCESSORS` (Windows)

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
├── validate.sh          # Validation script
└── readme.md           # This file
```

## Validation

Run the validation script to ensure both targets build correctly:

```bash
./validate.sh
```

This script will:
- Check prerequisites (just, cargo)
- Test ESP32-S3 build
- Test ESP32-C6 build
- Provide next steps for flashing and monitoring

## Features

- **Dual Target Support**: Build for both ESP32-S3 (Xtensa) and ESP32-C6 (RISC-V) from the same codebase
- **Feature Flags**: Clean separation between chip-specific configurations
- **Embassy Async**: Modern async/await runtime for efficient resource usage
- **Just Commands**: Streamlined build/flash workflow with parallel compilation
- **Cross-Architecture**: Demonstrates ESP-NOW communication between different ESP32 architectures
- **Non-blocking Operations**: All timing and I/O operations use async patterns