# Firmware Architecture

## Language & Toolchain

All firmware is written in **Rust**, compiled with the ARM GCC toolchain targeting the **STM32F411CEU6 Blackpill**.

Choosing Rust:

- Memory safety guaranteed at compile time
- Embassy async framework replaces traditional RTOS
- `embedded-hal` trait system enforces chip-agnostic driver design
- Each driver is reusable across any future embedded project

---

## Layer Model

| Layer | Location | Responsibility |
|---|---|---|
| Application | `runner/src/main.rs` | State machine, shot logic, safety interlocks |
| Service | `runner/src/` | PID controllers, shot profiler, data logger |
| Driver | `drivers/<component>/src/lib.rs` | Per-peripheral hardware drivers |
| Platform | `embassy-stm32` | GPIO, SPI, I2C, DAC, timers, DMA |

Each driver:

- Has no knowledge of other drivers — fully decoupled
- Depends only on `embedded-hal` traits — not `embassy-stm32` directly
- Can be unit tested in isolation on the host machine
- Is a standalone Cargo crate

---

## Driver Architecture

Drivers are built against `embedded-hal` and `embedded-hal-async` traits. The `runner` crate imports `embassy-stm32`, instantiates real hardware, and passes concrete peripheral types into each driver.

| Driver | Trait required |
|---|---|
| `esp8266` | `embedded_io::Read` + `Write` |
| `fg304` | DAC output (custom trait — embedded-hal has none) |
| `max6675` | `SpiDevice` |
| `ads1015` | `I2c` |
| `xdb401` | pure math — no hardware trait |
| `hx711` | `InputPin` + `OutputPin` |
| `vl53l0x` | `I2c` |

---

## State Machine

See `docs/firmware/state_machine.md` for full state definitions.

| Transition | Condition |
|---|---|
| IDLE → BREWING | Brew button pressed, temp in range |
| BREWING → IDLE | Target weight reached or shot timeout |
| IDLE → HEATING | Steam button pressed |
| HEATING → STEAMING | Steam temp reached |
| STEAMING → COOLDOWN | Steam button released |
| COOLDOWN → IDLE | Temp within 5 °C of brew setpoint |
| Any → FAULT | Safety condition triggered |
| FAULT → IDLE | Brew button held 3 seconds |

---

## PID Control Loops

Two independent PID loops running as Embassy async tasks.

**Temperature PID**

- Input: MAX6675 thermocouple reading
- Output: SSR duty cycle (time-proportional, 1-second window)
- Setpoints: brew 93 °C / steam 145 °C

**Pressure PID**

- Input: ADS1015 + XDB401 pressure reading
- Output: FG304 DAC speed command (0–0.8 V for espresso range)
- Setpoint: 9 bar default

---

## Safety Interlocks

Safety checks run on every cycle before any actuator output.

| Condition | Action |
|---|---|
| Temp > SAFETY_MAX_TEMP_C | SSR forced off → FAULT |
| Pressure > SAFETY_MAX_PRESSURE_BAR | Pump stopped → FAULT |
| Water level < SAFETY_LOW_WATER_MM | Pump disabled → FAULT |
| Pump fault pin asserted | Pump stopped → FAULT |
| Thermocouple open-circuit | SSR forced off → FAULT |

---

## Timing

| Task | Period | Notes |
|---|---|---|
| Temperature PID | 100 ms | Embassy timer task |
| Pressure PID | 100 ms | Embassy timer task |
| Scale read | 200 ms | HX711 conversion ~100 ms |
| Safety check | Every cycle | Runs before any actuator output |
| Wi-Fi telemetry | 1 s | UART to ESP8266, non-blocking |

---

## File Map

| File | Purpose |
|---|---|
| `runner/src/main.rs` | Entry point, Embassy executor, state machine |
| `drivers/esp8266/src/lib.rs` | ESP8266 AT command Wi-Fi bridge — UART |
| `drivers/fg304/src/lib.rs` | FG304 pump driver — DAC speed + TACHO input |
| `drivers/max6675/src/lib.rs` | MAX6675 thermocouple reader — SPI |
| `drivers/ads1015/src/lib.rs` | ADS1015 ADC driver — I2C |
| `drivers/xdb401/src/lib.rs` | XDB401 pressure conversion — pure math |
| `drivers/hx711/src/lib.rs` | HX711 load cell ADC — bit-bang GPIO |
| `drivers/vl53l0x/src/lib.rs` | VL53L0X water level sensor — I2C |
| `docs/firmware/state_machine.md` | State definitions and transitions |
| `docs/hardware/pinout.md` | STM32 pin assignments |
| `docs/PARTS.md` | Hardware reference |
