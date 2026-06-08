# Scales — HX711 Load Cell ADC

## Overview

Four load cells are mounted beneath the drip tray, connected in two pairs to
two HX711 24-bit ADC boards. The combined reading gives the weight of the cup
and shot output, enabling dose-by-weight auto-stop.

---

## Hardware Configuration

| Board | Load Cells | STM32 pins |
|---|---|---|
| HX711 A | Cells 1 & 2 (front pair) | TBD — assigned when driver is written |
| HX711 B | Cells 3 & 4 (rear pair) | TBD — assigned when driver is written |

The two boards are read independently and their gram readings are summed for
the final weight.

---

## HX711 Specifications

| Parameter | Value |
|---|---|
| ADC resolution | 24 bits |
| Gain (channel A) | 64 or 128 (set by extra SCK pulses) |
| Gain (channel B) | 32 |
| Conversion rate | 10 SPS or 80 SPS (set by RATE pin) |
| Interface | Bit-bang GPIO (DAT + CLK per board) |
| Supply voltage | 2.6–5.5 V |

Use channel A, gain 128 (25 SCK pulses per read) for best sensitivity.

---

## Calibration Procedure

Perform at installation. Record all values in engineering notebook.

1. Tare with empty drip tray and cup in place
2. Place known calibration mass (e.g. 100 g) on tray
3. Read raw ADC value
4. Calculate: scale_factor = (raw - tare_offset) / known_weight_grams
5. Store scale factor in STM32 flash — assigned when driver is written

Repeat for both HX711 boards independently.

---

## Use in Shot Control

| Use | Detail |
|---|---|
| Auto-stop | Pump stops when target output weight reached |
| Default target | 36 g (1:2 brew ratio) |
| Wi-Fi logging | Shot weight sent to Nexus 7 tablet via ESP8266 |

---

## Driver

| Item | Location |
|---|---|
| Rust driver crate | `drivers/hx711/src/lib.rs` |
| Interface traits | `embedded-hal InputPin + OutputPin` |
| Public functions | `tare()`, `read_grams()`, `set_scale()` |
