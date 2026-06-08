# GaggiaMod — Development Roadmap

Phases are ordered by safety-criticality and dependency.
Complete each phase before starting the next.
Each phase produces a working, testable result.

**Hardware target:** STM32F411CEU6 Blackpill throughout.
**Firmware:** Rust + Embassy.

---

## Phase 0 — Foundation *(current phase)*

**Goal:** Clean workspace, working build environment, all docs updated.

- [x] Create Cargo workspace structure
- [x] Configure .cargo/config.toml for STM32F411
- [x] Write memory.x from verified datasheet values
- [x] Update all documentation for Rust/Embassy/STM32
- [x] Initialize git repository and push to GitHub
- [ ] Install VSCode extensions and configure rust-analyzer
- [ ] Scaffold all driver crates (Cargo.toml + empty lib.rs)
- [ ] Scaffold runner crate (Cargo.toml + empty main.rs)
- [ ] Confirm workspace builds with cargo build

**Milestone:** Clean Rust workspace builds without errors.

---

## Phase 1 — FG304 Pump Driver

**Goal:** Write and test the pump driver in isolation before touching the machine.

**Deliverables:**

- Implement `drivers/fg304/src/lib.rs`
- DAC speed control (0–0.8 V for espresso range)
- TACHO feedback reading (RPM = Hz × 60 / 32)
- Fault pin monitoring
- Full unit test suite

**Testing:**

- [ ] Driver builds against embedded-hal traits
- [ ] Unit tests pass on host machine
- [ ] DAC outputs correct voltage at target RPM on bench
- [ ] TACHO reads correct RPM from running pump
- [ ] Fault pin detection works correctly
- [ ] Pump stops and requires speed = 0 to restart after fault

**Milestone:** Pump driver verified on bench. Safe to integrate.

---

## Phase 2 — MAX6675 Temperature Driver

**Goal:** Read boiler temperature reliably.

**Deliverables:**

- Implement `drivers/max6675/src/lib.rs`
- SPI read — 16-bit protocol, temperature = bits[14:3] × 0.25
- Open-circuit fault detection (bit 2)
- Minimum 250 ms between reads enforced
- Full unit test suite

**Testing:**

- [ ] Driver builds against embedded-hal traits
- [ ] Unit tests pass on host machine
- [ ] Reads room temperature correctly on bench (~20 °C)
- [ ] Open-circuit fault detected correctly
- [ ] 250 ms minimum read interval enforced

**Milestone:** Temperature driver verified on bench.

---

## Phase 3 — ADS1015 + XDB401 Pressure Driver

**Goal:** Read brew pressure reliably via I2C ADC.

**Deliverables:**

- Implement `drivers/ads1015/src/lib.rs`
- Implement `drivers/xdb401/src/lib.rs` (pressure conversion)
- I2C address 0x48, PGA ±4.096 V
- Conversion: 0.5–4.5 V → 0–12 bar (6 mbar per count)
- Full unit test suite

**Testing:**

- [ ] ADS1015 detected on I2C bus at address 0x48
- [ ] Reads 0 bar correctly at rest
- [ ] Reads known pressure correctly on bench
- [ ] Unit tests pass on host machine

**Milestone:** Pressure driver verified on bench.

---

## Phase 4 — Temperature PID + SSR Control

**Goal:** Replace stock thermostat with PID boiler control.

**Prerequisites:** Phase 2 complete.

**Deliverables:**

- Temperature PID implementation in runner
- SSR control (time-proportional, 1-second window)
- Safety interlock: over-temperature → SSR off → FAULT
- Safety interlock: thermocouple open-circuit → FAULT
- Brew setpoint: 93 °C / Steam setpoint: 145 °C

**Testing:**

- [ ] Machine heats to 93 °C and holds within ±2 °C
- [ ] Over-temperature FAULT triggers correctly
- [ ] Thermocouple open-circuit FAULT triggers correctly
- [ ] SSR switching verified with multimeter

**Milestone:** Machine makes espresso with PID temperature control.
Pump is still stock Ulka at this phase.

---

## Phase 5 — Pump Integration + Pressure PID

**Goal:** Replace Ulka pump with FG304, implement closed-loop pressure control.

**Prerequisites:** Phases 1, 3, 4 complete.

**Deliverables:**

- Physical: install FG304, pressure transducer, overpressure valve, pre-filter, tubing
- Pressure PID targeting 9 bar during brewing
- Pump fault detection → FAULT state
- Pump restart-after-fault sequence (speed = 0 required)
- Low-water stub interlock (digital sensor placeholder until Phase 6)

**Testing:**

- [ ] Pressure reads 0 bar at rest
- [ ] Pump responds to speed commands
- [ ] Pressure PID holds 9 bar within ±0.5 bar
- [ ] Over-pressure FAULT triggers correctly
- [ ] Pump fault detection works

**Milestone:** Machine makes espresso with temperature AND pressure PID.

---

## Phase 6 — Water Level Safety

**Goal:** Dry-pump prevention via VL53L0X ToF sensor.

**Prerequisites:** Phase 5 complete.

**Deliverables:**

- Implement `drivers/vl53l0x/src/lib.rs`
- Calibrate mounting height for full and empty thresholds
- Set SAFETY_LOW_WATER_MM constant
- Low-water → pump disabled → FAULT
- Optional: hardware float switch as hard cut-off

**Testing:**

- [ ] Sensor reads correct distance at various fill levels
- [ ] Low-water FAULT triggers before reservoir is fully empty
- [ ] Machine refuses to brew with low-water condition
- [ ] Fault clears after reservoir is refilled

**Milestone:** Machine safe to leave unattended — no dry-pump risk.

---

## Phase 7 — HX711 Scale + Dose-by-Weight

**Goal:** Auto-stop shot at target output weight.

**Prerequisites:** Phase 5 complete.

**Deliverables:**

- Implement `drivers/hx711/src/lib.rs`
- Tare and calibration routine
- Auto-stop pump when target weight reached (default 36 g)
- Calibration constants stored in STM32 flash

**Testing:**

- [ ] Scale reads 0 g tared correctly
- [ ] Scale reads known weight within ±0.5 g
- [ ] Shot auto-stops at target weight

**Milestone:** Fully automated dose-by-weight workflow.

---

## Phase 8 — Wi-Fi Telemetry

**Goal:** Shot data logging to Nexus 7 tablet via ESP8266.

**Prerequisites:** Phase 7 complete.

**Deliverables:**

- UART bridge to ESP8266
- Shot log: timestamp, weight, peak pressure, average temp, brew time
- JSON format over Wi-Fi to Nexus 7 tablet
- Nexus 7 web dashboard displays live and historical shot data

**Milestone:** Full scientific logging for recipe development.

---

## Phase 9 — Hardening & Installation

**Goal:** Production-ready firmware permanently installed in machine.

- [ ] Comprehensive unit tests for all drivers
- [ ] PCB design — replace breadboard with custom control board
- [ ] Cable management and permanent installation
- [ ] Calibration procedure documented for each sensor
- [ ] Stress test — 30+ shot session
- [ ] All safety interlocks verified under real conditions

**Milestone:** Production-ready stealth build complete.

---

## Quick-Start (Phase 0 → Phase 1)

1. Open workspace: `code ~/projects/Gaggia`
2. Scaffold driver crates
3. Run `cargo build` — confirm 0 errors
4. Connect ST-Link debugger
5. Start with `drivers/fg304` — first driver to write
