# ESP32 ESP-NOW Blinky Project
# Use `just --list` to see available commands

# Default recipe - build for ESP32-S3
default: build-s3

# List all available commands
list:
    @just --list

# Build for ESP32-S3
build-s3:
    cargo build --features esp32s3 --target xtensa-esp32s3-none-elf

# Build for ESP32-C6
build-c6:
    cargo build --no-default-features --features esp32c6 --target riscv32imac-esp-espidf

# Build for both chips
build-all: build-s3 build-c6

# Flash to ESP32-S3
flash-s3:
    cargo run --features esp32s3 --target xtensa-esp32s3-none-elf

# Flash to ESP32-C6
flash-c6:
    espflash flash --monitor --chip esp32c6 --log-format defmt target/riscv32imac-esp-espidf/debug/esp_now_blinky

# Clean build artifacts
clean:
    cargo clean

# Check code for both targets
check-s3:
    cargo check --features esp32s3 --target xtensa-esp32s3-none-elf

check-c6:
    cargo check --no-default-features --features esp32c6 --target riscv32imac-esp-espidf

check-all: check-s3 check-c6

# Release builds
release-s3:
    cargo build --release --features esp32s3 --target xtensa-esp32s3-none-elf

release-c6:
    cargo build --release --no-default-features --features esp32c6 --target riscv32imac-esp-espidf

release-all: release-s3 release-c6

# Flash release builds
flash-release-s3:
    espflash flash --monitor --chip esp32s3 --log-format defmt target/xtensa-esp32s3-none-elf/release/esp_now_blinky

flash-release-c6:
    espflash flash --monitor --chip esp32c6 --log-format defmt target/riscv32imac-esp-espidf/release/esp_now_blinky

# Monitor serial output for each chip
monitor-s3:
    espflash monitor --chip esp32s3

monitor-c6:
    espflash monitor --chip esp32c6

# Auto-detect and flash (useful when you have only one device connected)
flash-auto:
    espflash flash --monitor --log-format defmt target/xtensa-esp32s3-none-elf/debug/esp_now_blinky

# Show chip info
info-s3:
    espflash board-info --chip esp32s3

info-c6:
    espflash board-info --chip esp32c6

# Erase flash
erase-s3:
    espflash erase-flash --chip esp32s3

erase-c6:
    espflash erase-flash --chip esp32c6
