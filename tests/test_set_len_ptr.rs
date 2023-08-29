use cev::Cev;
use std::ptr::NonNull;

#[test]
fn test_set_len_ptr() {
    fn dangling_ptr_eq<T>(cev: &Cev<T>) -> bool {
        cev.as_ptr() == NonNull::<T>::dangling().as_ptr()
    }

    let mut cev: Cev<String> = Cev::new();
    assert_eq!(cev.as_ptr(), NonNull::<String>::dangling().as_ptr());
    assert!(dangling_ptr_eq(&cev));
    unsafe {
        cev.set_len_ptr(0);
    };
    assert!(dangling_ptr_eq(&cev));

    let mut cev: Cev<u8> = Cev::from([1]);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    cev.reserve(8);
    cev.push(4);
    cev.push(3);
    cev.push(2);
    cev.push(1);
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );
    cev.push(8);
    cev.push(7);
    cev.push(6);
    cev.push(5);
    cev.push(4);
    cev.push(3);
    cev.push(2);
    cev.push(1);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );

    let mut cev: Cev<()> = Cev::with_capacity(0);
    assert!(dangling_ptr_eq(&cev));
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(cev.as_ptr(), cev.raw_ptr());

    let mut cev: Cev<()> = Cev::with_capacity(1);
    assert!(dangling_ptr_eq(&cev));
    unsafe {
        cev.set_len_ptr(0);
    };
    assert!(dangling_ptr_eq(&cev));

    let mut cev: Cev<()> = Cev::with_capacity(10);
    assert!(dangling_ptr_eq(&cev));
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(cev.as_ptr(), cev.raw_ptr());

    let mut cev: Cev<Cev<u8>> = Cev::with_capacity(0);
    assert!(dangling_ptr_eq(&cev));
    unsafe {
        cev.set_len_ptr(0);
    };
    assert!(dangling_ptr_eq(&cev));

    let mut cev: Cev<Cev<u8>> = Cev::with_capacity(1);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(cev.as_ptr(), cev.raw_ptr());

    let mut cev: Cev<Cev<u8>> = Cev::with_capacity(10);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );

    let mut cev: Cev<u8> = Cev::with_capacity(4);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );
    cev.push(4);
    cev.push(3);
    cev.push(2);
    cev.push(1);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );
}
