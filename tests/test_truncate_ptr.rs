use cev::Cev;
use std::ptr::NonNull;

#[test]
fn test_truncate_ptr() {
    let mut cev: Cev<u8> = Cev::new();
    cev.truncate(0);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.as_ptr(), NonNull::<u8>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<u8> = Cev::with_capacity(0);
    cev.truncate(0);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.as_ptr(), NonNull::<u8>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<u8> = Cev::with_capacity(1);
    cev.truncate(0);
    assert_eq!(cev.len(), 0);
    assert_eq!( unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len() - 1) as isize);
    assert_eq!(cev, []);

    let mut cev: Cev<u8> = Cev::new();
    cev.reserve(1);
    cev.truncate(0);
    assert_eq!(cev.len(), 0);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len() - 1) as isize);
    assert_eq!(cev, []);

    let mut cev: Cev<u8> = Cev::from([5, 4, 3, 2, 1]);
    cev.truncate(3);
    assert_eq!(cev.len(), 3);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - cev.len()) as isize);
    assert_eq!(cev, [3, 2, 1]);

    let mut cev: Cev<u8> = Cev::from([5, 4, 3, 2, 1]);
    cev.truncate(5);
    assert_eq!(cev.len(), 5);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, 0);
    assert_eq!(cev, [5, 4, 3, 2, 1]);

    let mut cev: Cev<u8> = Cev::from([5, 4, 3, 2, 1]);
    cev.truncate(10);
    assert_eq!(cev.len(), 5);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, 0);
    assert_eq!(cev, [5, 4, 3, 2, 1]);

    let mut cev: Cev<u8> = Cev::from([5, 4, 3, 2, 1]);
    let cap = cev.capacity();
    assert_eq!(cap, 5);
    cev.truncate(0);
    assert_eq!(cev.capacity(), cap);
    assert_eq!(cev.len(), 0);
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - 1) as isize);
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::new();
    cev.truncate(0);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::with_capacity(0);
    cev.truncate(0);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::with_capacity(1);
    cev.truncate(0);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::with_capacity(10);
    cev.truncate(0);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::from([(), (), (), (), ()]);
    let cap = cev.capacity();
    cev.truncate(0);
    assert_eq!(cev.capacity(), cap);
    assert_eq!(cev.len(), 0);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::from([(), (), (), (), ()]);
    let cap = cev.capacity();
    cev.truncate(1);
    assert_eq!(cev.capacity(), cap);
    assert_eq!(cev.len(), 1);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev, [()]);
}
