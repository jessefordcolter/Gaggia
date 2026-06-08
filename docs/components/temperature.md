# Temperature Sensing — MAX6675 + K-type Thermocouple

## Overview

The stock bi-metal thermostat is replaced with a K-type thermocouple inserted
into the boiler, read by a MAX6675 thermocouple amplifier. This gives
continuous temperature feedback to the PID boiler controller.

---

## MAX6675 Specifications

| Parameter | Value |
|---|---|
| Thermocouple type | K-type only |
| Temperature range | 0–1024 °C |
| Resolution | 0.25 °C (12-bit ADC) |
| Interface | SPI (read-only, CS + SCK + MISO) |
| Supply voltage | 3.0–5.5 V |
| Conversion time | ~170 ms |

---

## SPI Protocol

The MAX6675 is a read-only device. Each read transfers 16 bits:

| Bits | Content |
|---|---|
| Bit 15 | Always 0 (device identifier) |
| Bits 14–3 | 12-bit temperature value (°C × 4) |
| Bit 2 | Thermocouple open-circuit fault (1 = open) |
| Bit 1 | Device ID (always 0) |
| Bit 0 | Three-state (don't care) |

Temperature calculation: T (°C) = bits[14:3] × 0.25

---

## Boiler PID Control

The temperature reading drives the temperature PID in the runner crate.
The SSR (ZGT-40 DA) switches the boiler heater element.

| Setpoint | Temperature |
|---|---|
| Brew | 93 °C |
| Steam | 145 °C |

PID algorithm: time-proportional control — within a 1-second window the SSR
is energised for a fraction proportional to the PID output.

---

## Critical Notes

- MAX6675 requires at least 250 ms between reads — do not poll faster
- Open thermocouple (wire break) sets bit 2 HIGH — firmware must detect
  this and trigger FAULT — never run the boiler without a valid reading
- K-type thermocouple must make good thermal contact with the boiler body
- Use thermal paste and a properly sized thermocouple well

---

## STM32 Connection

| MAX6675 pin | STM32 pin | Notes |
|---|---|---|
| CS | TBD | Assigned when driver is written |
| SCK | TBD | SPI clock |
| MISO | TBD | SPI data out |
| VCC | 3.3 V | |
| GND | GND | |

---

## Driver

| Item | Location |
|---|---|
| Rust driver crate | `drivers/max6675/src/lib.rs` |
| Interface trait | `embedded-hal SpiDevice` |
| Public function | `read_temp_celsius()` |
