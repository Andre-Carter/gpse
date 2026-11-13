(macro, purpose, example)

assert!     check a boolean condition   assert!(result > 0)
assert_eq!  check equality              assert_eq!(result, 21)
assert_ne!  check inequality            assert_ne!(result, 0)

Rust will show which assertion failed and, for assert_eq!, the actual and expected values.