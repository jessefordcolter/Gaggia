# State Machine Design

> The Keynote design file is at `hardware/schematics/StateMachines.key`
> This document is the text companion to that diagram.

---

## States

### IDLE

The machine is powered on, warmed up, and ready to brew. The boiler
temperature PID is active and holding the brew setpoint. The pump is off.

Transitions out:

- Brew button pressed + temp in range → BREWING
- Brew button pressed + temp out of range → stay in IDLE (no action)
- Steam button pressed → HEATING
- Any safety condition → FAULT

### BREWING

The pump is running. The pressure PID is active targeting 9 bar.
The temperature PID remains active.

Transitions out:

- Target weight reached → IDLE
- Shot timer exceeds timeout → IDLE (safety cut-off)
- Brew button pressed again → IDLE (manual stop)
- Any safety condition → FAULT

### HEATING

Boiler ramping to steam temperature. Pump is off.

Transitions out:

- Steam temp reached → STEAMING
- Any safety condition → FAULT

### STEAMING

Steam wand active. Boiler holds steam setpoint. Pump is off.

Transitions out:

- Steam button released → COOLDOWN
- Any safety condition → FAULT

### COOLDOWN

Machine locked out from brewing while boiler cools from steam temp back
to brew temp. Temperature PID active, targeting brew setpoint.

Transitions out:

- Temp ≤ brew setpoint + 5 °C → IDLE
- Any safety condition → FAULT

### FAULT

All outputs disabled — pump off, SSR off. Machine will not operate until
fault is cleared and manually reset via brew button held for 3 seconds.

Fault sources:

- Over-temperature (thermocouple > SAFETY_MAX_TEMP_C)
- Over-pressure (transducer > SAFETY_MAX_PRESSURE_BAR)
- Low water (VL53L0X distance > SAFETY_LOW_WATER_MM)
- Pump driver fault (FAULT pin asserted)
- Thermocouple open-circuit (MAX6675 fault flag)

Transitions out:

- Fault condition cleared + brew button held 3 seconds → IDLE

---

## Implementation Notes

The state machine is implemented in Rust as an enum with Embassy async tasks.
Each state is a variant. Transitions are explicit match arms.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum MachineState {
    Idle,
    Brewing,
    Heating,
    Steaming,
    Cooldown,
    Fault(FaultReason),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FaultReason {
    OverTemperature,
    OverPressure,
    LowWater,
    PumpFault,
    ThermocoupleOpen,
}
```

Each state has:

- An entry action (called once on state entry)
- A periodic update (async task, awaits sensor readings)
- Exit conditions (evaluated each update cycle)

Safety checks run on every cycle before any actuator output.
