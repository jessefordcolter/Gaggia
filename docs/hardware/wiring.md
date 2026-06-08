# Wiring Notes & Safety Considerations

> ⚠️ This machine operates at 120 V AC, 1350 W. Work on the AC side only
> when the machine is **fully unplugged from mains power**. The DC control
> electronics can be developed and tested independently with a bench supply.

---

## Power Architecture

| Rail | Voltage | Supplies |
|------|---------|----------|
| Mains | 120 V AC | Boiler element (via SSR), PSUs |
| PSU A | 24 V DC | FG304 pump driver |
| PSU B | 5 V DC | ADS1015, ESP8266 |
| Blackpill LDO | 3.3 V | STM32, VL53L0X, HX711, MAX6675 |

---

## Noise Mitigation

This machine has three significant noise sources in a small enclosure:

- SSR switching 120 V AC
- FG304 pump motor with switching driver
- Switch-mode power supplies

**Required practices:**

- 100 nF ceramic decoupling capacitor on every IC power pin, placed as close to the pin as possible
- Route analog signal wires away from motor and SSR wires — opposite sides of enclosure
- Keep signal wires short
- Twist analog signal wire pairs where possible
- Single ground point — analog ground and power ground joined at one location only

---

## SSR Wiring

The ZGT-40 DA is a DC-controlled SSR. It passes AC current to the boiler
element when the control input (3–32 V DC) is HIGH.

- **AC side:** inline with the boiler element hot wire
- **DC control:** STM32 GPIO (TBD) → SSR control input (+), GND → SSR control (-)
- 3.3 V HIGH from the STM32 is sufficient to drive the SSR

> Never bypass the SSR with a direct wire. The SSR is the primary safety
> boundary between firmware and a live heating element.

---

## Pump Wiring

The FG304 uses a 6-wire connector. Full wire colour reference in `docs/PARTS.md` Section 3.

| Wire colour | Symbol | Function |
|-------------|--------|----------|
| Red | PWR_VCC | 24 V DC power |
| Black | PWR_GND | Power ground |
| Orange | SPEED_IN | 0–5 V analog speed command |
| Brown | 0V | Speed command reference ground |
| Yellow | TACHO OUT | Speed feedback 0–5 V square wave |
| Green | DIRECTION | <2 V = CW, >4 V = CCW |

> ⚠️ Never connect PWR_GND (black) and 0V (brown) together.
> They are separate ground references. This will permanently damage the driver board.

**Speed command circuit (STM32 DAC 0–3.3 V → 0–5 V pump input):**

STM32 DAC → op-amp (rail-to-rail, 5 V supply, gain ~1.5×) → SPEED_IN (orange)

STM32F411 has a true 12-bit DAC. Op-amp scaling to 5 V required for full
5000 RPM range. Without scaling, max speed is ~3300 RPM.

**TACHO input (0–5 V → STM32 3.3 V max):**

TACHO OUT → voltage divider (R1=10 kΩ, R2=27 kΩ) → STM32 GPIO (TBD)

Formula: Speed (RPM) = Frequency (Hz) × 60 / 32

---

## Plumbing Safety

- **Pressure limit:** 12 bar maximum operating. Overpressure valve is the
  mechanical backup — never rely on firmware alone.
- **Safety factor:** Tubing rated for at least 18 bar (1.5× safety factor)
  and food-safe at 150 °C minimum.
- **Fittings:** Like-metal only — stainless-to-stainless or brass-to-brass.
  Mixed metals cause galvanic corrosion in a water system.
- **Pre-filter:** 10 µm filter before pump inlet — required per FG304 datasheet.

---

## Grounding

- Chassis ground of the Gaggia must be maintained for safety.
- Signal ground and chassis/earth ground must NOT be directly connected —
  ground loops will corrupt sensor readings.
- All DC signal grounds join at a single point.

---

## Reference Files

- `hardware/schematics/GaggiaSchematics.key`
- `hardware/images/GCP_120v.jpg`
- `hardware/images/GCP_wiring_PCBv3_120v.png`
- `hardware/images/pump_wiring_1.png`
- `hardware/images/pump_wiring_2.png`
