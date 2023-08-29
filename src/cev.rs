#![allow(clippy::partialeq_ne_impl)]
use crate::raw_cev::RawCev;
use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt;
use core::mem::{self, ManuallyDrop, MaybeUninit};
use core::ops::{self, Index, IndexMut};
use core::ptr::{self, NonNull};
use core::slice::{self, SliceIndex};

/// An array of data allocated on the heap that grows from end to beginning.
///
/// # Examples
///
/// ```
/// use cev::Cev;
///
/// let mut cev = Cev::new();
/// cev.push('v');
/// cev.push('e');
/// cev.push('c');
///
/// assert_eq!(cev, ['c', 'e', 'v']);
///
/// ```
pub struct Cev<T> {
    buf: RawCev<T>,
    len: usize,
}

impl<T> Cev<T> {
    /// Adds elements to the beginning of the `Cev` array, moving them from another `Cev` array,
    /// leaving the other empty to reuse the allocated memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let mut cev = Cev::from([4, 5, 6]);
    /// let mut cev_other = Cev::from([1, 2, 3]);
    /// cev.append(&mut cev_other);
    /// assert_eq!(cev, [1, 2, 3, 4, 5, 6]);
    /// assert_eq!(cev_other, []);
    /// ```
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        unsafe {
            self.append_elements(other.as_slice() as _);
            other.set_len_ptr(0);
        }
    }

    /// Returns an unsafe mutable pointer.
    /// If length is zero, then points to capacity minus 1 element of type `T`,
    /// otherwise capacity minus length.
    /// For unallocated array `Cev` and types of size zero, a `NonNull` dangling pointer is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use::cev::Cev;
    ///
    /// let len = 5;
    /// let mut cev = Cev::<u8>::with_capacity(len);
    /// let mov_ptr = cev.as_mut_ptr();
    ///
    /// unsafe {
    ///     for elem in 0..len {
    ///         *mov_ptr.sub(elem) = elem as u8;
    ///     }
    ///     cev.set_len_ptr(len);
    /// }
    ///
    /// // The `mov_ptr` pointer points to the element at index 4.
    /// assert_eq!(unsafe { *mov_ptr }, 0);
    /// assert_eq!(cev, [4, 3, 2, 1, 0]);
    /// ```
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.buf.ptr()
    }

    /// A mutable slice from this `Cev` array.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }

    /// Returns an unsafe constant pointer.
    /// If length is zero, then points to capacity minus 1 element of type `T`,
    /// otherwise capacity minus length.
    /// For unallocated array `Cev` and types of size zero, a `NonNull` dangling pointer is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use::cev::Cev;
    ///
    /// let mut cev = Cev::<u8>::from_vec(vec![2, 4, 6, 8, 10]);
    /// let len = cev.len();
    /// let mov_ptr = cev.as_mut_ptr();
    ///
    /// unsafe {
    ///     for index in 0..len {
    ///         
    ///         assert_eq!(unsafe { *mov_ptr.add(index) }, cev[index]);
    ///     }
    /// }
    ///
    /// ```
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.buf.ptr()
    }

    /// A constant slice from this `Cev` array.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self
    }

    /// Returns the capacity of this `Cev` array.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let cev = Cev::<String>::with_capacity(0);
    /// assert_eq!(cev.capacity(), 0);
    ///
    /// let cev = Cev::<String>::with_capacity(1);
    /// assert_eq!(cev.capacity(), 1);
    ///
    /// let cev = Cev::<()>::with_capacity(0);
    /// assert_eq!(cev.capacity(), usize::MAX);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    /// Clears the `Cev` array, removing all elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let mut cev: Cev<&str> = Cev::from(["c", "l", "e", "a", "r"]);
    /// cev.clear();
    /// assert!(cev.is_empty());
    ///
    /// cev.append(&mut Cev::from_vec(vec!["r", "e", "u", "s", "e"]));
    /// assert_eq!(cev, Cev::from_vec(vec!["r", "e", "u", "s", "e"]));
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        let elems: *mut [T] = self.as_mut_slice();

        unsafe {
            self.set_len_ptr(0);
            ptr::drop_in_place(elems);
        }
    }

    /// Creates a `Cev<T>` directly from a pointers, a capacity, and a length.
    #[inline]
    pub unsafe fn from_raw_parts(mov_ptr: *mut T, raw_ptr: *mut T, len: usize, cap: usize) -> Self {
        unsafe {
            Cev {
                buf: RawCev::from_raw_parts_ptr(mov_ptr, raw_ptr, cap),
                len,
            }
        }
    }

    /// Converting a std vector to a `Cev` array.
    /// When length and capacity are equal, data copying is not required.
    /// If the capacity is larger, the data is copied to the end of the array.
    /// In both cases the same memory is reused.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let vec = vec!["std", "vector"];
    /// let cev = Cev::from_vec(vec);
    /// assert_eq!(cev, ["std", "vector"]);
    /// ```
    #[inline]
    pub fn from_vec(mut vec: Vec<T>) -> Self {
        let (raw_ptr, len, capacity) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
        unsafe {
            vec.set_len(0);
            mem::forget(vec);

            Cev::from_raw_parts(
                if capacity == len {
                    raw_ptr
                } else if len == 0 {
                    raw_ptr.add(capacity - 1)
                } else {
                    let mov_ptr = raw_ptr.add(capacity - len);
                    ptr::copy(raw_ptr, mov_ptr, len);
                    mov_ptr
                },
                raw_ptr,
                len,
                capacity,
            )
        }
    }

    /// Inserts an element at position `index` within the `Cev` array, shifting all
    /// elements before it to the left.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let mut cev = Cev::from([2, 2, 3]);
    /// cev.insert(1, 0);
    /// assert_eq!(cev, [2, 0, 2, 3]);
    /// cev.insert(4, 8);
    /// assert_eq!(cev, [2, 0, 2, 3, 8]);
    /// ```
    pub fn insert(&mut self, index: usize, element: T) {
        #[cold]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("insertion index (is {index}) should be <= len (is {len})");
        }

        let len = self.len();

        if len == self.buf.capacity() {
            self.reserve(1);
        }

        unsafe {
            if index == 0 {
                if len != 0 {
                    self.buf.mov_ptr_sub(1)
                }
            } else if index <= len {
                let p = self.as_mut_ptr();
                ptr::copy(p, p.sub(1), index);
                self.buf.mov_ptr_sub(1)
            } else {
                assert_failed(index, len);
            }

            ptr::write(self.as_mut_ptr().add(index), element);
            self.set_len(len + 1);
        }
    }

    /// Converting a `Cev` array to a `std` vector.
    /// When length and capacity are equal, data copying is not required.
    /// If the capacity is larger, the data is copied to the begining of the array.
    /// In both cases the same memory is reused.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let cev = Cev::from_vec(vec!["std", "vector"]);
    /// let vec = cev.into_vec();
    /// assert_eq!(vec, ["std", "vector"]);
    /// ```
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        let (ptr, len, capacity) = (self.buf.raw_ptr(), self.len(), self.capacity());
        unsafe {
            if capacity != len {
                ptr::copy(self.as_ptr(), ptr, len);
            }

            mem::forget(self);
            Vec::from_raw_parts(ptr, len, capacity)
        }
    }

    /// Returns `true` if the `Cev` array contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let mut cev = Cev::new();
    /// assert!(cev.is_empty());
    ///
    /// cev.push("push one element");
    /// assert!(!cev.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// `Cev` array length, the number of elements in the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let cev = Cev::from([1, 2, 3]);
    /// assert_eq!(cev.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Creates a new, empty `Cev<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let cev: Cev<i64> = Cev::new();
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Cev {
            buf: RawCev::NEW,
            len: 0,
        }
    }

    /// Removes the first element from a collection and returns it, or None if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let mut cev = Cev::from([1, 9, 7, 5]);
    /// assert_eq!(cev.pop(), Some(1));
    /// assert_eq!(cev, [9, 7, 5]);
    /// assert_eq!(cev.pop(), Some(9));
    /// assert_eq!(cev.pop(), Some(7));
    /// assert_eq!(cev.pop(), Some(5));
    /// assert_eq!(cev.pop(), None);
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                let ptr = self.as_ptr();
                if self.len != 0 {
                    self.buf.mov_ptr_add(1);
                }
                Some(ptr::read(ptr))
            }
        }
    }

    /// Appends an element to the beginning of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let mut cev: Cev<i8> = Cev::new();
    /// cev.push(1);
    /// cev.push(2);
    /// cev.push(3);
    /// assert_eq!(cev, [3, 2, 1]);
    /// ```
    #[inline]
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity() {
            self.buf.reserve_for_push(self.len);
        }
        unsafe {
            if self.len != 0 {
                self.buf.mov_ptr_sub(1);
            }
            self.as_mut_ptr().write(value);
            self.len += 1;
        };
    }

    /// Returns a constant unsafe pointer.
    /// Pointer for alloc and dealloc memory.
    #[inline]
    pub fn raw_ptr(&self) -> *const T {
        self.buf.raw_ptr()
    }

    /// Reserves capacity for the elements in this collection.
    /// It is possible that more space will be reserved than specified.
    /// Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let mut cev = Cev::from(["one"]);
    /// cev.reserve(5);
    /// assert!(cev.capacity() >= 6);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.buf.reserve(self.len, additional);
    }

    /// Changes the current length to `new_len`.
    ///
    /// # Safety
    /// - Length of new_len must be initialized.
    /// - The length must be equal to or less than the capacity.
    /// - The `mov_ptr` **pointer must be set to the correct position**.
    ///
    /// # Exaples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let mut cev = Cev::<u8>::from([21, 32, 43]);
    /// unsafe {
    ///     cev.set_len(2);
    ///     cev.as_mut_ptr().add(cev.capacity() - cev.len());
    /// };
    /// assert_eq!(cev, [21, 32]);
    /// ```
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());

        self.len = new_len;
    }

    /// Changes the current length to `new_len`,
    /// also sets the `mov_ptr` pointer to the desired position.
    ///
    /// # Safety
    /// - Length of new_len must be initialized.
    /// - The length of `new_len` must not be greater than the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let mut cev = Cev::from([1, 2, 3, 4, 5]);
    ///
    /// unsafe { cev.set_len_ptr(2) };
    /// assert_eq!(cev, [4, 5]);
    /// ```
    #[inline]
    pub unsafe fn set_len_ptr(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());

        self.buf.mov_ptr(self.mov_ptr(new_len));
        self.len = new_len;
    }

    #[inline]
    unsafe fn mov_ptr(&self, new_len: usize) -> *mut T {
        if self.capacity() == new_len {
            self.buf.raw_ptr()
        } else if new_len == 0 {
            self.buf.raw_ptr().add(self.capacity() - 1)
        } else {
            self.buf.raw_ptr().add(self.capacity() - new_len)
        }
    }

    /// Returns the remaining spare capacity of the `Cev` array as a slice of
    /// `MaybeUninit<T>`.
    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        unsafe {
            slice::from_raw_parts_mut(
                self.buf.raw_ptr() as *mut MaybeUninit<T>,
                self.buf.capacity() - self.len,
            )
        }
    }

    /// Reduces the length of the `Cev` array to `len`,
    /// by removing elements from the beginning of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    ///
    /// let mut cev = Cev::from(['T', 'r', 'u', 'n', 'c', 'a', 't', 'e']);
    /// cev.truncate(4);
    /// assert_eq!(cev, ['c', 'a', 't', 'e']);
    /// ```
    pub fn truncate(&mut self, len: usize) {
        unsafe {
            if len >= self.len {
                return;
            }

            let remaining_len = self.len - len;
            let s = ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), remaining_len);
            if len != 0 {
                self.buf.mov_ptr_add(remaining_len);
            } else {
                self.buf.mov_ptr_add(remaining_len - 1);
            }

            self.len = len;
            ptr::drop_in_place(s);
        }
    }

    /// Creates a new `Cev` array and allocates memory for type `T` with the given capacity.
    /// If `capacity` is null, no memory is allocated.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use cev::Cev;
    /// use std::ptr::NonNull;
    ///
    /// let mut cev = Cev::with_capacity(0);
    /// assert_eq!(cev.as_ptr(), NonNull::<u32>::dangling().as_ptr());
    ///
    /// let capacity = 5;
    /// let mut cev = Cev::with_capacity(capacity);
    ///
    /// assert!(cev.capacity() >= capacity);
    ///
    /// for val in 0..=capacity {
    ///     cev.insert(0, val);
    /// }
    /// assert_eq!(cev, [5, 4, 3, 2, 1, 0]);
    ///
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Cev {
            buf: RawCev::with_capacity(capacity),
            len: 0,
        }
    }

    /// Appends elements to `self` from other buffer.
    #[inline]
    unsafe fn append_elements(&mut self, other: *const [T]) {
        let count = (*other).len();
        self.reserve(count);
        self.set_len_ptr(self.len() + count);
        ptr::copy_nonoverlapping(other as *const T, self.as_mut_ptr(), count);
    }
}

impl<T> AsRef<Cev<T>> for Cev<T> {
    fn as_ref(&self) -> &Cev<T> {
        self
    }
}

impl<T> AsMut<Cev<T>> for Cev<T> {
    fn as_mut(&mut self) -> &mut Cev<T> {
        self
    }
}

impl<T> Borrow<[T]> for Cev<T> {
    fn borrow(&self) -> &[T] {
        &self[..]
    }
}

impl<T> BorrowMut<[T]> for Cev<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

impl<T> AsRef<[T]> for Cev<T> {
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T> AsMut<[T]> for Cev<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T> ops::Deref for Cev<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
    }
}

impl<T> ops::DerefMut for Cev<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }
}

impl<T> Drop for Cev<T> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), self.len)) }
    }
}

impl<T: fmt::Debug> fmt::Debug for Cev<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

macro_rules! impl_slice_eq {
    ([$($vars:tt)*] $lhs:ty, $rhs:ty $(where $ty:ty: $bound:ident)?) => {
        impl<T, U, $($vars)*> PartialEq<$rhs> for $lhs
        where
            T: PartialEq<U>,
            $($ty: $bound)?
        {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool { self[..] == other[..] }
            #[inline]
            fn ne(&self, other: &$rhs) -> bool { self[..] != other[..] }
        }
    }
}

impl_slice_eq! { [] Cev<T>, &[U]}
impl_slice_eq! { [const N: usize] Cev<T>, [U; N] }
impl_slice_eq! { [const N: usize] Cev<T>, &[U; N] }
impl_slice_eq! { [] Cev<T>, &mut [U] }
impl_slice_eq! { [] Cev<T>, [U] }
impl_slice_eq! { [] [T], Cev<U> }
impl_slice_eq! { [] &[T], Cev<U> }
impl_slice_eq! { [] &mut [T], Cev<U> }
impl_slice_eq! { [] Cev<T>, Cev<U> }
impl_slice_eq! { [] Cev<T>, Vec<U> }
impl_slice_eq! { [] Vec<T>, Cev<U> }

impl<T: PartialOrd> PartialOrd for Cev<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl<T: Eq> Eq for Cev<T> {}

impl<T: Ord> Ord for Cev<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl<T> Default for Cev<T> {
    fn default() -> Cev<T> {
        Cev::new()
    }
}

impl<T> Clone for Cev<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        to_cev(self)
    }
}

pub fn to_cev<T: ConvertCev>(s: &[T]) -> Cev<T> {
    T::to_cev(s)
}

impl<T, I: SliceIndex<[T]>> Index<I> for Cev<T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for Cev<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}

impl<T, const N: usize> TryFrom<Cev<T>> for [T; N] {
    type Error = Cev<T>;

    fn try_from(mut cev: Cev<T>) -> Result<[T; N], Cev<T>> {
        if cev.len() != N {
            return Err(cev);
        }

        unsafe { cev.set_len(0) };
        let array = unsafe { ptr::read(cev.as_ptr() as *const [T; N]) };
        Ok(array)
    }
}

impl<T> From<Cev<T>> for Vec<T> {
    fn from(cev: Cev<T>) -> Vec<T> {
        cev.into_vec()
    }
}

impl<T> From<Vec<T>> for Cev<T> {
    fn from(vec: Vec<T>) -> Cev<T> {
        Cev::from_vec(vec)
    }
}

impl<T, const N: usize> From<[T; N]> for Cev<T> {
    fn from(slice: [T; N]) -> Cev<T> {
        Cev::from(Vec::from(slice))
    }
}

impl<T> From<&[T]> for Cev<T>
where
    T: Clone,
{
    fn from(slice: &[T]) -> Cev<T> {
        to_cev(slice)
    }
}

impl<T> From<&mut [T]> for Cev<T>
where
    T: Clone,
{
    fn from(slice: &mut [T]) -> Cev<T> {
        to_cev(slice)
    }
}

pub trait ConvertCev {
    fn to_cev(s: &[Self]) -> Cev<Self>
    where
        Self: Sized;
}

impl<T> FromIterator<T> for Cev<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Cev<T> {
        Self::from_vec(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<T> IntoIterator for Cev<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let cev = ManuallyDrop::new(self);
        let ptr = cev.as_ptr();
        let len = cev.len();

        IntoIter {
            buf: unsafe { NonNull::new_unchecked(cev.buf.raw_ptr()) },
            cap: cev.capacity(),
            len,
            ptr,
            end: unsafe { ptr.add(len) },
        }
    }
}

impl<'a, T> IntoIterator for &'a Cev<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Cev<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: Clone> ConvertCev for T {
    fn to_cev(s: &[Self]) -> Cev<Self> {
        struct DropGuard<'a, T> {
            cev: &'a mut Cev<T>,
            num_init: usize,
        }

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                unsafe {
                    self.cev.set_len(self.num_init);
                }
            }
        }

        let mut cev = Cev::with_capacity(s.len());
        let mut guard = DropGuard {
            cev: &mut cev,
            num_init: 0,
        };

        let slots = guard.cev.spare_capacity_mut();
        for (i, b) in s.iter().enumerate().take(slots.len()) {
            guard.num_init = i;
            slots[i].write(b.clone());
        }

        core::mem::forget(guard);
        unsafe {
            cev.set_len(s.len());
            cev.buf.mov_ptr(cev.buf.raw_ptr());
        }
        cev
    }
}

pub struct IntoIter<T> {
    buf: NonNull<T>,
    cap: usize,
    len: usize,
    ptr: *const T,
    end: *const T,
}

impl<T> IntoIter<T> {
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len()) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { &mut *self.as_raw_mut_slice() }
    }

    fn as_raw_mut_slice(&mut self) -> *mut [T] {
        ptr::slice_from_raw_parts_mut(self.ptr as *mut T, self.len())
    }
}

impl<T> AsRef<[T]> for IntoIter<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> Default for IntoIter<T> {
    fn default() -> Self {
        Cev::new().into_iter()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                let ptr = self.ptr;
                self.ptr = self.ptr.add(1);
                Some(ptr::read(ptr))
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            for _ in &mut *self {}
            unsafe {
                let _ = RawCev::from_raw_parts_ptr(self.buf.as_ptr(), self.buf.as_ptr(), self.cap);
            }
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                self.end = self.end.sub(1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.as_slice()).finish()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}
