;; A simple WebAssembly module that exports a 1-page memory (64 KiB),
;; and a function returning the integer 42.
(module
  (memory (export "memory") 1)
  (func (export "main") (result i32) i32.const 42)
)
