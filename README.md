# raw-pointer

Allows for safely handling raw, mutable pointers to a generic type in Rust.

## Usage

### Pointer to Data
```rust
let val: u32 = 6;
let ptr_val = Pointer::<u32>::new(&mut val);

*ptr_val = 9;
```

### Pointer to Specific Address
```rust
let temp_register: Pointer<MyRegister> = Pointer::<MyRegister> {
  ptr: 0xFFFF0000 as *mut MyRegister,
};
```
