#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::sync::atomic::Ordering;
use core::sync::atomic::AtomicU32;
//use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::main;
//use esp_hal::time::{Duration, Instant};
use panic_rtt_target as _;
use esp_println::println;

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 1.0.1
    println!("🦀 WASM Host Demo - Fibonacci Generator (wasmi Runtime)");

    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    //esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    esp_alloc::heap_allocator!(size: 131072);

    run_wasm();

    loop {}
}

use wasmi::{Engine, Linker, Module, Store};

fn run_wasm() {
    // 1. Embed the Wasm binary as a byte array
    let wasm_bytes = include_bytes!("../../../guest-fibonacci/target/wasm32-unknown-unknown/release/guest_fibonacci.wasm");
    //let wasm_bytes = include_bytes!("../../../guest-fibonacci/target/wasm32v1-none/release/guest_fibonacci.wasm");
    //let wasm_bytes = include_bytes!("../../minimal.wasm");
    //let wasm_bytes = include_bytes!("../../42.wasm");
    //let wasm_bytes = include_bytes!("../../memory.wasm");

    // 2. Set up the wasmi engine and store
    println!("Initialising engine...");
    let engine = Engine::default();
    println!("Initialising module...");
    let module = Module::new(&engine, wasm_bytes).expect("Failed to create module");
    println!("Initialising store...");
    let mut store = Store::new(&engine, ());
    println!("Initialising linker...");
    let mut linker = Linker::<()>::new(&engine);

    let count = AtomicU32::new(0);

    // Define the host function that the WASM module can call
    linker.func_wrap("env", "output", move |num: u64| {
        let c = count.fetch_add(1, Ordering::Relaxed) + 1;
        if c % 10_000 == 0 {
            println!("-- Calculated {} Fibonacci numbers -- {num}", c);
        }
    }).expect("Failed to define host function");

    // 3. Instantiate the module
    println!("Instantiating instance...");
    let instance = linker.instantiate_and_start(&mut store, &module).expect("Failed to instantiate module");

    // Get the 'fib' function from the WASM module
    println!("Fetching 'fib' function...");
    let fib_func = instance
        .get_typed_func::<u64, u64>(&mut store, "fib")
        .expect("Failed to get 'fib' function");

    println!("Fetching 'add' function...");
    let add_func = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "add")
        .expect("Failed to get 'add' function");

    // Call the 'add' function as a quick test
    println!("Calling 'add' function...");
    let result = add_func.call(&mut store, (41, 1)).expect("Failed to call 'add' function");
    println!("add(41, 1) returned: {result}");

    // Call the 'fib' function to start the Fibonacci sequence
    println!("Calling 'fib' function...");
    let result = fib_func.call(&mut store, 100_000).expect("Failed to call 'fib' function");
    println!("fib(100_000_000) returned: {result}");
}
