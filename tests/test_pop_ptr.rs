use cev::Cev;
use std::ptr::NonNull;

#[test]
fn test_extra_pop() {
    let mut cev = Cev::<u8>::from([253]);
    assert_eq!(cev.capacity(), 1);
    assert_eq!(cev.pop(), Some(253));
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );

    let mut cev: Cev<isize> = Cev::new();
    assert_eq!(cev.pop(), None);
    assert_eq!(cev.as_ptr(), NonNull::<isize>::dangling().as_ptr());

    let mut cev = Cev::<()>::from([()]);
    assert_eq!(cev.pop(), Some(()));
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());

    let mut cev = Cev::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(cev.capacity(), 8);
    assert_eq!(cev.pop(), Some(1));
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev.pop(), Some(2));
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev.pop(), Some(3));
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev.pop(), Some(4));
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev.pop(), Some(5));
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev.pop(), Some(6));
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev.pop(), Some(7));
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev.pop(), Some(8));
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );
    assert_eq!(cev.pop(), None);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );
}
