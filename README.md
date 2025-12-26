# ESP32 Wasmi Demo

This is a simple project demonstrating the use of the [Wasmi WebAssembly Interpreter](https://github.com/wasmi-labs/wasmi)
with an ESP32-C6 microcontroller, using `no_std` + `alloc`.

`host-esp32c6` is a simple **riscv32imac-unknown-none-elf** targeted application that provides
some host functions and executes a built-in WASM program.

`guest-fibonacci` is a simple WebAssembly program that, when run on the host, makes use of the host
function bindings to access host features.

The demo assumes a grid of 16x16 WS21812 LEDs connected to GPIO10 in a sequential serpentine
arrangement. A Wokwi configuration is provided to simulate this, if such hardware is not available.

## Wasmi Limitations

Wasmi 1.0 supports `no_std` but requires atomics support. This makes it problematic for use
with targets that do not support atomics, such as the ESP32-C3, which is a **riscv32imc-unknown-none-elf**
target and lacks hardware support for atomic operations.

I did attempt to modify `wasmi` and `wasm-tools` to support atomics via `portable-atomic`,
however I ran out of steam, and know-how, and was unable to get this to work.

Changing to a target like the ESP32-C6 that _does_ support atomics was a lot simpler.
