default:
    just --list

# Build everything
build: build-guest build-host

# Build the guest WASM application
build-guest:
    just -f guest-fibonacci/justfile build

# Build the host Wasmi application
build-host:
    just -f host-esp32c6/justfile build

clean:
    just -f guest-fibonacci/justfile clean
    just -f host-esp32c6/justfile clean
