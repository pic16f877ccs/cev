use cev::Cev;
use std::any::Any;
use std::ptr::NonNull;

#[test]
fn test_extra_append() {
    let mut cev: Cev<u8> = Cev::new();
    let mut cev_apd: Cev<u8> = Cev::new();
    cev.append(&mut cev_apd);
    assert_eq!(cev.as_ptr(), NonNull::<u8>::dangling().as_ptr());
    assert_eq!(cev_apd.as_ptr(), NonNull::<u8>::dangling().as_ptr());
    assert_eq!(cev.as_ptr(), Cev::new().as_ptr());
    assert_eq!(cev_apd.as_ptr(), Cev::new().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<u8> = Cev::with_capacity(0);
    let mut cev_apd: Cev<u8> = Cev::with_capacity(0);
    cev.append(&mut cev_apd);
    assert_eq!(cev.as_ptr(), NonNull::<u8>::dangling().as_ptr());
    assert_eq!(cev_apd.as_ptr(), NonNull::<u8>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<u8> = Cev::with_capacity(1);
    let mut cev_apd: Cev<u8> = Cev::with_capacity(0);
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len() - 1) as isize
    );
    assert_eq!(cev_apd.as_ptr(), NonNull::<u8>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<u8> = Cev::with_capacity(1);
    let mut cev_apd: Cev<u8> = Cev::with_capacity(1);
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len() - 1) as isize
    );
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len() - 1) as isize
    );
    assert_eq!(cev, []);

    let mut cev: Cev<u8> = Cev::with_capacity(0);
    let mut cev_apd: Cev<u8> = Cev::with_capacity(1);
    cev.append(&mut cev_apd);
    assert_eq!(cev.as_ptr(), NonNull::<u8>::dangling().as_ptr());
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len() - 1) as isize
    );
    assert_eq!(cev, []);

    let mut cev: Cev<u8> = Cev::new();
    cev.reserve(10);
    let mut cev_apd: Cev<u8> = Cev::new();
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len() - 1) as isize
    );
    assert_eq!(cev_apd.as_ptr(), Cev::new().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<u8> = Cev::with_capacity(0);
    let mut cev_apd: Cev<u8> = Cev::new();
    cev.append(&mut cev_apd);
    assert_eq!(cev.as_ptr(), NonNull::<u8>::dangling().as_ptr());
    assert_eq!(cev_apd.as_ptr(), NonNull::<u8>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<u8> = Cev::new();
    let mut cev_apd: Cev<u8> = Cev::with_capacity(0);
    cev.append(&mut cev_apd);
    assert_eq!(cev.as_ptr(), NonNull::<u8>::dangling().as_ptr());
    assert_eq!(cev_apd.as_ptr(), NonNull::<u8>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::new();
    let mut cev_apd: Cev<()> = Cev::with_capacity(0);
    cev.append(&mut cev_apd);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev_apd.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::new();
    let mut cev_apd: Cev<()> = Cev::new();
    cev.append(&mut cev_apd);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev_apd.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::with_capacity(0);
    let mut cev_apd: Cev<()> = Cev::with_capacity(0);
    cev.append(&mut cev_apd);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev_apd.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::with_capacity(1);
    let mut cev_apd: Cev<()> = Cev::with_capacity(0);
    cev.append(&mut cev_apd);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev_apd.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::with_capacity(1);
    let mut cev_apd: Cev<()> = Cev::with_capacity(1);
    cev.append(&mut cev_apd);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev_apd.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::with_capacity(10);
    let mut cev_apd: Cev<()> = Cev::with_capacity(10);
    cev.append(&mut cev_apd);
    assert_eq!(cev.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev_apd.as_ptr(), NonNull::<()>::dangling().as_ptr());
    assert_eq!(cev, []);

    let mut cev: Cev<u8> = Cev::with_capacity(10);
    cev.push(5);
    cev.push(4);
    cev.push(3);
    cev.push(2);
    cev.push(1);
    let mut cev_apd: Cev<u8> = Cev::with_capacity(10);
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - 1) as isize
    );
    assert_eq!(cev, [1, 2, 3, 4, 5]);

    let mut cev: Cev<u8> = Cev::with_capacity(10);
    cev.push(5);
    cev.push(4);
    cev.push(3);
    cev.push(2);
    cev.push(1);
    let mut cev_apd: Cev<u8> = Cev::with_capacity(0);
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev_apd.as_ptr(), Cev::with_capacity(0).as_ptr());
    assert_eq!(cev, [1, 2, 3, 4, 5]);

    let mut cev: Cev<u8> = Cev::with_capacity(5);
    cev.push(10);
    cev.push(9);
    cev.push(8);
    cev.push(7);
    cev.push(6);
    let mut cev_apd: Cev<u8> = Cev::with_capacity(5);
    cev_apd.push(5);
    cev_apd.push(4);
    cev_apd.push(3);
    cev_apd.push(2);
    cev_apd.push(1);
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len() - 1) as isize
    );
    assert_eq!(cev, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(cev_apd, Cev::new());
    cev_apd.push(5);
    cev_apd.push(4);
    cev_apd.push(3);
    cev_apd.push(2);
    cev_apd.push(1);
    assert_eq!(cev_apd, [1, 2, 3, 4, 5]);

    let mut cev: Cev<u8> = Cev::new();
    cev.reserve(5);
    cev.push(10);
    cev.push(9);
    cev.push(8);
    cev.push(7);
    cev.push(6);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    let mut cev_apd: Cev<u8> = Cev::new();
    cev_apd.reserve(5);
    cev_apd.push(5);
    cev_apd.push(4);
    cev_apd.push(3);
    cev_apd.push(2);
    cev_apd.push(1);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len() - 1) as isize
    );

    let mut cev: Cev<u8> = Cev::from([6, 7, 8, 9, 10]);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    let mut cev_apd: Cev<u8> = Cev::from([1, 2, 3, 4, 5]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len() - 1) as isize
    );

    let mut cev: Cev<u8> = Cev::from([1, 2, 3, 4, 5]);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    let mut cev_apd: Cev<u8> = Cev::from([]);
    assert_eq!(cev_apd.as_ptr(), Cev::from([]).as_ptr());
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev, [1, 2, 3, 4, 5]);
    assert_eq!(cev_apd.as_ptr(), Cev::from([]).as_ptr());

    let mut cev: Cev<u8> = Cev::from([1]);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    let mut cev_apd: Cev<u8> = Cev::from([]);
    assert_eq!(cev_apd.as_ptr(), Cev::from([]).as_ptr());
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev, [1]);
    assert_eq!(cev_apd.as_ptr(), Cev::from([]).as_ptr());

    let mut cev: Cev<u8> = Cev::from([]);
    assert_eq!(cev.as_ptr(), Cev::from([]).as_ptr());
    let mut cev_apd: Cev<u8> = Cev::from([]);
    assert_eq!(cev_apd.as_ptr(), Cev::from([]).as_ptr());
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(cev, []);
    assert_eq!(cev_apd.as_ptr(), Cev::from([]).as_ptr());

    let mut cev: Cev<String> = Cev::from([]);
    assert_eq!(cev.as_ptr(), Cev::from([]).as_ptr());
    let mut cev_apd: Cev<String> = Cev::from([]);
    assert_eq!(cev_apd.as_ptr(), Cev::from([]).as_ptr());
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );
    assert_eq!(cev, Cev::<String>::new());
    assert_eq!(cev_apd, Cev::<String>::new());

    let mut cev: Cev<String> = Cev::from(["one".to_string()]);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    let mut cev_apd: Cev<String> = Cev::from([]);
    assert_eq!(cev_apd.as_ptr(), Cev::from([]).as_ptr());
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );
    assert_eq!(cev, ["one"]);
    assert_eq!(cev_apd, Cev::<String>::new());

    let mut cev: Cev<String> = Cev::from(["one".to_string()]);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    let mut cev_apd: Cev<String> = Cev::from(["two".to_string()]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len() - 1) as isize
    );
    assert_eq!(cev, ["two", "one"]);
    assert_eq!(cev_apd, Cev::<String>::new());
    cev_apd.push("three".to_string());
    cev_apd.push("two".to_string());
    cev_apd.push("one".to_string());
    assert_eq!(cev_apd, ["one", "two", "three"]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );

    let mut cev: Cev<String> = Cev::from(["one".to_string()]);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    let mut cev_apd: Cev<String> = Cev::from(vec!["two".to_string()]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len() - 1) as isize
    );
    assert_eq!(cev, ["two", "one"]);
    assert_eq!(cev_apd, Cev::<String>::new());
    cev_apd.push("three".to_string());
    cev_apd.push("two".to_string());
    cev_apd.push("one".to_string());
    assert_eq!(cev_apd, ["one", "two", "three"]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );

    let mut cev: Cev<String> = Cev::from(vec!["one".to_string()]);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    let mut cev_apd: Cev<String> = Cev::from(vec!["two".to_string()]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len() - 1) as isize
    );
    assert_eq!(cev, ["two", "one"]);
    assert_eq!(cev_apd, Cev::<String>::new());
    cev_apd.push("three".to_string());
    cev_apd.push("two".to_string());
    cev_apd.push("one".to_string());
    assert_eq!(cev_apd, ["one", "two", "three"]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );

    let mut cev: Cev<String> = Cev::from(vec!["one".to_string()]);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    let mut cev_apd: Cev<String> = Cev::from(["two".to_string()]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len() - 1) as isize
    );
    assert_eq!(cev, ["two", "one"]);
    assert_eq!(cev_apd, Cev::<String>::new());
    cev_apd.push("three".to_string());
    cev_apd.push("two".to_string());
    cev_apd.push("one".to_string());
    assert_eq!(cev_apd, ["one", "two", "three"]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );

    let mut cev: Cev<String> = Cev::from_vec(vec!["one".to_string()]);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    let mut cev_apd: Cev<String> = Cev::from(["two".to_string()]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len() - 1) as isize
    );
    assert_eq!(cev, ["two", "one"]);
    assert_eq!(cev_apd, Cev::<String>::new());
    cev_apd.push("three".to_string());
    cev_apd.push("two".to_string());
    cev_apd.push("one".to_string());
    assert_eq!(cev_apd, ["one", "two", "three"]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );

    let mut cev: Cev<String> = Cev::from_vec(vec!["one".to_string()]);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    let mut cev_apd: Cev<String> = Cev::from_vec(vec!["two".to_string()]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );
    cev.append(&mut cev_apd);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - cev.len()) as isize
    );
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len() - 1) as isize
    );
    assert_eq!(cev, ["two", "one"]);
    assert_eq!(cev_apd, Cev::<String>::new());
    cev_apd.push("three".to_string());
    cev_apd.push("two".to_string());
    cev_apd.push("one".to_string());
    assert_eq!(cev_apd, ["one", "two", "three"]);
    assert_eq!(
        unsafe { cev_apd.as_ptr().offset_from(cev_apd.raw_ptr()) },
        (cev_apd.capacity() - cev_apd.len()) as isize
    );

    trait Com {}
    impl Com for u8 {}
    impl Com for u16 {}

    let mut cev_dyn: Cev<Box<dyn Com>> = Cev::from([
        Box::new(128u8) as Box<dyn Com>,
        Box::new(1024u16) as Box<dyn Com>,
    ]);
    let mut cev = Cev::with_capacity(2);
    cev.append(&mut cev_dyn);
    assert!(cev_dyn.is_empty());

    let mut cev = Cev::with_capacity(2);
    let mut cev_dyn_any: Cev<&dyn Any> = Cev::from([&128u8 as &dyn Any, &1024u16 as &dyn Any]);
    let cev_idx_one = cev_dyn_any[0].downcast_ref::<u8>().unwrap();
    let cev_idx_two = cev_dyn_any[1].downcast_ref::<u16>().unwrap();
    assert_eq!(*cev_idx_one, 128u8);
    assert_eq!(*cev_idx_two, 1024u16);
    cev.append(&mut cev_dyn_any);
    let cev_idx_one = cev[0].downcast_ref::<u8>().unwrap();
    let cev_idx_two = cev[1].downcast_ref::<u16>().unwrap();
    assert_eq!(*cev_idx_one, 128u8);
    assert_eq!(*cev_idx_two, 1024u16);
}
