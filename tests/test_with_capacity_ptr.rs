use cev::Cev;
use std::ptr::NonNull;

#[test]
fn test_with_capacity_ptr() {
    let cev = Cev::<u8>::with_capacity(0);
    assert_eq!(cev.capacity(), 0);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.as_ptr(), NonNull::dangling().as_ptr());

    let cev = Cev::<u64>::with_capacity(1);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.capacity(), 1);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - 1) as isize);

    let cev = Cev::<isize>::with_capacity(6);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.capacity(), 6);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - 1) as isize);

    let cev = Cev::<()>::with_capacity(0);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.capacity(), usize::MAX);
    assert_eq!(cev.as_ptr(), NonNull::dangling().as_ptr());

    let cev = Cev::<()>::with_capacity(1);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.capacity(), usize::MAX);
    assert_eq!(cev.as_ptr(), NonNull::dangling().as_ptr());

    let cev = Cev::<()>::with_capacity(5);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.capacity(), usize::MAX);
    assert_eq!(cev.as_ptr(), NonNull::dangling().as_ptr());

    let cev = Cev::<()>::with_capacity(usize::MAX);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.capacity(), usize::MAX);
    assert_eq!(cev.as_ptr(), NonNull::dangling().as_ptr());
}
