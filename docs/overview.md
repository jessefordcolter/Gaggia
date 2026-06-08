# GaggiaMod — Project Overview

## The Machine

**Gaggia Classic Pro (2019)**

- Model: SIN035R
- Supply: 120 V AC, 60 Hz, 1350 W
- Serial: TU902123086576
- Date of manufacture: June 2021
- Origin: Made in Italy

The Gaggia Classic Pro is a prosumer single-boiler espresso machine. The stock
machine uses a vibration pump and single on/off thermostat without user-readable
feedback. This project replaces its control system entirely while preserving
the machine's outward appearance ("stealth build").

---

## Project Goals

1. **Espresso experimentation platform** — precise, repeatable control of
   temperature, pressure, and dose weight for dialing in recipes.

2. **Embedded systems portfolio project** — production-quality Rust/Embassy
   firmware demonstrating driver design, PID control, sensor integration,
   state machines, and hardware-abstracted libraries reusable in future projects.

---

## Feature Priorities

| Priority | Feature | Notes |
|---|---|---|
| 1 | PID temperature control | Boiler SSR driven by PID loop |
| 1 | PID pressure control | Gear pump speed driven by pressure PID |
| 1 | Safety interlocks | Over-temp, over-pressure, dry-pump prevention |
| 1 | Stealth / stock-mode default | Default operation identical to stock machine |
| 2 | Drip-tray load-cell scale | Auto-stops at target output weight |
| 2 | Prevent brew after steam | Lockout until temp drops to brew range |
| 3 | Data logging | Wi-Fi shot logging via ESP8266 to Nexus 7 tablet |
| 3 | Auto-off by weight | Walk-away workflow |

---

## Physical Modifications

### Pump Replacement

The stock Ulka vibration pump is replaced with a **Fluid-O-Tech FG304XD0PT10000**
magnetically-coupled gear pump:

- Integrated 6-wire electronic motor driver
- 0–5 V analog speed control
- Up to 20 bar (software limited to 12 bar)
- Built-in thermal, over/under-voltage, and stall protection

An overpressure relief valve is installed after the pump outlet. A pressure
transducer is tee'd into the line between the pump and the boiler group head.

### Boiler Temperature

The stock bi-metal thermostat is replaced with a solid-state relay (SSR)
driven by the MCU. A K-type thermocouple in the boiler feeds the MAX6675
sensor, providing temperature feedback to the PID controller.

### Water Level

A VL53L0X time-of-flight sensor mounted above the reservoir measures distance
to the water surface. When the level drops below threshold, the pump is
disabled. A hardware float switch may be added as a redundant hard cut-off.

### Drip Tray Scale

Four load cells under the drip tray, read by two HX711 24-bit ADC boards,
provide output weight for auto-stop and shot logging.

---

## Stealth Build Philosophy

Default operation requires no configuration — pressing brew starts a shot at
9 bar / 93 °C. Monitoring and configuration are handled via an ESP8266 Wi-Fi
module bridging to the Nexus 7 tablet. Externally the machine looks and
operates identically to stock.

---

## Related Documents

- `docs/hardware/bom.md` — Bill of materials
- `docs/hardware/pinout.md` — STM32 pin assignments
- `docs/hardware/wiring.md` — Wiring notes and safety
- `docs/components/` — Per-component integration notes
- `docs/firmware/architecture.md` — Firmware design
- `ROADMAP.md` — Development phases
