use crate::raw_cev::*;

struct ZST;

fn zst_sanity<T>(v: &RawCev<T>) {
    assert_eq!(v.capacity(), usize::MAX);
    assert_eq!(v.ptr(), core::ptr::NonNull::<T>::dangling().as_ptr());
    assert_eq!(v.allocated_memory(), None);
}

#[test]
fn zst() {
    let cap_err = Err(TryReserveErrorKind::CapacityOverflow.into());

    assert_eq!(std::mem::size_of::<ZST>(), 0);

    let v: RawCev<ZST> = RawCev::new();
    zst_sanity(&v);

    let v: RawCev<ZST> = RawCev::with_capacity(100);
    zst_sanity(&v);

    let v: RawCev<ZST> = RawCev::with_capacity(100);
    zst_sanity(&v);

    let v: RawCev<ZST> = RawCev::allocate(0);
    zst_sanity(&v);

    let v: RawCev<ZST> = RawCev::allocate(100);
    zst_sanity(&v);

    let mut v: RawCev<ZST> = RawCev::allocate(usize::MAX);
    zst_sanity(&v);

    assert_eq!(v.grow_amortized(100, usize::MAX - 100), cap_err);
    assert_eq!(v.grow_amortized(101, usize::MAX - 100), cap_err);
    zst_sanity(&v);
}

