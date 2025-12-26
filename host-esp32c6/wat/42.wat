;; A simple WebAssembly module that exports a function "main" returning the integer 42.
(module
  (func
    (export "main") (result i32) i32.const 42
  )
)
