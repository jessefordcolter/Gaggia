# STM32F411CEU6 Blackpill — Pin Assignments

> Pin assignments are defined per driver as each is written and tested.
> This document is updated when a pin is physically committed.
> Never assign pins speculatively — only assign when wiring hardware.

---

## Assigned Pins

| STM32 Pin | Signal | Driver | Notes |
|-----------|--------|--------|-------|
| — | — | — | No pins assigned yet |

---

## Bus Assignments

| Bus | Peripheral | Devices | Status |
|-----|-----------|---------|--------|
| SPI | TBD | MAX6675 | Assigned when max6675 driver is written |
| I2C | TBD | ADS1015, VL53L0X | Assigned when ads1015 driver is written |
| DAC | TBD | FG304 SPEED_IN | Assigned when fg304 driver is written |
| UART | TBD | ESP8266 | Assigned when Wi-Fi bridge is written |

---

## I2C Device Addresses

| Device | Address | Notes |
|--------|---------|-------|
| ADS1015 | 0x48 | ADDR pin tied to GND |
| VL53L0X | 0x29 | Default address |

No address conflict. 4.7 kΩ pull-up resistors required on SDA and SCL.

---

## Power Budget (estimated)

| Device | Current |
|--------|---------|
| MAX6675 | ~10 mA |
| VL53L0X | ~10 mA |
| ADS1015 | ~1 mA |
| HX711 × 2 | ~2 mA each |
| ESP8266 | ~80 mA (Wi-Fi active) |
| **Total** | **~105 mA** |

STM32F411 onboard 3.3 V LDO — check current rating before powering all
sensors simultaneously. ESP8266 may need its own 3.3 V regulator.

---

## Reference

- Blackpill pin layout: `hardware/datasheets/STM32/STM32F4x1 v2.0+ Pin Layout.pdf`
- Full peripheral mapping: STM32F411CEU6 Reference Manual, Section 3
