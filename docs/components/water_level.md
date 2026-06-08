# Water Level — VL53L0X Time-of-Flight Sensor

## Overview

The VL53L0X measures the distance from the sensor (mounted above the water
reservoir lid) to the water surface below. When the water level drops and the
measured distance exceeds the threshold, the pump is disabled to prevent dry
running.

---

## Sensor Specifications

| Parameter | Value |
|---|---|
| Model | VL53L0X (ST Microelectronics) |
| Measurement range | Up to 2 m |
| Ranging accuracy | ±3% typical |
| Interface | I2C (default address 0x29) |
| Supply voltage | 2.6–3.5 V (3.3 V from MCU regulator) |
| Field of view | 25° |

---

## Mounting Notes

- Sensor mounted above reservoir lid, firing down through an aperture onto the water surface
- Mounting height must place the usable range (empty to full) within the sensor reliable range
- Measure distance at full and minimum safe level to determine SAFETY_LOW_WATER_MM threshold
- Calibration values recorded in engineering notebook at time of installation

---

## Redundancy

A hardware float switch is recommended as a hard electrical cut-off for the
pump. The TOF sensor provides a soft warning margin before the float switch triggers.

| Event | Action |
|---|---|
| TOF distance > SAFETY_LOW_WATER_MM | Firmware disables pump (soft) |
| Float switch opens | Hardware breaks pump power circuit (hard) |

---

## I2C Bus

Shares I2C bus with ADS1015. No address conflict.

| Device | Address | Notes |
|---|---|---|
| VL53L0X | 0x29 | Default; changeable via XSHUT pin |
| ADS1015 | 0x48 | ADDR pin = GND |

4.7 kΩ pull-up resistors required on SDA and SCL.

STM32 I2C pins: TBD — assigned when driver is written.

---

## Driver

| Item | Location |
|---|---|
| Rust driver crate | `drivers/vl53l0x/src/lib.rs` |
| Interface trait | `embedded-hal I2c` |
| Public function | `water_level_read_mm()` |
