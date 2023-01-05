use crate::{arc, cf, define_cf_type};

use super::{Allocator, Index, String, Type, TypeId};
use std::{ffi::c_void, intrinsics::transmute, marker::PhantomData};

pub type RetainCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type ReleaseCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type CopyDescriptionCallBack = extern "C" fn(value: *const c_void) -> Option<arc::R<String>>;
pub type EqualCallBack = extern "C" fn(value1: *const c_void, value2: *const c_void) -> bool;

#[repr(C)]
pub struct Callbacks {
    version: Index,
    retain: RetainCallBack,
    release: ReleaseCallBack,
    copy_description: CopyDescriptionCallBack,
    equal: EqualCallBack,
}

impl Callbacks {
    #[inline]
    pub fn default() -> Option<&'static Callbacks> {
        unsafe { Some(&kCFTypeArrayCallBacks) }
    }
}

define_cf_type!(Array(Type));

#[derive(Debug)]
#[repr(transparent)]
pub struct ArrayOf<T>(Array, PhantomData<T>);

impl<T> ArrayOf<T> {
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { transmute(Array::new()) }
    }

    #[inline]
    pub fn new_in(allocator: Option<&Allocator>) -> Option<arc::R<Self>> {
        unsafe { transmute(Array::new_in(allocator)) }
    }

    pub fn contains(&self, value: &cf::Type) -> bool {
        if self.is_empty() {
            return false;
        }
        unsafe {
            CFArrayContainsValue(
                self,
                cf::Range::new(0, self.count()),
                value as *const _ as _,
            )
        }
    }

    #[inline]
    pub fn iter(&self) -> ArrayOfIterator<T> {
        ArrayOfIterator {
            array: self,
            index: 0,
            len: self.len(),
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn copy_mut(&self) -> Option<arc::R<ArrayOfMut<T>>> {
        let copy = self.0.copy_mut();
        unsafe { transmute(copy) }
    }

    #[inline]
    pub fn from_slice(values: &[&T]) -> arc::R<Self>
    where
        T: arc::Retain,
    {
        unsafe {
            let arr = Array::create_in(
                values.as_ptr() as _,
                values.len() as _,
                Callbacks::default(),
                None,
            );
            transmute(arr)
        }
    }

    #[inline]
    pub fn from_retained_slice(values: &[arc::R<T>]) -> Option<arc::R<Self>>
    where
        T: arc::Retain,
    {
        unsafe {
            let arr = Array::create_in(
                values.as_ptr() as _,
                values.len() as _,
                Callbacks::default(),
                None,
            );
            transmute(arr)
        }
    }
}

impl<T> std::ops::Deref for ArrayOf<T> {
    type Target = Array;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::Index<usize> for ArrayOf<T>
where
    T: arc::Retain,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { transmute::<&Type, &T>(&self.0[index]) }
    }
}

impl<T> std::ops::IndexMut<usize> for ArrayOf<T>
where
    T: arc::Retain,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { transmute::<&mut Type, &mut T>(&mut self.0[index]) }
    }
}

impl<T> arc::Release for ArrayOf<T> {
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<T> arc::Retain for ArrayOf<T> {
    fn retained(&self) -> arc::R<Self> {
        unsafe { transmute(self.0.retained()) }
    }
}

pub struct ArrayOfIterator<'a, T> {
    array: &'a Array,
    index: usize,
    len: usize,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for ArrayOfIterator<'a, T>
where
    T: arc::Retain,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let res = unsafe { transmute::<&Type, &'a T>(&self.array[self.index]) };
            self.index += 1;
            Some(res)
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for ArrayOfIterator<'a, T>
where
    T: arc::Retain,
{
    fn len(&self) -> usize {
        self.array.len() - self.index
    }
}

#[repr(transparent)]
pub struct ArrayOfMut<T>(ArrayMut, PhantomData<T>);

impl<T> ArrayOfMut<T> {
    #[inline]
    pub fn new() -> arc::R<ArrayOfMut<T>> {
        Self::with_capacity(0)
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> arc::R<Self> {
        unsafe { Self::with_capacity_in(capacity, None).unwrap_unchecked() }
    }

    #[inline]
    pub fn with_capacity_in(capacity: usize, alloc: Option<&Allocator>) -> Option<arc::R<Self>> {
        let arr = ArrayMut::create_in(capacity as _, Callbacks::default(), alloc);
        unsafe { transmute(arr) }
    }

    #[inline]
    pub fn iter(&self) -> ArrayOfIterator<T> {
        ArrayOfIterator {
            array: self,
            index: 0,
            len: self.len(),
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn push(&mut self, value: &T) {
        self.0.append(unsafe { transmute(value) });
    }
}

impl<T> std::ops::Deref for ArrayOfMut<T> {
    type Target = ArrayOf<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self) }
    }
}

impl<T> std::ops::Index<usize> for ArrayOfMut<T>
where
    T: arc::Retain,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { transmute::<&Type, &T>(&self.0[index]) }
    }
}

impl<T> std::ops::IndexMut<usize> for ArrayOfMut<T>
where
    T: arc::Retain,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { transmute::<&mut Type, &mut T>(&mut self.0[index]) }
    }
}

impl<T> arc::Release for ArrayOfMut<T> {
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<T> arc::Retain for ArrayOfMut<T> {
    fn retained(&self) -> arc::R<Self> {
        unsafe { transmute(self.0.retained()) }
    }
}

impl Array {
    /// ```
    /// use cidre::cf;
    ///
    /// let type_id = cf::Array::type_id();
    /// assert_eq!(type_id, 19);
    ///
    /// unsafe {
    ///     let type_desc = cf::copy_type_id_description(type_id).unwrap();
    ///     assert_eq!("CFArray", type_desc.to_string());
    /// }
    /// ```
    #[inline]
    pub fn type_id() -> TypeId {
        unsafe { CFArrayGetTypeID() }
    }
    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::Array::new();
    /// assert_eq!(arr.count(), 0);
    /// ```
    #[inline]
    pub fn count(&self) -> Index {
        unsafe { CFArrayGetCount(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr = unsafe { cf::Array::create_in(std::ptr::null(), 0, None, None).expect("arr") };
    /// assert_eq!(arr.len(), 0);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.count() as _
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::Array::new();
    /// assert_eq!(arr.is_empty(), true);
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr1 = cf::Array::new();
    /// let arr2 = arr1.copy_in(None).expect("copy");
    ///
    /// ```
    #[inline]
    pub fn copy_in(&self, allocator: Option<&Allocator>) -> Option<arc::R<Self>> {
        unsafe { CFArrayCreateCopy(allocator, self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr1 = cf::Array::new();
    /// let arr2 = arr1.copy().expect("copy");
    /// ```
    #[inline]
    pub fn copy(&self) -> Option<arc::R<Array>> {
        self.copy_in(None)
    }

    #[inline]
    pub unsafe fn create_in(
        values: *const c_void,
        num_values: Index,
        callbacks: Option<&Callbacks>,
        allocator: Option<&Allocator>,
    ) -> Option<arc::R<Self>> {
        CFArrayCreate(allocator, values, num_values, callbacks)
    }

    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { Self::new_in(None).unwrap_unchecked() }
    }

    #[inline]
    pub fn new_in(allocator: Option<&Allocator>) -> Option<arc::R<Self>> {
        unsafe { Self::create_in(std::ptr::null(), 0, Callbacks::default(), allocator) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(10);
    /// let arr = cf::Array::from_type_refs(&[&num, &num, &num]).unwrap();
    /// assert_eq!(3, arr.len());
    /// ```
    #[inline]
    pub fn from_type_refs<const N: usize>(values: &[&Type; N]) -> Option<arc::R<Self>> {
        unsafe { Array::create_in(values.as_ptr() as _, N as _, Callbacks::default(), None) }
    }

    #[inline]
    pub fn from_slice<T>(values: &[&T]) -> Option<arc::R<Array>>
    where
        T: arc::Retain,
    {
        unsafe {
            Array::create_in(
                values.as_ptr() as _,
                values.len() as _,
                Callbacks::default(),
                None,
            )
        }
    }

    #[inline]
    pub fn from_copyable<const N: usize, T>(values: &[T; N]) -> Option<arc::R<Self>>
    where
        T: Copy,
    {
        unsafe { Array::create_in(values.as_ptr() as _, N as _, None, None) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(10);
    ///
    /// let empty_arr = cf::Array::new();
    /// let mut mut_arr = empty_arr.copy_mut_in(0, None).unwrap();
    ///
    ///
    /// mut_arr.append(&num);
    ///
    /// assert_eq!(1, mut_arr.len());
    /// assert_eq!(0, empty_arr.len());
    ///
    /// ```
    #[inline]
    pub fn copy_mut_in(
        &self,
        capacity: Index,
        allocator: Option<&Allocator>,
    ) -> Option<arc::R<ArrayMut>> {
        unsafe { CFArrayCreateMutableCopy(allocator, capacity, self) }
    }

    #[inline]
    pub fn copy_mut(&self) -> Option<arc::R<ArrayMut>> {
        unsafe { CFArrayCreateMutableCopy(None, 0, self) }
    }

    #[inline]
    pub fn copy_mut_with_capacity(&self, capacity: usize) -> Option<arc::R<ArrayMut>> {
        self.copy_mut_in(capacity as _, None)
    }
}

impl std::ops::Index<usize> for Array {
    type Output = Type;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { CFArrayGetValueAtIndex(self, index as _) }
    }
}

impl std::ops::IndexMut<usize> for Array {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { CFArrayGetValueAtIndex(self, index as _) }
    }
}

define_cf_type!(ArrayMut(Array));

impl ArrayMut {
    #[inline]
    pub unsafe fn append_value(&mut self, value: *const c_void) {
        CFArrayAppendValue(self, value)
    }

    #[inline]
    pub fn append(&mut self, value: &Type) {
        unsafe { self.append_value(value.as_type_ptr()) }
    }

    #[inline]
    pub fn remove_all_values(&mut self) {
        unsafe {
            CFArrayRemoveAllValues(self);
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.remove_all_values();
    }

    #[inline]
    pub fn create_in(
        capacity: Index,
        callbacks: Option<&Callbacks>,
        allocator: Option<&Allocator>,
    ) -> Option<arc::R<ArrayMut>> {
        unsafe { CFArrayCreateMutable(allocator, capacity, callbacks) }
    }

    #[inline]
    pub fn with_capacity(capacity: Index) -> arc::R<Self> {
        unsafe { Self::create_in(capacity, Callbacks::default(), None).unwrap_unchecked() }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let mut arr = cf::ArrayMut::new();
    /// assert_eq!(0, arr.len());
    ///
    /// let num = cf::Number::from_i32(0);
    ///
    /// arr.append(&num);
    /// arr.append(&num);
    /// assert_eq!(2, arr.len());
    ///
    /// arr.remove_all_values();
    /// assert_eq!(0, arr.len());
    /// ```
    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::with_capacity(0)
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFTypeArrayCallBacks: Callbacks;

    fn CFArrayGetTypeID() -> TypeId;

    fn CFArrayGetValueAtIndex(array: &Array, idx: Index) -> &mut Type;

    fn CFArrayCreate(
        allocator: Option<&Allocator>,
        values: *const c_void,
        num_values: Index,
        callbacks: Option<&Callbacks>,
    ) -> Option<arc::R<Array>>;

    fn CFArrayCreateCopy(allocator: Option<&Allocator>, array: &Array) -> Option<arc::R<Array>>;

    fn CFArrayGetCount(array: &Array) -> Index;

    fn CFArrayCreateMutable(
        allocator: Option<&Allocator>,
        capacity: Index,
        callbacks: Option<&Callbacks>,
    ) -> Option<arc::R<ArrayMut>>;

    fn CFArrayCreateMutableCopy(
        allocator: Option<&Allocator>,
        capacity: Index,
        array: &Array,
    ) -> Option<arc::R<ArrayMut>>;

    fn CFArrayAppendValue(array: &mut ArrayMut, value: *const c_void);

    fn CFArrayRemoveAllValues(array: &mut ArrayMut);

    fn CFArrayContainsValue(array: &Array, range: cf::Range, value: *const c_void) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::cf;

    #[test]
    pub fn empty_arrays_are_same() {
        let arr1 = cf::Array::new();
        let arr2 = arr1.copy().expect("copy");
        let arr3 = cf::Array::new();
        let arr4 = arr2.copy_mut().expect("copy");
        unsafe {
            assert_eq!(arr1.as_type_ptr(), arr2.as_type_ptr());
            assert_eq!(arr3.as_type_ptr(), arr2.as_type_ptr());
            assert_ne!(arr1.as_type_ptr(), arr4.as_type_ptr());
        }
    }
}
