# Pump — Fluid-O-Tech FG304XD0PT10000

## Overview

The stock Ulka vibration pump is replaced with this magnetically-coupled gear
pump. Gear pumps produce smoother, more controllable flow than vibratory pumps
and are responsive to analog speed commands, making closed-loop pressure
control possible.

---

## Specifications

| Parameter | Value |
|---|---|
| Model | FG304XD0PT10000 |
| Max pressure | 20 bar (software limited to 12 bar) |
| Speed input | 0–5 V DC (analog) |
| Supply voltage | 16.5–29 V DC (nominal 24 V) |
| Interface | 6-wire connector on integrated driver |
| Protection | Thermal, over/under-voltage, stall |

---

## Wire Colours

| Colour | Symbol | Function |
|---|---|---|
| Red | PWR_VCC | Power supply +24 V |
| Black | PWR_GND | Power ground (0 V) |
| Orange | SPEED_IN | Analog speed command 0–5 V |
| Brown | 0V | Speed command reference ground |
| Yellow | TACHO OUT | Speed feedback 0–5 V square wave |
| Green | DIRECTION | <2 V = CW, >4 V = CCW |

> ⚠️ Never connect PWR_GND (black) and 0V (brown) together.
> This will permanently damage the integrated driver board.

---

## Integrated Driver Protections

All fault conditions require speed input = 0 V before pump will restart.

| Protection | Threshold | Reset condition |
|---|---|---|
| Thermal cut-off | 120 °C internal | Temp < 110 °C AND speed = 0 |
| Over-voltage | 30 V | Voltage < 29 V AND speed = 0 |
| Under-voltage | 15.5 V | Voltage > 16.5 V AND speed = 0 |
| Motor stall | No rotation for 1 s | Speed = 0 |

> ⚠️ Steam cycle reaches ~145 °C. Keep pump body thermally isolated from
> boiler. Keep steaming duration short.

---

## Speed Command

| Parameter | Value |
|---|---|
| Target espresso flow | 1–3 mL/s |
| Pump displacement | 0.3 mL/rev |
| Max RPM needed | ~600–800 RPM |
| DAC voltage at 800 RPM | ~0.8 V |
| STM32 DAC range | 0–3.3 V |
| Op-amp required | No — STM32 DAC range is sufficient |

TACHO formula: Speed (RPM) = Frequency (Hz) × 60 / 32

TACHO output is 0–5 V — requires 10 kΩ / 27 kΩ voltage divider before STM32 GPIO.

---

## Integrated Driver Protections

All fault conditions require speed input = 0 V before pump will restart.

---

## Plumbing Notes

- Install overpressure relief valve immediately after pump outlet — mechanical backstop
- Install 10 µm pre-filter before pump inlet — required per datasheet
- Like-metal fittings only — stainless-to-stainless or brass-to-brass
- Tubing rated minimum 18 bar, food-safe at 150 °C

---

## Pressure Transducer Integration

XDB401 tee'd into the line between pump and boiler group head.
Feeds the pressure PID loop via ADS1015 on I2C.

| Parameter | Value |
|---|---|
| Range | 0–1.2 MPa (0–12 bar) |
| Output | 0.5–4.5 V ratiometric |
| Supply | 5–12 V |
| Read via | ADS1015 ADC — see docs/components/pressure.md |

---

## STM32 Connection

| Signal | STM32 pin | Notes |
|---|---|---|
| SPEED_IN | TBD (DAC) | 0–0.8 V for espresso range |
| TACHO OUT | TBD (GPIO) | Via 10 kΩ / 27 kΩ voltage divider |
| DIRECTION | GND via 10 kΩ | Permanent CW — pump side |
| FAULT | TBD (GPIO) | Active LOW — input with pull-up |

---

## Driver

| Item | Location |
|---|---|
| Rust driver crate | `drivers/fg304/src/lib.rs` |
| Interface traits | `embedded-hal DacChannel + InputPin` |
| Public functions | `set_speed_rpm()`, `read_tacho_rpm()`, `is_fault()` |

---

## Reference Images

- `hardware/images/pump_wiring_1.png`
- `hardware/images/pump_wiring_2.png`
