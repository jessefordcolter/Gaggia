# ESP-01 AT Firmware Flash Procedure

## Hardware

- Module: ESP-01 (ESP8266EX)
- Flash chip: 1MB (manufacturer ID 0x5E, device 0x3214)
- Flash mode: DOUT (verified — DIO gives csum err, QIO gives flash read err)
- Flash freq: 40MHz (conservative bound — manufacturer unconfirmed from chip markings)

## Firmware

- SDK: ESP8266 NonOS SDK v2.2.1
- Source: https://github.com/espressif/ESP8266_NONOS_SDK/archive/refs/tags/v2.2.1.zip
- Extract to: `hardware/firmware/esp8266/ESP8266_NONOS_SDK-2.2.1/`

## Flash command

From `hardware/firmware/esp8266/ESP8266_NONOS_SDK-2.2.1/bin/`:

```
esptool --port /dev/cu.usbserial-10 --baud 115200 write-flash \
  --flash-mode dout --flash-freq 40m --flash-size 1MB \
  0x00000 boot_v1.7.bin \
  0x01000 at/512+512/user1.1024.new.2.bin \
  0xFB000 blank.bin \
  0xFC000 esp_init_data_default_v08.bin \
  0xFE000 blank.bin
```

Put chip in flash mode before running: hold PROG, press RST, release RST, release PROG.

## Verification

After flashing, connect CoolTerm at 115200 baud, press RST (no PROG).
Boot message appears at 74880 baud (garbled — normal).
After boot, type `AT` + Return. Chip responds `OK`.

## CoolTerm settings

- Baud: 115200
- Enter key emulation: CR+LF
- Local echo: off
- Software flow control: off
- DTR: off
- RTS: off
