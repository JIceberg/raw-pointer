# raw-pointer

Allows for safely handling raw, mutable pointers to a generic type in Rust.

## Usage

### Pointer to Data
```rust
let mut val: u32 = 6;
let mut ptr_val = Pointer::<u32>::new(&mut val);

*ptr_val = 9;
```

### Pointer to Specific Address
```rust
let mut temp_tegister: Pointer<MyRegister> = Pointer::<MyRegister>::from(0xFFFF0000);

test_and_set(temp_register.value);
```
