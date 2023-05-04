use crate::{define_cf_type, ns, objc};

use crate::{arc, cf};

use super::{Allocator, HashCode, Index, String, Type, TypeId};
use std::marker::PhantomData;
use std::{ffi::c_void, intrinsics::transmute, ptr::NonNull};

pub type RetainCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type ReleaseCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type CopyDescriptionCallBack = extern "C" fn(value: *const c_void) -> Option<arc::R<String>>;
pub type EqualCallBack = extern "C" fn(value1: *const c_void, value2: *const c_void) -> bool;
pub type HashCallBack = extern "C" fn(value: *const c_void) -> HashCode;

#[repr(C)]
pub struct KeyCallBacks {
    version: Index,
    retain: RetainCallBack,
    release: ReleaseCallBack,
    copy_description: CopyDescriptionCallBack,
    equal: EqualCallBack,
    hash: HashCallBack,
}

impl KeyCallBacks {
    #[inline]
    pub fn default() -> Option<&'static KeyCallBacks> {
        unsafe { Some(&kCFTypeDictionaryKeyCallBacks) }
    }

    #[inline]
    pub fn copy_strings() -> Option<&'static KeyCallBacks> {
        unsafe { Some(&kCFCopyStringDictionaryKeyCallBacks) }
    }
}

#[repr(C)]
pub struct ValueCallBacks {
    version: Index,
    retain: RetainCallBack,
    release: ReleaseCallBack,
    copy_description: CopyDescriptionCallBack,
    equal: EqualCallBack,
}

impl ValueCallBacks {
    #[inline]
    pub fn default() -> Option<&'static ValueCallBacks> {
        unsafe { Some(&kCFTypeDictionaryValueCallBacks) }
    }
}

pub type ApplierFunction =
    extern "C" fn(key: *const c_void, value: *const c_void, context: *mut c_void);

define_cf_type!(Dictionary(Type));

impl Dictionary {
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { Self::new_in(None).unwrap_unchecked() }
    }

    #[inline]
    pub fn new_in(allocator: Option<&Allocator>) -> Option<arc::R<Self>> {
        unsafe { CFDictionaryCreate(allocator, std::ptr::null(), std::ptr::null(), 0, None, None) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let type_id = cf::Dictionary::type_id();
    /// assert_eq!(type_id, 18);
    ///
    /// unsafe {
    ///     let type_desc = cf::copy_type_id_description(type_id).unwrap();
    ///     assert_eq!("CFDictionary", type_desc.to_string());
    /// }
    /// ```
    #[inline]
    pub fn type_id() -> TypeId {
        unsafe { CFDictionaryGetTypeID() }
    }

    /// try use contains_key first
    #[inline]
    pub unsafe fn contains_raw_key(&self, key: *const c_void) -> bool {
        CFDictionaryContainsKey(self, key)
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::with_keys_values(&[&key], &[&value]).unwrap();
    ///
    /// assert!(d.contains_key(&key));
    ///
    /// let key2 = cf::Number::from_i8(12);
    /// assert!(!d.contains_key(&key2));
    /// ```
    #[inline]
    pub fn contains_key(&self, key: &Type) -> bool {
        unsafe { CFDictionaryContainsKey(self, key.as_type_ptr()) }
    }

    #[inline]
    pub unsafe fn contains_raw_value(&self, value: *const c_void) -> bool {
        CFDictionaryContainsValue(self, value)
    }

    #[inline]
    pub unsafe fn raw_value(&self, key: *const c_void) -> Option<NonNull<c_void>> {
        CFDictionaryGetValue(self, key)
    }

    #[inline]
    pub unsafe fn raw_value_if_present(
        &self,
        key: *const c_void,
    ) -> Option<Option<NonNull<c_void>>> {
        let mut value = Option::None;

        if CFDictionaryGetValueIfPresent(self, key, &mut value) {
            Some(value)
        } else {
            None
        }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::with_keys_values(&[&key], &[&value]).unwrap();
    ///
    /// let v = d.value(&key).unwrap();
    /// assert!(v.equal(&value));
    /// unsafe {
    ///     assert_eq!(v.as_type_ptr(), value.as_type_ptr());
    /// }
    /// ```
    pub fn value<'a>(&'a self, key: &Type) -> Option<&'a Type> {
        unsafe {
            let mut value = Option::None;
            if CFDictionaryGetValueIfPresent(self, key.as_type_ptr(), &mut value) {
                Some(transmute(value))
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn count(&self) -> Index {
        unsafe { CFDictionaryGetCount(self) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.count() as _
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::with_keys_values(&[&key], &[&value]).unwrap();
    ///
    /// assert!(!d.is_empty());
    /// assert_eq!(1, d.len());
    /// ```
    #[inline]
    pub fn with_keys_values<const N: usize>(
        keys: &[&Type; N],
        values: &[&Type; N],
    ) -> Option<arc::R<Dictionary>> {
        unsafe {
            Self::create(
                None,
                keys.as_ptr() as _,
                values.as_ptr() as _,
                N as _,
                KeyCallBacks::default(),
                ValueCallBacks::default(),
            )
        }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let dict = unsafe { cf::Dictionary::create(None, std::ptr::null(), std::ptr::null(), 0, None, None).unwrap() };
    ///
    /// dict.show();
    /// ```
    #[inline]
    pub unsafe fn create(
        allocator: Option<&Allocator>,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: Index,
        key_callbacks: Option<&KeyCallBacks>,
        value_callbacks: Option<&ValueCallBacks>,
    ) -> Option<arc::R<Dictionary>> {
        CFDictionaryCreate(
            allocator,
            keys,
            values,
            num_values,
            key_callbacks,
            value_callbacks,
        )
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::with_keys_values(&[&key], &[&value]).unwrap();
    ///
    /// let keys = d.keys();
    ///
    /// assert!(!d.is_empty());
    /// assert_eq!(1, d.len());
    /// assert_eq!(1, keys.len());
    /// assert!(key.equal(keys[0]));
    /// ```
    pub fn keys(&self) -> Vec<&Type> {
        let len = self.len();
        let mut keys: Vec<&Type> = Vec::with_capacity(len);
        unsafe {
            keys.set_len(len);
            let keys = keys.as_ptr() as *const *const c_void;
            self.keys_and_values(keys, std::ptr::null());
        }
        keys
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::with_keys_values(&[&key], &[&value]).unwrap();
    ///
    /// let vals = d.values();
    ///
    /// assert!(!d.is_empty());
    /// assert_eq!(1, d.len());
    /// assert_eq!(1, vals.len());
    /// assert!(value.equal(vals[0]));
    /// ```
    pub fn values(&self) -> Vec<&Type> {
        let len = self.len();
        let mut values: Vec<&Type> = Vec::with_capacity(len);
        unsafe {
            values.set_len(len);
            let values = values.as_ptr() as *const *const c_void;
            self.keys_and_values(std::ptr::null(), values);
        }
        values
    }

    pub fn keys_with_values(&self) -> (Vec<&Type>, Vec<&Type>) {
        let len = self.len();
        let mut keys: Vec<&Type> = Vec::with_capacity(len);
        let mut values: Vec<&Type> = Vec::with_capacity(len);
        unsafe {
            keys.set_len(len);
            values.set_len(len);
            let keys = keys.as_ptr() as *const *const c_void;
            let values = values.as_ptr() as *const *const c_void;
            self.keys_and_values(keys, values);
        }
        (keys, values)
    }

    #[inline]
    pub unsafe fn keys_and_values(&self, keys: *const *const c_void, values: *const *const c_void) {
        CFDictionaryGetKeysAndValues(self, keys, values)
    }

    #[inline]
    pub fn copy(&self) -> arc::R<Self> {
        unsafe { self.copy_in(None).unwrap_unchecked() }
    }

    #[inline]
    pub fn copy_in(&self, allocator: Option<&cf::Allocator>) -> Option<arc::R<Self>> {
        unsafe { CFDictionaryCreateCopy(allocator, self) }
    }
}

impl Default for arc::R<Dictionary> {
    fn default() -> Self {
        Dictionary::new()
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFTypeDictionaryKeyCallBacks: KeyCallBacks;
    static kCFCopyStringDictionaryKeyCallBacks: KeyCallBacks;

    static kCFTypeDictionaryValueCallBacks: ValueCallBacks;

    fn CFDictionaryGetTypeID() -> TypeId;

    fn CFDictionaryContainsKey(the_dict: &Dictionary, key: *const c_void) -> bool;
    fn CFDictionaryContainsValue(the_dict: &Dictionary, value: *const c_void) -> bool;

    fn CFDictionaryGetCount(the_dict: &Dictionary) -> Index;
    fn CFDictionaryGetValue(the_dict: &Dictionary, key: *const c_void) -> Option<NonNull<c_void>>;
    fn CFDictionaryGetValueIfPresent(
        the_dict: &Dictionary,
        key: *const c_void,
        value: *mut Option<NonNull<c_void>>,
    ) -> bool;

    fn CFDictionaryCreate(
        allocator: Option<&Allocator>,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: Index,
        key_callbacks: Option<&KeyCallBacks>,
        value_callbacks: Option<&ValueCallBacks>,
    ) -> Option<arc::R<Dictionary>>;

    fn CFDictionaryGetKeysAndValues(
        the_dict: &Dictionary,
        keys: *const *const c_void,
        values: *const *const c_void,
    );

    fn CFDictionaryCreateCopy(
        allocator: Option<&cf::Allocator>,
        the_dict: &cf::Dictionary,
    ) -> Option<arc::R<cf::Dictionary>>;

}

define_cf_type!(DictionaryMut(Dictionary));

impl DictionaryMut {
    pub fn with_capacity(capacity: usize) -> arc::R<Self> {
        unsafe { Self::with_capacity_in(None, capacity).unwrap_unchecked() }
    }

    pub fn with_capacity_in(
        allocator: Option<&Allocator>,
        capacity: usize,
    ) -> Option<arc::R<Self>> {
        unsafe {
            Self::create(
                allocator,
                capacity as _,
                KeyCallBacks::default(),
                ValueCallBacks::default(),
            )
        }
    }

    pub unsafe fn create(
        allocator: Option<&cf::Allocator>,
        capacity: cf::Index,
        key_callbacks: Option<&KeyCallBacks>,
        value_callbacks: Option<&ValueCallBacks>,
    ) -> Option<arc::R<Self>> {
        CFDictionaryCreateMutable(allocator, capacity, key_callbacks, value_callbacks)
    }

    pub fn insert(&mut self, key: &cf::String, value: &cf::Type) {
        unsafe { CFDictionarySetValue(self, key.as_type_ptr(), value.as_type_ptr()) }
    }

    pub fn remove(&mut self, key: &cf::String) {
        unsafe { CFDictionaryRemoveValue(self, key.as_type_ptr()) }
    }

    pub unsafe fn add_value(&mut self, key: *const c_void, value: *const c_void) {
        CFDictionaryAddValue(self, key, value)
    }

    pub unsafe fn set_value(&mut self, key: *const c_void, value: *const c_void) {
        CFDictionarySetValue(self, key, value)
    }

    pub unsafe fn replace_value(&mut self, key: *const c_void, value: *const c_void) {
        CFDictionaryReplaceValue(self, key, value)
    }

    pub unsafe fn remove_value(&mut self, key: *const c_void) {
        CFDictionaryRemoveValue(self, key)
    }

    pub fn remove_all_values(&mut self) {
        unsafe { CFDictionaryRemoveAllValues(self) }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct DictionaryOf<K, V>(Dictionary, PhantomData<(K, V)>)
where
    K: arc::Retain,
    V: arc::Retain;

impl<K, V> DictionaryOf<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    pub fn get(&self, k: &K) -> Option<&V> {
        unsafe { transmute(self.0.value(transmute(k))) }
    }
}

impl<K, V> DictionaryOf<K, V>
where
    K: objc::Obj,
    V: objc::Obj,
{
    /// Toll-Free Bridged
    pub fn as_cf(&self) -> &ns::Dictionary<K, V> {
        unsafe { transmute(self) }
    }
}

impl<K, V> std::ops::Deref for DictionaryOf<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    type Target = Dictionary;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> arc::Release for DictionaryOf<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<K, V> arc::Retain for DictionaryOf<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    fn retained(&self) -> arc::R<Self> {
        unsafe { transmute(self.0.retained()) }
    }
}

#[repr(transparent)]
pub struct DictionaryOfMut<K, V>(DictionaryMut, PhantomData<(K, V)>);

impl<K, V> DictionaryOf<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    #[inline]
    pub fn with_keys_values<const N: usize>(
        keys: &[&K; N],
        values: &[&V; N],
    ) -> arc::R<DictionaryOf<K, V>> {
        unsafe {
            let dict = Dictionary::create(
                None,
                keys.as_ptr() as _,
                values.as_ptr() as _,
                N as _,
                KeyCallBacks::default(),
                ValueCallBacks::default(),
            );

            transmute(dict)
        }
    }
}

extern "C" {
    fn CFDictionaryCreateMutable(
        allocator: Option<&cf::Allocator>,
        capacity: cf::Index,
        key_callbacks: Option<&KeyCallBacks>,
        value_callbacks: Option<&ValueCallBacks>,
    ) -> Option<arc::R<DictionaryMut>>;

    fn CFDictionaryAddValue(the_dict: &mut DictionaryMut, key: *const c_void, value: *const c_void);
    fn CFDictionarySetValue(the_dict: &mut DictionaryMut, key: *const c_void, value: *const c_void);
    fn CFDictionaryReplaceValue(
        the_dict: &mut DictionaryMut,
        key: *const c_void,
        value: *const c_void,
    );
    fn CFDictionaryRemoveValue(the_dict: &mut DictionaryMut, key: *const c_void);
    fn CFDictionaryRemoveAllValues(the_dict: &mut DictionaryMut);
}
