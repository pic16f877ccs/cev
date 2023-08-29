use crate::alloc_err::{AllocError, TryReserveError, TryReserveErrorKind};
use core::cmp;
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout, LayoutError};
use std::mem;
use std::ptr::{self, NonNull};

#[cfg(test)]
mod raw_cev_tests;

pub(crate) struct RawCev<T> {
    mov_ptr: NonNull<T>,
    raw_ptr: NonNull<T>,
    cap: usize,
}

unsafe impl<T: Send> Send for RawCev<T> {}
unsafe impl<T: Sync> Sync for RawCev<T> {}

impl<T> RawCev<T> {
    const MIN_NON_ZERO_CAP: usize = if mem::size_of::<T>() == 1 {
        8
    } else if mem::size_of::<T>() <= 1024 {
        4
    } else {
        1
    };

    pub const IS_ZST: bool = is_zst::<T>();
    pub const NEW: Self = Self::new();

    pub const fn new() -> Self {
        Self {
            mov_ptr: NonNull::dangling(),
            raw_ptr: NonNull::dangling(),
            cap: 0,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::allocate(capacity)
    }

    #[inline]
    pub fn ptr(&self) -> *mut T {
        self.mov_ptr.as_ptr()
    }

    #[inline]
    pub unsafe fn mov_ptr(&mut self, ptr: *mut T) {
        self.mov_ptr = NonNull::new_unchecked(ptr);
    }

    #[inline]
    pub unsafe fn mov_ptr_add(&mut self, count: usize) {
        self.mov_ptr = NonNull::new_unchecked(self.ptr().add(count));
    }

    #[inline]
    pub unsafe fn mov_ptr_sub(&mut self, count: usize) {
        self.mov_ptr = NonNull::new_unchecked(self.ptr().sub(count));
    }

    #[inline]
    pub fn raw_ptr(&self) -> *mut T {
        self.raw_ptr.as_ptr()
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        if Self::IS_ZST {
            usize::MAX
        } else {
            self.cap
        }
    }

    #[inline]
    unsafe fn from_raw_parts(mov_ptr: NonNull<T>, raw_ptr: NonNull<T>, cap: usize) -> Self {
        Self {
            mov_ptr,
            raw_ptr,
            cap,
        }
    }

    #[inline]
    pub unsafe fn from_raw_parts_ptr(mov_ptr: *mut T, raw_ptr: *mut T, cap: usize) -> Self {
        RawCev::from_raw_parts(
            NonNull::new_unchecked(mov_ptr),
            NonNull::new_unchecked(raw_ptr),
            cap,
        )
    }

    fn allocate(capacity: usize) -> Self {
        if Self::IS_ZST || capacity == 0 {
            Self::new()
        } else {
            let layout = match Layout::array::<T>(capacity) {
                Ok(layout) => layout,
                Err(_) => capacity_overflow(),
            };

            match alloc_guard(layout.size()) {
                Ok(_) => {}
                Err(_) => capacity_overflow(),
            }

            let ptr = unsafe { alloc_usr(layout) };

            Self {
                mov_ptr: unsafe { NonNull::new_unchecked(ptr.cast::<T>().add(capacity - 1)) },
                raw_ptr: unsafe { NonNull::new_unchecked(ptr.cast::<T>()) },
                cap: capacity,
            }
        }
    }

    fn allocated_memory(&self) -> Option<(NonNull<u8>, NonNull<u8>, Layout)> {
        if Self::IS_ZST || self.cap == 0 {
            None
        } else {
            #[allow(clippy::let_unit_value)]
            let _: () = { assert!(mem::size_of::<T>() % mem::align_of::<T>() == 0) };
            unsafe {
                let align = mem::align_of::<T>();
                let size = mem::size_of::<T>() * self.cap;
                let layout = Layout::from_size_align_unchecked(size, align);
                Some((
                    NonNull::new_unchecked(self.raw_ptr.as_ptr().cast::<u8>()),
                    NonNull::new_unchecked(self.mov_ptr.as_ptr().cast::<u8>()),
                    layout,
                ))
            }
        }
    }

    pub fn reserve_for_push(&mut self, len: usize) {
        handle_reserve(self.grow_amortized(len, 1));
    }

    #[inline]
    pub fn reserve(&mut self, len: usize, additional: usize) {
        #[cold]
        fn do_reserve_and_handle<T>(slf: &mut RawCev<T>, len: usize, additional: usize) {
            handle_reserve(slf.grow_amortized(len, additional));
        }

        if self.needs_to_grow(len, additional) {
            do_reserve_and_handle(self, len, additional);
        }
    }

    fn needs_to_grow(&self, len: usize, additional: usize) -> bool {
        additional > self.capacity().wrapping_sub(len)
    }

    fn grow_amortized(&mut self, len: usize, additional: usize) -> Result<(), TryReserveError> {
        debug_assert!(additional > 0);

        if Self::IS_ZST {
            return Err(TryReserveErrorKind::CapacityOverflow.into());
        }

        let required_cap = len
            .checked_add(additional)
            .ok_or(TryReserveErrorKind::CapacityOverflow)?;
        let cap = cmp::max(self.cap * 2, required_cap);
        let cap = cmp::max(Self::MIN_NON_ZERO_CAP, cap);
        let new_layout = Layout::array::<T>(cap);
        let ptr = finish_increase(new_layout, self.allocated_memory(), mem::size_of::<T>())?;

        self.raw_ptr = ptr.0.cast::<T>();
        self.mov_ptr = ptr.1.cast::<T>();
        self.cap = cap;
        Ok(())
    }
}

#[inline(never)]
fn finish_increase(
    new_layout: Result<Layout, LayoutError>,
    allocated_memory: Option<(NonNull<u8>, NonNull<u8>, Layout)>,
    offset: usize,
) -> Result<(NonNull<u8>, NonNull<u8>), TryReserveError> {
    let new_layout = new_layout.map_err(|_| TryReserveErrorKind::CapacityOverflow)?;

    alloc_guard(new_layout.size())?;

    let memory = if let Some((ptr, mov_ptr, old_layout)) = allocated_memory {
        debug_assert_eq!(old_layout.align(), new_layout.align());
        unsafe { increase((ptr, mov_ptr), old_layout, new_layout) }
    } else {
        unsafe {
            let ptr = alloc_usr(new_layout);
            let mov_ptr = NonNull::new_unchecked(ptr.add(new_layout.size() - offset));
            let raw_ptr = NonNull::new_unchecked(ptr);

            Ok((raw_ptr, mov_ptr))
        }
    };

    memory.map_err(|_| {
        TryReserveErrorKind::AllocError {
            layout: new_layout,
            non_exhaustive: (),
        }
        .into()
    })
}

unsafe fn increase(
    ptr: (NonNull<u8>, NonNull<u8>),
    old_layout: Layout,
    new_layout: Layout,
) -> Result<(NonNull<u8>, NonNull<u8>), AllocError> {
    debug_assert!(
        new_layout.size() >= old_layout.size(),
        "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
    );

    let len = old_layout.size() - ptr.1.as_ptr().offset_from(ptr.0.as_ptr()) as usize;
    let ptr_offset_val = new_layout.size() - len;
    let new_ptr = alloc_usr(new_layout);
    let new_mov_ptr = new_ptr.add(ptr_offset_val);

    unsafe {
        ptr::copy_nonoverlapping(ptr.1.as_ptr(), new_ptr.add(ptr_offset_val), len);
        dealloc(ptr.0.as_ptr(), old_layout);
    }

    Ok((
        NonNull::new_unchecked(new_ptr),
        NonNull::new_unchecked(new_mov_ptr),
    ))
}

impl<T> Drop for RawCev<T> {
    fn drop(&mut self) {
        if let Some((ptr, _, layout)) = self.allocated_memory() {
            unsafe { dealloc(ptr.as_ptr(), layout) }
        }
    }
}

fn capacity_overflow() -> ! {
    panic!("capacity overflow");
}

#[inline]
fn alloc_guard(alloc_size: usize) -> Result<(), TryReserveError> {
    if usize::BITS < 64 && alloc_size > isize::MAX as usize {
        Err(TryReserveErrorKind::CapacityOverflow.into())
    } else {
        Ok(())
    }
}

const fn is_zst<U>() -> bool {
    mem::size_of::<U>() == 0
}

#[inline]
fn handle_reserve(result: Result<(), TryReserveError>) {
    match result.map_err(|e| e.kind()) {
        Err(TryReserveErrorKind::CapacityOverflow) => capacity_overflow(),
        Err(TryReserveErrorKind::AllocError { layout, .. }) => handle_alloc_error(layout),
        Ok(()) => {}
    }
}

unsafe fn alloc_usr(layout: Layout) -> *mut u8 {
    let ptr = alloc(layout);

    if ptr.is_null() {
        handle_alloc_error(layout);
    }
    ptr
}
