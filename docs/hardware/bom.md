# Bill of Materials

> Last updated: 2026-06-08
> Status indicators: ✅ In hand · 🔲 To order · ❓ Under evaluation

---

## Microcontroller

| Item | Part Number | Notes | Status |
|---|---|---|---|
| STM32 Blackpill | STM32F411CEU6 | 100 MHz, 512K flash, 128K SRAM | ✅ |
| WeAct Mini Debugger | STM32F103CBT6 | SWD + UART interface | ✅ |

---

## Wi-Fi

| Item | Part Number | Notes | Status |
|---|---|---|---|
| ESP8266 module | ESP8266 | Wi-Fi bridge to Nexus 7 tablet via UART | ✅ |

---

## Sensors

| Item | Part Number | Interface | Status |
|---|---|---|---|
| Thermocouple amplifier | MAX6675 | SPI | ✅ |
| K-type thermocouple | — | — | ✅ |
| Pressure transducer | XIDIBEI XDB401 | Analog 0.5–4.5 V | ✅ |
| ADC for pressure | ADS1015 | I2C | ✅ |
| Time-of-flight (water level) | VL53L0X | I2C | ✅ |
| Load cells (×4) | — | Wheatstone bridge | ✅ |
| Load cell ADC boards (×2) | HX711 | Bit-bang GPIO | ✅ |

---

## Actuators & Control

| Item | Part Number | Notes | Status |
|---|---|---|---|
| Solid state relay (boiler) | ZGT-40 DA | DC control, AC load, 3–32 V input | ✅ |
| Gear pump | FG304XD0PT10000 | Max ~800 RPM for espresso flow rates | ✅ |
| Overpressure relief valve | — | Required — install after pump outlet | 🔲 |
| 10 µm inlet pre-filter | — | Required per FG304 datasheet | 🔲 |

---

## Power

| Item | Notes | Status |
|---|---|---|
| 24 V DC PSU | FG304 pump driver (16.5–29 V range) | 🔲 |
| 5 V DC PSU | ADS1015, ESP8266 | 🔲 |
| 3.3 V LDO | Blackpill onboard — verify current budget | ✅ |

---

## Passives & Connectors

| Item | Notes | Status |
|---|---|---|
| Decoupling capacitors | 100 nF ceramic, one per IC power pin | 🔲 |
| Resistors (10 kΩ, 27 kΩ) | Voltage dividers for TACHO and pressure sensor | 🔲 |
| Food-grade silicone tubing | Min 18 bar rating, food-safe at 150 °C | 🔲 |
| Brass or stainless fittings | Like-metal only — no mixed metals | 🔲 |

---

## Monitoring

| Item | Notes | Status |
|---|---|---|
| Nexus 7 (2013) tablet | LineageOS, receives shot data over Wi-Fi | ✅ |

---

## Notes

- All plumbing fittings must be the same metal — galvanic corrosion risk.
- Tubing rated minimum 18 bar (1.5× safety factor on 12 bar max).
- FG304 requires 16.5–29 V DC — dedicated 24 V PSU required.
- ESP8266 may need its own 3.3 V regulator — verify vs Blackpill LDO rating.
- Espresso flow rates (1–3 mL/s) require ~600–900 RPM — well under the ~3300 RPM
  achievable at 3.3 V. STM32 DAC is sufficient; no op-amp required.
- TACHO OUT (0–5 V) requires 10 kΩ / 27 kΩ voltage divider before STM32 GPIO.
- Pressure sensor AO (0.5–4.5 V) requires 10 kΩ / 27 kΩ voltage divider
  before ADS1015 AIN0.
