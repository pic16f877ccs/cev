use cev::Cev;
use core::fmt::Debug;
use core::mem::size_of;
use std::rc::Rc;
use std::ptr::NonNull;
//use core::cell::Cell;
//use std::panic::catch_unwind;

struct DropCounter<'a> {
    count: &'a mut u32,
}

impl Drop for DropCounter<'_> {
    fn drop(&mut self) {
        *self.count += 1;
    }
}

#[test]
fn test_into_vec() {
    let c: Cev<String> = [
        "c".to_string(),
        "l".to_string(),
        "o".to_string(),
        "n".to_string(),
        "e".to_string(),
    ]
    .into();
    let mut cev = Cev::with_capacity(5);
    cev.push("e".to_string());
    cev.push("n".to_string());
    cev.push("o".to_string());
    cev.push("l".to_string());
    cev.push("c".to_string());
    assert_eq!(c, cev.into_vec());
    let mut cev = Cev::with_capacity(4);
    cev.push("e".to_string());
    cev.push("n".to_string());
    cev.push("o".to_string());
    assert_eq!(&c[2..], cev.into_vec());

    let empty_arr = [(), (), ()];
    let empty_cev: Cev<u8> = Cev::new();
    assert_eq!(empty_cev.into_vec(), Vec::new());

    let mut c: Cev<()> = Cev::with_capacity(4);
    c.push(());
    c.push(());
    c.push(());
    assert_eq!(c.into_vec(), empty_arr);
}

#[test]
fn test_clone() {
    let mut scev = Cev::with_capacity(4);
    scev.push("clone".to_string());
    scev.push("from".to_string());
    scev.push("cev".to_string());
    let ccev = scev.clone();
    assert_eq!(scev.len(), ccev.len());
    assert_ne!(scev.capacity(), ccev.capacity());
    assert_eq!(scev, ccev);
    assert_ne!(scev.as_ptr(), ccev.as_ptr());
    assert_ne!(scev[0].as_ptr(), ccev[0].as_ptr());
    assert_ne!(scev[1].as_ptr(), ccev[1].as_ptr());
    assert_ne!(scev[2].as_ptr(), ccev[2].as_ptr());

    scev.push("len eq cap".to_string());
    let ecev = scev.clone();
    assert_eq!(scev.len(), ecev.len());
    assert_eq!(scev.capacity(), ecev.capacity());
    assert_eq!(scev, ecev);
    assert_ne!(scev.as_ptr(), ecev.as_ptr());
    assert_ne!(scev[0].as_ptr(), ecev[0].as_ptr());
    assert_ne!(scev[1].as_ptr(), ecev[1].as_ptr());
    assert_ne!(scev[2].as_ptr(), ecev[2].as_ptr());

    let empty_cev: Cev<u8> = Cev::new();
    assert_eq!(empty_cev.clone(), empty_cev);

    let mut c: Cev<()> = Cev::with_capacity(4);
    c.push(());
    c.push(());
    c.push(());
    assert_eq!(c, c.clone());
}

#[test]
fn test_small_vec_struct() {
    assert_eq!(size_of::<Cev<u8>>(), size_of::<usize>() * 4);
}

#[test]
fn test_double_drop() {
    struct TwoCev<T> {
        x: Cev<T>,
        y: Cev<T>,
    }

    let (mut count_x, mut count_y) = (0, 0);
    {
        let mut tv = TwoCev {
            x: Cev::new(),
            y: Cev::new(),
        };
        tv.x.push(DropCounter {
            count: &mut count_x,
        });
        tv.y.push(DropCounter {
            count: &mut count_y,
        });

        drop(tv.x);
    }

    assert_eq!(count_x, 1);
    assert_eq!(count_y, 1);
}

#[test]
fn test_indexing() {
    let v: Cev<isize> = Cev::from([10, 20]);
    assert_eq!(v[0], 10);
    assert_eq!(v[1], 20);
    let mut x: usize = 0;
    assert_eq!(v[x], 10);
    assert_eq!(v[x + 1], 20);
    x = x + 1;
    assert_eq!(v[x], 20);
    assert_eq!(v[x - 1], 10);
}

#[test]
fn test_debug_fmt() {
    let vec1: Cev<isize> = Cev::from([]);
    assert_eq!("[]", format!("{:?}", vec1));

    let vec2 = Cev::from([0, 1]);
    assert_eq!("[0, 1]", format!("{:?}", vec2));
}

#[test]
fn test_push() {
    let mut v = Cev::from([]);
    v.push(1);
    assert_eq!(v, [1]);
    v.push(2);
    assert_eq!(v, [2, 1]);
    v.push(3);
    assert_eq!(v, [3, 2, 1]);
}

#[test]
fn test_from_cev() {
    let cev = Cev::from(["1".to_string(), "2".to_string(), "3".to_string()]);
    let vec: Vec<String> = cev.into();
    assert_eq!(vec, ["1", "2", "3"]);

    let mut cev = Cev::with_capacity(4);
    cev.push(1_i128);
    cev.push(2_i128);
    cev.push(3_i128);
    let c_cev = cev.clone();
    let vec: Vec<i128> = cev.into();
    assert_eq!(vec, c_cev);

}

fn test_ptr_pos<T>(cev: &Cev<T>) -> bool {
    if cev.capacity() == usize::MAX {
        cev.as_ptr() == NonNull::<T>::dangling().as_ptr()
    } else if cev.capacity() == 0 {
        //cev.as_ptr() == cev.raw_ptr()
        cev.as_ptr() == NonNull::<T>::dangling().as_ptr()
    } else if cev.len() == 0 {
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) == ((cev.capacity() - 1) as isize) }
    } else {
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) == ((cev.capacity() - cev.len()) as isize) }
    }
}

#[test]
fn test_from_vec() {
    let vec = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let vec_cap = vec.capacity();
    let vec_ptr = vec.as_ptr();
    let cev = Cev::from_vec(vec);
    assert_eq!(cev, ["1", "2", "3"]);
    assert_eq!(cev.capacity(), vec_cap);
    assert_eq!(cev.as_ptr(), vec_ptr);
    assert_eq!(cev.raw_ptr(), vec_ptr);
    assert!(test_ptr_pos(&cev));

    let mut vec = Vec::with_capacity(4);
    let vec_ptr = vec.as_ptr();
    vec.push(1_i128);
    let cev: Cev<i128> = Cev::from_vec(vec);
    assert_ne!(cev.as_ptr(), vec_ptr);
    assert_eq!(cev.raw_ptr(), vec_ptr);
    assert!(test_ptr_pos(&cev));
    assert_eq!(cev, [1]);

    let cev = Cev::from_vec(vec![1]);
    assert!(test_ptr_pos(&cev));
    assert_eq!(cev, [1]);

    let cev: Cev<()> = Cev::from_vec(vec![()]);
    assert!(test_ptr_pos(&cev));
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    assert_eq!(cev, [()]);

    let vec = Vec::from([(), (), (), ()]);
    let cev = Cev::from_vec(vec);
    assert!(test_ptr_pos(&cev));
    assert_eq!(cev, [(), (), (), ()]);

    let cev: Cev<()> = Cev::from_vec(Vec::new());
    assert!(test_ptr_pos(&cev));
    assert_eq!(cev, Cev::<()>::new());

    let cev: Cev<&str> = Cev::from_vec(Vec::new());
    assert!(test_ptr_pos(&cev));
    assert_eq!(cev, Cev::<&str>::new());

    let cev: Cev<u8> = Cev::from_vec(Vec::new());
    assert!(test_ptr_pos(&cev));
    assert_eq!(cev, Cev::<u8>::new());

    let cev: Cev<u8> = Cev::from_vec(Vec::with_capacity(1));
    assert!(test_ptr_pos(&cev));
    assert_eq!(cev, Cev::<u8>::new());

    let cev: Cev<()> = Cev::from_vec(Vec::with_capacity(1));
    assert!(test_ptr_pos(&cev));
    assert_eq!(cev, Cev::<()>::new());

    let cev: Cev<()> = Cev::from_vec(Vec::with_capacity(usize::MAX));
    assert!(test_ptr_pos(&cev));
    assert_eq!(cev, Cev::<()>::new());
}

#[test]
fn test_slice_from_ref() {
    let values = Cev::from([1, 2, 3, 4, 5]);
    let slice = &values[1..3];

    assert_eq!(slice, [2, 3]);
}

#[test]
fn test_slice_from_mut() {
    let mut cev: Cev<i32> = [1, 2, 3, 4, 5].into();
    {
        let slice = &mut cev[2..];
        assert!(slice == [3, 4, 5]);
        for ptr in slice {
            *ptr += 2;
        }
    }

    assert!(cev == [1, 2, 5, 6, 7]);
}

#[test]
fn test_slice_to_mut() {
    let mut cev: Cev<u16> = [1, 2, 3, 4, 5].into();
    {
        let slice = &mut cev[..2];
        assert!(slice == [1, 2]);
        for p in slice {
            *p += 1;
        }
    }

    assert!(cev == [2, 3, 3, 4, 5]);
}

#[test]
fn test_cmp() {
    let x: Cev<isize> = vec![1, 2, 3, 4, 5].into();
    let cmp: &[isize] = &[1, 2, 3, 4, 5];
    assert_eq!(&x[..], cmp);
    let cmp: &[isize] = &[3, 4, 5];
    assert_eq!(&x[2..], cmp);
    let cmp: &[isize] = &[1, 2, 3];
    assert_eq!(&x[..3], cmp);
    let cmp: &[isize] = &[2, 3, 4];
    assert_eq!(&x[1..4], cmp);
}

#[test]
fn test_index() {
    let cev: Cev<i32> = vec![1, 2, 3].into();
    assert!(cev[1] == 2);
}

#[test]
#[should_panic]
fn test_index_out_of_bounds() {
    let cev: Cev<i32> = vec![1, 2, 3].into();
    let _ = cev[3];
}

#[test]
#[should_panic]
fn test_slice_out_of_bounds_1() {
    let c = Cev::from([1, 2, 3, 4, 5]);
    let _ = &c[!0..];
}

#[test]
#[should_panic]
fn test_slice_out_of_bounds_2() {
    let x = Cev::from([1, 2, 3, 4, 5]);
    let _ = &x[..6];
}

#[test]
#[should_panic]
fn test_slice_out_of_bounds_3() {
    let x = Cev::from([1, 2, 3, 4, 5]);
    let _ = &x[!0..4];
}

#[test]
#[should_panic]
fn test_slice_out_of_bounds_4() {
    let x = Cev::from([1, 2, 3, 4, 5]);
    let _ = &x[1..6];
}

#[test]
#[should_panic]
fn test_slice_out_of_bounds_5() {
    let x = Cev::from([1, 2, 3, 4, 5]);
    let _ = &x[3..2];
}

#[test]
fn test_into_iter_count() {
    assert_eq!(Cev::from([1, 2, 3]).into_iter().count(), 3);
}

#[test]
fn test_pop() {
    let mut cev = Cev::<u8>::from([253]);
    assert_eq!(cev.capacity(), 1);
    assert_eq!(cev.pop(), Some(253));
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - 1) as isize);
}

#[test]
fn zero_sized_values() {
    let mut v = Cev::new();
    assert_eq!(v.len(), 0);
    v.push(());
    assert_eq!(v.len(), 1);
    v.push(());

    assert_eq!(v.len(), 2);
    assert_eq!(v.pop(), Some(()));
    assert_eq!(v.pop(), Some(()));
    assert_eq!(v.pop(), None);

    unsafe {
        v.set_len(0);
    }

    assert_eq!(v, []);
    assert_eq!(v.iter().count(), 0);
    v.push(());
    assert_eq!(v.iter().count(), 1);
    v.push(());
    assert_eq!(v.iter().count(), 2);

    for &() in &v {}

    assert_eq!(v.iter_mut().count(), 2);
    v.push(());
    assert_eq!(v.iter_mut().count(), 3);
    v.push(());
    assert_eq!(v.iter_mut().count(), 4);

    for &mut () in &mut v {}
}

#[test]
fn test_zst_capacity() {
    assert_eq!(Cev::<()>::new().capacity(), usize::MAX);
}

#[test]
fn test_zero_sized_cev_push() {
    const N: usize = 8;

    for len in 0..N {
        let mut cev = Cev::with_capacity(len);
        assert_eq!(cev.len(), 0);
        assert!(cev.capacity() >= len);
        for _ in 0..len {
            cev.push(());
        }
        assert_eq!(cev.len(), len);
        assert_eq!(cev.iter().count(), len);
        cev.clear();
    }
}

#[test]
fn test_zero_sized_capacity() {
    for len in [0, 1, 2, 4, 8, 16, 32, 64, 128, 256] {
        let cev = Cev::<()>::with_capacity(len);
        assert_eq!(cev.len(), 0);
        assert_eq!(cev.capacity(), usize::MAX);
    }
}

#[test]
fn test_move_items_zero_sized() {
    let c = Cev::from([(), (), ()]);
    let mut c2 = Cev::new();
    for i in c {
        c2.push(i);
    }
    assert_eq!(c2, [(), (), ()]);
}

macro_rules! assert_partial_eq_valid {
    ($a2:expr, $a3:expr; $b2:expr, $b3: expr) => {
        assert!($a2 == $b2);
        assert!($a2 != $b3);
        assert!($a3 != $b2);
        assert!($a3 == $b3);
        assert_eq!($a2, $b2);
        assert_ne!($a2, $b3);
        assert_ne!($a3, $b2);
        assert_eq!($a3, $b3);
    };
}

macro_rules! generate_assert_eq_vec_and_prim {
    ($name:ident<$B:ident>($type:ty)) => {
        fn $name<A: PartialEq<$B> + Debug, $B: Debug>(a: Cev<A>, b: $type) {
            assert!(a == b);
            assert_eq!(a, b);
        }
    };
}

generate_assert_eq_vec_and_prim! { assert_eq_vec_and_slice  <B>(&[B])   }
generate_assert_eq_vec_and_prim! { assert_eq_vec_and_array_3<B>([B; 3]) }

#[test]
fn partialeq_vec_and_prim() {
    assert_eq_vec_and_slice(Cev::from([1, 2, 3]), &[1, 2, 3]);
    assert_eq_vec_and_array_3(Cev::from([1, 2, 3]), [1, 2, 3]);
}

#[test]
fn partialeq_vec_full() {
    let vec2: Cev<_> = Cev::from([1, 2]);
    let vec3: Cev<_> = Cev::from([1, 2, 3]);
    let slice2: &[_] = &[1, 2];
    let slice3: &[_] = &[1, 2, 3];
    let slicemut2: &[_] = &mut [1, 2];
    let slicemut3: &[_] = &mut [1, 2, 3];
    let array2: [_; 2] = [1, 2];
    let array3: [_; 3] = [1, 2, 3];
    let arrayref2: &[_; 2] = &[1, 2];
    let arrayref3: &[_; 3] = &[1, 2, 3];

    assert_partial_eq_valid!(vec2,vec3; vec2,vec3);
    assert_partial_eq_valid!(vec2,vec3; slice2,slice3);
    assert_partial_eq_valid!(vec2,vec3; slicemut2,slicemut3);
    assert_partial_eq_valid!(slice2,slice3; vec2,vec3);
    assert_partial_eq_valid!(slicemut2,slicemut3; vec2,vec3);
    assert_partial_eq_valid!(vec2,vec3; array2,array3);
    assert_partial_eq_valid!(vec2,vec3; arrayref2,arrayref3);
    assert_partial_eq_valid!(vec2,vec3; arrayref2[..],arrayref3[..]);
}

#[test]
fn test_push_growth_strategy() {
    // If the element size is 1, we jump from 0 to 8, then double.
    {
        let mut v1: Cev<u8> = Cev::new();
        assert_eq!(v1.capacity(), 0);

        for _ in 0..8 {
            v1.push(0);
            assert_eq!(v1.capacity(), 8);
        }

        for _ in 8..16 {
            v1.push(0);
            assert_eq!(v1.capacity(), 16);
        }

        for _ in 16..32 {
            v1.push(0);
            assert_eq!(v1.capacity(), 32);
        }

        for _ in 32..64 {
            v1.push(0);
            assert_eq!(v1.capacity(), 64);
        }
    }

    // If the element size is 2..=1024, we jump from 0 to 4, then double.
    {
        let mut v2: Cev<u16> = Cev::new();
        let mut v1024: Cev<[u8; 1024]> = Cev::new();
        assert_eq!(v2.capacity(), 0);
        assert_eq!(v1024.capacity(), 0);

        for _ in 0..4 {
            v2.push(0);
            v1024.push([0; 1024]);
            assert_eq!(v2.capacity(), 4);
            assert_eq!(v1024.capacity(), 4);
        }

        for _ in 4..8 {
            v2.push(0);
            v1024.push([0; 1024]);
            assert_eq!(v2.capacity(), 8);
            assert_eq!(v1024.capacity(), 8);
        }

        for _ in 8..16 {
            v2.push(0);
            v1024.push([0; 1024]);
            assert_eq!(v2.capacity(), 16);
            assert_eq!(v1024.capacity(), 16);
        }

        for _ in 16..32 {
            v2.push(0);
            v1024.push([0; 1024]);
            assert_eq!(v2.capacity(), 32);
            assert_eq!(v1024.capacity(), 32);
        }

        for _ in 32..64 {
            v2.push(0);
            v1024.push([0; 1024]);
            assert_eq!(v2.capacity(), 64);
            assert_eq!(v1024.capacity(), 64);
        }
    }

    // If the element size is > 1024, we jump from 0 to 1, then double.
    {
        let mut v1025: Cev<[u8; 1025]> = Cev::new();
        assert_eq!(v1025.capacity(), 0);

        for _ in 0..1 {
            v1025.push([0; 1025]);
            assert_eq!(v1025.capacity(), 1);
        }

        for _ in 1..2 {
            v1025.push([0; 1025]);
            assert_eq!(v1025.capacity(), 2);
        }

        for _ in 2..4 {
            v1025.push([0; 1025]);
            assert_eq!(v1025.capacity(), 4);
        }

        for _ in 4..8 {
            v1025.push([0; 1025]);
            assert_eq!(v1025.capacity(), 8);
        }

        for _ in 8..16 {
            v1025.push([0; 1025]);
            assert_eq!(v1025.capacity(), 16);
        }

        for _ in 16..32 {
            v1025.push([0; 1025]);
            assert_eq!(v1025.capacity(), 32);
        }

        for _ in 32..64 {
            v1025.push([0; 1025]);
            assert_eq!(v1025.capacity(), 64);
        }
    }
}

#[test]
fn vec_macro_repeating_null_raw_fat_pointer() {
    let raw_dyn = &mut (|| ()) as &mut dyn Fn() as *mut dyn Fn();
    let vtable = dbg!(ptr_metadata(raw_dyn));
    let null_raw_dyn = ptr_from_raw_parts(std::ptr::null_mut(), vtable);
    assert!(null_raw_dyn.is_null());

    let vec = Cev::from([null_raw_dyn; 1]);
    dbg!(ptr_metadata(vec[0]));
    assert!(vec[0] == null_raw_dyn);

    fn ptr_metadata(ptr: *mut dyn Fn()) -> *mut () {
        unsafe { std::mem::transmute::<*mut dyn Fn(), DynRepr>(ptr).vtable }
    }

    fn ptr_from_raw_parts(data: *mut (), vtable: *mut ()) -> *mut dyn Fn() {
        unsafe { std::mem::transmute::<DynRepr, *mut dyn Fn()>(DynRepr { data, vtable }) }
    }

    #[repr(C)]
    struct DynRepr {
        data: *mut (),
        vtable: *mut (),
    }
}

#[test]
fn test_from_iter_specialization_head_tail_drop() {
    let drop_count: Cev<_> = (0..=2).map(|_| Rc::new(())).collect();
    let src: Cev<_> = drop_count.iter().cloned().collect();
    let iter = src.into_iter();
    let sink: Cev<_> = iter.skip(1).take(1).collect();
    assert_eq!(Rc::strong_count(&drop_count[0]), 1, "front was dropped");
    assert_eq!(
        Rc::strong_count(&drop_count[1]),
        2,
        "one element was collected"
    );
    assert_eq!(Rc::strong_count(&drop_count[2]), 1, "tail was dropped");
    assert_eq!(sink.len(), 1);
}

#[test]
fn test_into_iter_clone() {
    fn iter_equal<I: Iterator<Item = i32>>(it: I, slice: &[i32]) {
        let v: Cev<i32> = it.collect();
        assert_eq!(&v[..], slice);
    }
    let mut it = [1, 2, 3].into_iter();
    iter_equal(it.clone(), &[1, 2, 3]);
    assert_eq!(it.next(), Some(1));
    let mut it = it.rev();
    iter_equal(it.clone(), &[3, 2]);
    assert_eq!(it.next(), Some(3));
    iter_equal(it.clone(), &[2]);
    assert_eq!(it.next(), Some(2));
    iter_equal(it.clone(), &[]);
    assert_eq!(it.next(), None);
}

#[test]
fn test_into_iter_as_mut_slice() {
    let c = Cev::from(['a', 'b', 'c']);
    let mut into_iter = c.into_iter();
    assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
    into_iter.as_mut_slice()[0] = 'x';
    into_iter.as_mut_slice()[1] = 'y';
    assert_eq!(into_iter.next().unwrap(), 'x');
    assert_eq!(into_iter.as_slice(), &['y', 'c']);
}

#[test]
fn test_into_iter_as_slice() {
    let c: Cev<char> = ['a', 'b', 'c'].into();
    let mut into_iter = c.into_iter();
    assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
    let _ = into_iter.next().unwrap();
    assert_eq!(into_iter.as_slice(), &['b', 'c']);
    let _ = into_iter.next().unwrap();
    let _ = into_iter.next().unwrap();
    assert_eq!(into_iter.as_slice(), &[]);
}

#[test]
fn test_move_items_reverse() {
    let cev = Cev::from([1, 2, 3]);
    let mut cev2 = Cev::from([]);
    for i in cev.into_iter().rev() {
        cev2.push(i);
    }
    assert_eq!(cev2, [1, 2, 3]);
}

#[test]
fn test_cev_truncate_with_cap() {
    let mut truncate = Cev::with_capacity(10);
    truncate.push("trun".to_string());
    truncate.push("cate".to_string());
    truncate.push("test".to_string());
    truncate.truncate(1);
    assert_eq!(truncate, ["trun"]);
    truncate.truncate(0);
    assert!(truncate.is_empty());
    truncate.push("trun".to_string());
    truncate.push("cate".to_string());
    truncate.push("test".to_string());
    assert_eq!(truncate, ["test", "cate", "trun"]);
}

#[test]
fn test_cev_truncate_zst() {
    let mut truncate = Cev::new();
    truncate.push(());
    truncate.push(());
    truncate.push(());
    truncate.truncate(1);
    assert_eq!(truncate, [()]);
    truncate.truncate(0);
    assert!(truncate.is_empty());
    truncate.push(());
    truncate.push(());
    truncate.push(());
    assert_eq!(truncate, [(), (), ()]);
}

#[test]
fn test_cev_truncate_drop() {
    static mut DROPS: u32 = 0;
    struct Elem(i32);
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    let mut v = Cev::from([Elem(1), Elem(2), Elem(3), Elem(4), Elem(5)]);
    assert_eq!(unsafe { DROPS }, 0);
    v.truncate(3);
    assert_eq!(unsafe { DROPS }, 2);
    v.truncate(0);
    assert_eq!(unsafe { DROPS }, 5);
}

#[test]
#[should_panic]
fn test_cev_truncate_fail() {
    struct BadElem(i32);
    impl Drop for BadElem {
        fn drop(&mut self) {
            let BadElem(ref mut x) = *self;
            if *x == 0xbadbeef {
                panic!("BadElem panic: 0xbadbeef")
            }
        }
    }

    let mut v = Cev::from([BadElem(1), BadElem(2), BadElem(0xbadbeef), BadElem(4)]);
    v.truncate(0);
}

#[test]
fn test_reserve() {
    let mut v = Cev::new();
    assert_eq!(v.capacity(), 0);

    v.reserve(2);
    assert!(v.capacity() >= 2);

    for i in 0..16 {
        v.push(i);
    }

    assert!(v.capacity() >= 16);
    v.reserve(16);
    assert!(v.capacity() >= 32);

    v.push(16);

    v.reserve(16);
    assert!(v.capacity() >= 33)
}

#[test]
fn test_reserve_ptr() {
    let mut reserve: Cev<u8> = Cev::new();
    reserve.reserve(1);
    assert!(reserve.capacity() >= 1);
    assert_eq!(
        unsafe { reserve.as_ptr().offset_from(reserve.raw_ptr()) as usize },
        reserve.capacity() - 1
    );
    reserve.push(1);
    assert_eq!(
        unsafe { reserve.as_ptr().offset_from(reserve.raw_ptr()) as usize },
        reserve.capacity() - 1
    );
    reserve.push(2);
    assert_eq!(
        unsafe { reserve.as_ptr().offset_from(reserve.raw_ptr()) as usize },
        reserve.capacity() - 2
    );
    reserve.reserve(10);
    assert!(reserve.capacity() >= 10);
    assert_eq!(
        unsafe { reserve.as_ptr().offset_from(reserve.raw_ptr()) as usize },
        reserve.capacity() - 2
    );
    reserve.reserve(20);
    assert_eq!(
        unsafe { reserve.as_ptr().offset_from(reserve.raw_ptr()) as usize },
        reserve.capacity() - 2
    );
    assert!(reserve.capacity() >= 20);
    reserve.reserve(10);
    assert_eq!(
        unsafe { reserve.as_ptr().offset_from(reserve.raw_ptr()) as usize },
        reserve.capacity() - 2
    );
    assert!(reserve.capacity() >= 20);
}

#[test]
fn test_append() {
    let mut vec = Cev::from([4, 5, 6]);
    let mut vec2 = Cev::from([1, 2, 3]);
    vec.append(&mut vec2);
    assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    assert_eq!(vec2, []);
    assert_eq!(
        unsafe { vec2.as_ptr().offset_from(vec2.raw_ptr()) as usize },
        vec2.capacity() - vec2.len() - 1
    );

    let mut cev = Cev::from([3, 4, 5, 6, 7, 8, 9]);
    let mut front_app = Cev::with_capacity(10);
    front_app.push(2);
    front_app.push(1);
    front_app.push(0);

    cev.append(&mut front_app);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) as usize },
        cev.capacity() - cev.len()
    );
    assert_eq!(cev, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_insert() {
    let mut cev = Cev::from([1, 2, 3]);
    cev.insert(0, 0);
    assert_eq!(cev, [0, 1, 2, 3]);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) as usize },
        cev.capacity() - cev.len()
    );
    cev.insert(4, 4);
    assert_eq!(cev, [0, 1, 2, 3, 4]);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) as usize },
        cev.capacity() - cev.len()
    );

    let mut cev = Cev::<u8>::new();
    cev.insert(0, 1);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) as usize },
        cev.capacity() - cev.len()
    );
    assert_eq!(cev, [1]);

    let mut cev = Cev::<u8>::with_capacity(1);
    cev.insert(0, 2);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) as usize },
        cev.capacity() - cev.len()
    );
    assert_eq!(cev, [2]);
}

#[test]
#[should_panic]
fn test_insert_panic() {
    let mut ist = Cev::from([1, 2, 3]);
    ist.insert(5, 5);
}

#[test]
fn test_into_iter_debug() {
    let cev = Cev::from(['a', 'b', 'c']);
    let into_iter = cev.into_iter();
    let debug = format!("{into_iter:?}");
    assert_eq!(debug, "IntoIter(['a', 'b', 'c'])");
}

#[test]
fn test_cev_clear_drop() {
    static mut DROPS: u32 = 0;
    struct Elem(i32);
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    let mut cev = Cev::with_capacity(5);
    let mut cev_three = Cev::from([Elem(1), Elem(2), Elem(3)]);
    let mut cev_five = Cev::from([Elem(1), Elem(2), Elem(3), Elem(4), Elem(5)]);
    assert_eq!(unsafe { DROPS }, 0);
    cev.append(&mut cev_three);
    cev.clear();
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - 1) as isize);
    assert_eq!(unsafe { DROPS }, 3);
    unsafe { DROPS = 0 };
    cev.append(&mut cev_five);
    cev.clear();
    assert_eq!(unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) }, (cev.capacity() - 1) as isize);
    assert_eq!(unsafe { DROPS }, 5);
}

#[test]
fn test_clear() {
    let mut cev: Cev<u8> = Cev::new();
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    cev.clear();
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    assert_eq!(cev, Cev::new());

    let mut cev: Cev<u8> = Cev::with_capacity(0);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    cev.clear();
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    assert_eq!(cev, Cev::new());

    let mut cev: Cev<u8> = Cev::from([1]);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    cev.clear();
    assert_eq!(cev, []);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    cev.reserve(8);
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );
    assert_eq!(cev, []);
    cev.clear();
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );
    assert_eq!(cev, []);
    cev.push(4);
    cev.push(3);
    cev.push(2);
    cev.push(1);
    cev.clear();
    assert_eq!(
        unsafe { cev.as_ptr().offset_from(cev.raw_ptr()) },
        (cev.capacity() - 1) as isize
    );
    assert_eq!(cev, []);

    let mut cev: Cev<()> = Cev::with_capacity(0);
    cev.clear();
    assert_eq!(cev, []);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());

    let mut cev: Cev<()> = Cev::with_capacity(1);
    cev.clear();
    assert_eq!(cev, []);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(cev.as_ptr(), cev.raw_ptr());

    let mut cev: Cev<()> = Cev::with_capacity(10);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    cev.clear();
    assert_eq!(cev, []);
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
}

#[test]
fn test_set_len_ptr() {
    let mut cev: Cev<String> = Cev::new();
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(cev.as_ptr(), cev.raw_ptr());

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
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(cev.as_ptr(), cev.raw_ptr());

    let mut cev: Cev<()> = Cev::with_capacity(1);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(cev.as_ptr(), cev.raw_ptr());

    let mut cev: Cev<()> = Cev::with_capacity(10);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(cev.as_ptr(), cev.raw_ptr());

    let mut cev: Cev<Cev<u8>> = Cev::with_capacity(0);
    assert_eq!(cev.as_ptr(), cev.raw_ptr());
    unsafe {
        cev.set_len_ptr(0);
    };
    assert_eq!(cev.as_ptr(), cev.raw_ptr());

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

    let mut cev: Cev<u8> = Cev::with_capacity(4);
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

//#[test]
//fn test_into_iter_leak() {
//    static mut DROPS: i32 = 0;
//
//    struct D(bool);
//
//    impl Drop for D {
//        fn drop(&mut self) {
//            unsafe {
//                DROPS += 1;
//            }
//
//            if self.0 {
//                panic!("panic in `drop`");
//            }
//        }
//    }
//
//    let v = Cev::from([D(false), D(true), D(false)]);
//
//    catch_unwind(move || drop(v.into_iter())).ok();
//
//    assert_eq!(unsafe { DROPS }, 3);
//}

//#[test]
//fn test_vec_cycle() {
//    #[derive(Debug)]
//    struct C<'a> {
//        v: Cev<Cell<Option<&'a C<'a>>>>,
//    }
//
//    impl<'a> C<'a> {
//        fn new() -> C<'a> {
//            C { v: Cev::new() }
//        }
//    }
//
//    let mut c1 = C::new();
//    let mut c2 = C::new();
//    let mut c3 = C::new();
//
//    // Push
//    c1.v.push(Cell::new(None));
//    c1.v.push(Cell::new(None));
//
//    c2.v.push(Cell::new(None));
//    c2.v.push(Cell::new(None));
//
//    c3.v.push(Cell::new(None));
//    c3.v.push(Cell::new(None));
//
//    // Set
//    c1.v[0].set(Some(&c2));
//    c1.v[1].set(Some(&c3));
//
//    c2.v[0].set(Some(&c2));
//    c2.v[1].set(Some(&c3));
//
//    c3.v[0].set(Some(&c1));
//    c3.v[1].set(Some(&c2));
//}
