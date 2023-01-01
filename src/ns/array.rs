use std::{
    ffi::c_void,
    marker::PhantomData,
    mem::transmute,
    ops::{Deref, Index},
};

use crate::{cf, msg_send, ns};

#[derive(Debug)]
#[repr(transparent)]
pub struct Array<T>(ns::Id, PhantomData<T>);

impl<T> Deref for Array<T> {
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> cf::Release for Array<T> {
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<T> cf::Retain for Array<T> {
    fn retained(&self) -> cf::Retained<Self> {
        unsafe { transmute(self.0.retained()) }
    }
}

impl<T> Array<T>
where
    T: cf::Release,
{
    pub fn from_slice(objs: &[&T]) -> cf::Retained<Self> {
        unsafe { transmute(NSArray_withObjs(objs.as_ptr() as _, objs.len())) }
    }

    #[inline]
    pub fn count(&self) -> usize {
        msg_send!("ns", self, ns_count)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.count()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        msg_send!("ns", self, ns_objectAtIndex_index, index)
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSArray_withObjs(objects: *const c_void, count: ns::UInteger)
        -> cf::Retained<Array<ns::Id>>;
}

#[cfg(test)]
mod tests {
    use crate::ns;
    #[test]
    fn basics() {
        let one = ns::Number::with_u8(5);
        let arr: &[&ns::Number] = &[&one];
        let array = ns::Array::from_slice(&arr);
        assert_eq!(5, array[0].as_u8());
    }
}
