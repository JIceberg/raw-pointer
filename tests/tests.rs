#![cfg(test)]

use raw_pointer::Pointer;

#[test]
fn test_pointer_safe() {
    let mut num: u32 = 5;
    let _ = Pointer::<u32>::new(&mut num);
}

#[test]
fn test_correct_value() {
    let mut num: u32 = 5;
    let ptr_num = Pointer::<u32>::new(&mut num);

    assert_eq!(ptr_num.unwrap(), &num);

    num = 2;
    assert_eq!(num, 2);
    assert_eq!(ptr_num.unwrap(), &2u32);
}

#[test]
fn test_changes_value() {
    let mut num: u32 = 5;
    let ptr_num = Pointer::<u32>::new(&mut num);

    assert_eq!(ptr_num.unwrap(), &5u32);

    num = 2;
    assert_eq!(num, 2);
    assert_eq!(ptr_num.unwrap(), &2u32);
}

#[test]
fn test_mutable_value() {
    let mut num: u32 = 5;
    let ptr_num = Pointer::<u32>::new(&mut num);

    *ptr_num.unwrap_mut() = 2u32;
    assert_eq!(num, 2);
}

#[test]
fn test_dereference() {
    let mut num: u32 = 5;
    let mut ptr_num = Pointer::<u32>::new(&mut num);

    assert_eq!(*ptr_num, 5u32);
    *ptr_num = 2;
    assert_eq!(num, 2);
}
