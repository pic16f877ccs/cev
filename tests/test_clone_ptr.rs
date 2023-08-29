use cev::Cev;
use std::ptr::NonNull;

#[test]
fn test_clone_ptr() {
    let cev = Cev::<u64>::from([5, 4, 3, 2, 1, 0]);
    let cld = cev.clone();
    assert_eq!(cev, cld);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());

    let cev = Cev::<u8>::new();
    let cld = cev.clone();
    assert_eq!(cev, cld);
    assert_eq!(cev.as_ptr(), NonNull::<u8>::dangling().as_ptr());

    let cev = Cev::<usize>::with_capacity(1);
    let cld = cev.clone();
    assert_eq!(cev, cld);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - 1) as isize);
}
