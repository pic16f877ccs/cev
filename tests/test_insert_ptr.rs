use cev::Cev;
use std::ptr::NonNull;

#[test]
fn test_extra_insert() {
    let mut cev = Cev::<u8>::new();
    cev.insert(0, 1);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len()) as isize);
    assert_eq!(cev, [1]);
    cev.insert(1, 2);
    assert_eq!(cev, [1, 2]);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len()) as isize);
    cev.insert(0, 0);
    assert_eq!(cev, [0, 1, 2]);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len()) as isize);
    cev.insert(3, 3);
    assert_eq!(cev, [0, 1, 2, 3]);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len()) as isize);
    cev.insert(4, 4);
    assert_eq!(cev, [0, 1, 2, 3, 4]);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len()) as isize);
    cev.insert(5, 5);
    assert_eq!(cev, [0, 1, 2, 3, 4, 5]);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len()) as isize);
    cev.insert(6, 6);
    assert_eq!(cev, [0, 1, 2, 3, 4, 5, 6]);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len()) as isize);
    cev.insert(7, 7);
    assert_eq!(cev, [0, 1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len()) as isize);
    cev.insert(8, 8);
    assert_eq!(cev, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len()) as isize);
    let mut cev = Cev::<u8>::with_capacity(1);
    cev.insert(0, 2);
    assert_eq!(cev, [2]);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len()) as isize);

    let mut cev = Cev::<()>::new();
    cev.insert(0, ());
    assert_eq!(cev, [()]);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());

    assert_eq!(cev, [()]);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    cev.insert(1, ());
    assert_eq!(cev, [(), ()]);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    cev.insert(0, ());
    assert_eq!(cev, [(), (), ()]);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
}

