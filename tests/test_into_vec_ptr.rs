use cev::Cev;
use std::ptr::NonNull;

#[test]
fn test_into_vec_ptr() {
    let cev: Cev<u8> = Cev::new();
    let vec = cev.into_vec();
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.as_ptr(), NonNull::dangling().as_ptr());
    assert_eq!(vec, []);

    let cev: Cev<u8> = Cev::with_capacity(0);
    let vec = cev.into_vec();
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.as_ptr(), NonNull::dangling().as_ptr());
    assert_eq!(vec, []);

    let cev: Cev<u8> = Cev::with_capacity(1);
    let raw_ptr = cev.raw_ptr();
    let mov_ptr = cev.as_ptr();
    let vec = cev.into_vec();
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 1);
    assert_eq!(vec.as_ptr(), mov_ptr);
    assert_eq!(vec.as_ptr(), raw_ptr);
    assert_eq!(vec, []);

    let mut cev: Cev<u8> = Cev::with_capacity(1);
    cev.push(173);
    let raw_ptr = cev.raw_ptr();
    let mov_ptr = cev.as_ptr();
    let vec = cev.into_vec();
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.capacity(), 1);
    assert_eq!(vec.as_ptr(), mov_ptr);
    assert_eq!(vec.as_ptr(), raw_ptr);
    assert_eq!(vec, [173]);

    let mut cev: Cev<u8> = Cev::with_capacity(5);
    cev.push(173);
    cev.push(172);
    cev.push(171);
    assert_eq!(cev[0], 171);
    let raw_ptr = cev.raw_ptr();
    let vec = cev.into_vec();
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.capacity(), 5);
    assert_eq!(vec.as_ptr(), raw_ptr);
    assert_eq!(vec[0], 171);
    assert_eq!(unsafe { *raw_ptr }, 171);
    assert_eq!(vec, [171, 172, 173]);

    let cev: Cev<()> = Cev::with_capacity(0);
    let vec = cev.into_vec();
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), usize::MAX);
    assert_eq!(vec.as_ptr(), NonNull::dangling().as_ptr());
    assert_eq!(vec, []);

    let mut cev: Cev<()> = Cev::with_capacity(1);
    cev.push(());
    let vec = cev.into_vec();
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.capacity(), usize::MAX);
    assert_eq!(vec.as_ptr(), NonNull::dangling().as_ptr());
    assert_eq!(vec, [()]);

    let mut cev: Cev<()> = Cev::with_capacity(7);
    cev.push(());
    cev.push(());
    cev.push(());
    cev.push(());
    let vec = cev.into_vec();
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.capacity(), usize::MAX);
    assert_eq!(vec.as_ptr(), NonNull::dangling().as_ptr());
    assert_eq!(vec, [(), (), (), ()]);
}
