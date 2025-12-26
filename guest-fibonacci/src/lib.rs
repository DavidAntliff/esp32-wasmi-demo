#![no_std]

// External function provided by the host
extern "C" {
    fn output(num: u64);
}


// Don't call the entry 'main' as it will get wrapped with C-style (argc, argv) parameters

#[no_mangle]
pub extern "C" fn fib(mut count: u64) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    unsafe {
        output(a);
        output(b);
    }
    
    while count > 0 {
        let next = a.wrapping_add(b);
        unsafe {
            output(next);
        }
        
        a = b;
        b = next;

        // Prevent overflow by resetting when numbers get too large
        if next > 1_000_000_000_000_000_000 {
            a = 0;
            b = 1;
        }

        count -= 1;
    }

    b
}

#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Panic handler required for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
