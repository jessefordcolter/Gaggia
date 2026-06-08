# Parts Reference

All components used in the Gaggia espresso machine mod project.

---

## 1. Pressure Sensor

| Field | Value |
|-------|-------|
| **Name** | XDB401 Series Pressure Transmitter |
| **Manufacturer** | Xidibei (Shenzhen Xidibei Sensing Technology Co., Ltd) |
| **Model** | XDB401 |
| **Type** | Ceramic capacitive, analog voltage output |
| **Supply voltage** | 5–12 V DC |
| **Output signal** | 0.5–4.5 V (ratiometric) |
| **Pressure range** | 0–1.2 MPa (0–12 bar / 0–174 psi) |
| **Connector** | 3-wire |

### Wire colours

| Colour | Function |
|--------|----------|
| Red | VCC (power, 5–12 V) |
| Black | GND (ground) |
| Yellow | AO (analog signal output, 0.5–4.5 V) |

### Notes

- Sensor range (0–1.2 MPa = 0–12 bar) is a near-perfect match for espresso.
  Normal brew pressure (9 bar) sits at 75% of full scale, giving good resolution
  across the entire working range without wasting ADC headroom.
- With the ADS1015 at ±4.096 V gain, the 4.0 V sensor output maps to ~2000
  ADC counts across 12 bar → approximately **6 mbar per count**.
- Output voltage (max 4.5 V) exceeds STM32 GPIO max (3.3 V) — use the ADS1015
  ADC module. Do NOT connect the sensor output directly to any MCU GPIO.

---

## 2. ADC Module

| Field | Value |
|-------|-------|
| **Name** | ADS1015 Ultra-Small, Low-Power 12-Bit ADC |
| **Manufacturer** | Texas Instruments |
| **Model** | ADS1015 |
| **Type** | I²C ADC with internal PGA and oscillator |
| **Supply voltage** | 2.0–5.5 V |
| **Resolution** | 12-bit |
| **Sample rate** | 128–3300 SPS (programmable) |
| **Channels** | 4 single-ended or 2 differential |
| **Interface** | I²C |
| **I²C address** | 0x48 (ADDR pin tied to GND) |
| **PGA range** | ±0.256 V to ±6.144 V (programmable) |

### Module pin wiring

| Module pin | Wire colour | Connected to |
|------------|-------------|--------------|
| V (VDD) | Red | 3.3 V power |
| G (GND) | Black | Ground |
| SCL | Yellow | I²C clock |
| SDA | Blue | I²C data |
| ADDR | — | GND (sets address 0x48) |
| A0 (AIN0) | Yellow (sensor) | XDB401 sensor output |
| A2 (AIN2) | Red | TBD |
| ALERT | — | Not connected |
| A1 (AIN1) | — | Not connected |
| A3 (AIN3) | — | Not connected |

### STM32 connection

| ADS1015 pin | STM32 pin | Notes |
|-------------|-----------|-------|
| VDD | 3.3 V | |
| GND | GND | |
| SCL | TBD | I²C clock — assigned when driver is written |
| SDA | TBD | I²C data — assigned when driver is written |
| A0 | — | Via voltage divider from sensor (10 kΩ / 27 kΩ) |

### Voltage divider (required)

Sensor AO (max 4.5 V) must be scaled to ≤ 3.3 V before AIN0.

| Node | Voltage |
|------|---------|
| Sensor AO | 4.5 V max |
| R1 | 10 kΩ (top) |
| R2 | 27 kΩ (bottom to GND) |
| AIN0 junction | 3.28 V max ✓ |

R1 and R2 form a divider: Vout = Vin × R2 / (R1 + R2)
= 4.5 × 27 / (10 + 27) = 3.28 V ✓

### Why use ADS1015 instead of MCU raw ADC?

- XDB401 output (max 4.5 V) exceeds STM32F411 ADC input max (3.6 V) —
  direct connection would damage the MCU
- ADS1015 is isolated from the STM32 digital noise via I²C
- PGA allows gain tuning for better resolution across the 0–12 bar range
- 4 channels allow multiple sensors on one module
- Dedicated conversion clock eliminates interference from MCU activity

---

## 3. Pump-Motor Unit

| Field | Value |
|-------|-------|
| **Name** | FG Series Pump-Motor Unit (with integrated electronic driver) |
| **Manufacturer** | Fluid-o-Tech |
| **Model** | FG304xD0PT10000 |
| **Type** | BLDC mag-drive external gear pump, 24 V |
| **Supply voltage** | 20–29 V DC (nominal 24 V) |
| **Max supply current** | 3.5 A (intermittent) / 2.5 A (continuous >30 min) |
| **Speed range** | 300–5000 RPM |
| **Speed command** | Analog 0–5 V (orange wire) |
| **Max discharge pressure** | 12 bar (174 psi) |
| **Max fluid temperature** | 55 °C at max torque / 95 °C at lower torque |
| **Max ambient temperature** | 40 °C at max torque / 70 °C at lower torque |
| **Protection level** | IP52 |
| **Thermal protection** | Auto-stop at 120 °C, restart when <110 °C |
| **Under-voltage protection** | Stops at <15.5 V, restarts when >16.5 V |
| **Over-voltage protection** | Stops at >30 V, restarts when <29 V |
| **Stall protection** | Auto-stop if unable to rotate for 1 second |

### Wire colours (version WITH integrated electronic driver)

| Colour | Symbol | Function |
|--------|--------|----------|
| Red | PWR_VCC | Power supply +24 V |
| Black | PWR_GND | Power ground (0 V) |
| Orange | SPEED_IN | Analog speed command 0–5 V (0–5000 RPM) |
| Brown | 0V | Speed command reference ground |
| Yellow | TACHO OUT | Speed feedback: 0–5 V square wave, max 2.7 kHz |
| Green | DIRECTION | <2 V = CW (pump side), >4 V = CCW |

### ⚠ Critical wiring notes

**1. Never connect PWR_GND (black) and 0V (brown) together.**
They are separate ground references. Connecting them will permanently damage
the integrated driver board.

**2. TACHO OUT (yellow) is 0–5 V — requires voltage divider before STM32.**
Use R1=10 kΩ, R2=27 kΩ. STM32 GPIO max is 3.3 V.

**3. SPEED_IN (orange) needs 0–5 V for full 5000 RPM range.**
STM32F411 DAC outputs 0–3.3 V → max ~3300 RPM without op-amp scaling.
STM32F411 has a true 12-bit DAC — better resolution than ESP32.
For full 5000 RPM range, scale 3.3 V → 5 V via op-amp (e.g. MCP6002).

**4. DIRECTION (green) needs >4 V for CCW — STM32 3.3 V GPIO won't reach this.**
Tie to GND (via 10 kΩ) for permanent CW operation.

### Speed command details

| SPEED_IN voltage | Output speed |
|-----------------|--------------|
| < 0.2 V | 0 (stopped) |
| 0.3–5.0 V | 300–5000 RPM (linear ±5%) |

### TACHO output formula

`Speed (RPM) = Frequency (Hz) × 60 / 32`

---

## 4. Microcontroller

| Field | Value |
|-------|-------|
| **Name** | STM32F411CEU6 Blackpill |
| **Manufacturer** | WeAct Studio v3.1 |
| **Chip** | STM32F411CEU6 |
| **CPU** | ARM Cortex-M4 with FPU |
| **Clock** | Up to 100 MHz |
| **Flash** | 512K @ 0x08000000 |
| **SRAM** | 128K @ 0x20000000 |
| **Supply voltage** | 1.7–3.6 V (logic) |
| **Connector** | USB-C |
| **Pins** | 40 (2×20) |

---

## 5. Wi-Fi Module

| Field | Value |
|-------|-------|
| **Name** | ESP8266 |
| **Role** | Wi-Fi co-processor — bridges STM32 UART to Nexus 7 tablet |
| **Interface to STM32** | UART |

---

## 6. Pressure Sensor ADC — see Section 2 (ADS1015)

---

## 7. Load Cell ADC

| Field | Value |
|-------|-------|
| **Name** | HX711 24-bit ADC |
| **Quantity** | 2 boards (4 load cells total) |
| **Interface** | Bit-bang GPIO (DAT + CLK per board) |
| **Supply voltage** | 2.6–5.5 V |
| **Resolution** | 24-bit |
| **Gain (channel A)** | 64 or 128 |
| **Conversion rate** | 10 or 80 SPS |

### STM32 connection

| HX711 pin | STM32 pin | Notes |
|-----------|-----------|-------|
| DAT (A) | TBD | Assigned when driver is written |
| CLK (A) | TBD | Assigned when driver is written |
| DAT (B) | TBD | Assigned when driver is written |
| CLK (B) | TBD | Assigned when driver is written |

---

## 8. Thermocouple Amplifier

| Field | Value |
|-------|-------|
| **Name** | MAX6675 |
| **Interface** | SPI (read-only) |
| **Thermocouple type** | K-type |
| **Range** | 0–1024 °C |
| **Resolution** | 0.25 °C |
| **Supply voltage** | 3.0–5.5 V |
| **Conversion time** | ~170 ms |

### STM32 connection

| MAX6675 pin | STM32 pin | Notes |
|-------------|-----------|-------|
| CS | TBD | Assigned when driver is written |
| SCK | TBD | SPI clock |
| MISO | TBD | SPI data out |
| VCC | 3.3 V | |
| GND | GND | |

---

## 9. Water Level Sensor

| Field | Value |
|-------|-------|
| **Name** | VL53L0X |
| **Interface** | I²C |
| **Default address** | 0x29 |
| **Range** | Up to 2 m |
| **Supply voltage** | 2.6–3.5 V |

---

## 10. Solid State Relay

| Field | Value |
|-------|-------|
| **Name** | ZGT-40 DA |
| **Type** | DC control, AC load |
| **Control voltage** | 3–32 V DC |
| **Load** | Boiler heater element (120 V AC) |
| **STM32 pin** | TBD — assigned when driver is written |
