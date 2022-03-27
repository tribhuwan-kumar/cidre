use crate::ns::Id;
use crate::{cf, define_obj_type};

define_obj_type!(FunctionDescriptor(Id));

#[repr(usize)]
pub enum FunctionOptions {
    None = 0,
    CompileToBinary = 1 << 0,
}

impl FunctionDescriptor {
    #[inline]
    pub fn name<'copy>(&self) -> Option<cf::Retained<'copy, cf::String>> {
        unsafe { copy_nullable_rsel_name(self) }
    }

    #[inline]
    pub fn set_name(&mut self, name: Option<&cf::String>) {
        unsafe { nullable_wsel_setName(self, name) }
    }

    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let fd = mtl::FunctionDescriptor::default();
    ///
    /// assert!(fd.name().is_none());
    ///
    /// let name = cf::String::from_str("hello");
    ///
    /// fd.set_name(Some(&name));
    ///
    /// let actual_name = fd.name().unwrap();
    ///
    /// assert!(name.equal(&actual_name));
    ///
    pub fn default<'autorelease>() -> &'autorelease mut FunctionDescriptor {
        unsafe { MTLFunctionDescriptor_functionDescriptor() }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn copy_nullable_rsel_name<'copy>(id: &Id) -> Option<cf::Retained<'copy, cf::String>>;
    fn nullable_wsel_setName(id: &mut Id, name: Option<&cf::String>);
    fn MTLFunctionDescriptor_functionDescriptor<'autorelease>(
    ) -> &'autorelease mut FunctionDescriptor;

}
