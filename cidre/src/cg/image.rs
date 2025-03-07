use crate::{cf, cg, define_cf_type};

#[cfg(feature = "iio")]
pub mod source;
#[cfg(feature = "iio")]
pub use source::Src as ImageSrc;

#[cfg(feature = "iio")]
pub mod destination;
#[cfg(feature = "iio")]
pub use destination::Dst as ImageDst;

#[cfg(feature = "iio")]
pub mod animation;
#[cfg(feature = "iio")]
pub use animation::OptKey as AnimationOptKey;
#[cfg(feature = "iio")]
pub use animation::err as animation_err;

#[cfg(feature = "iio")]
pub use animation::AnimationBlock;

#[cfg(feature = "iio")]
pub use animation::animate_image_at_url;
#[cfg(feature = "iio")]
pub use animation::animate_image_at_url_with_block;
#[cfg(feature = "iio")]
pub use animation::animate_image_data;
#[cfg(feature = "iio")]
pub use animation::animate_image_data_with_block;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum AlphaInfo {
    /// For example, RGB
    None,
    /// For example, premultiplied RGBA
    PremultipliedLast,
    /// For example, premultiplied ARGB
    PremultipliedFirst,
    /// For example, non-premultiplied RGBA
    Last,
    /// For example, non-premultiplied ARGB
    First,
    /// For example, RBGX
    NoneSkipLast,
    /// For example, XRGB
    NoneSkipFirst,
    Only,
}

define_cf_type!(Image(cf::Type));

unsafe impl Send for Image {}

impl Image {
    /// Return true if `image' is an image mask, false otherwise.`
    #[inline]
    pub fn is_mask(&self) -> bool {
        unsafe { CGImageIsMask(self) }
    }

    #[inline]
    pub fn width(&self) -> usize {
        unsafe { CGImageGetWidth(self) }
    }

    #[inline]
    pub fn height(&self) -> usize {
        unsafe { CGImageGetHeight(self) }
    }

    #[inline]
    pub fn alpha_info(&self) -> AlphaInfo {
        unsafe { CGImageGetAlphaInfo(self) }
    }

    #[inline]
    pub fn ut_type(&self) -> Option<&cf::String> {
        unsafe { CGImageGetUTType(self) }
    }

    /// Return the color space of `image', or None if `image' is an image
    /// mask.
    #[inline]
    pub fn color_space(&self) -> Option<&cg::ColorSpace> {
        unsafe { CGImageGetColorSpace(self) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C-unwind" {
    fn CGImageIsMask(image: &Image) -> bool;
    fn CGImageGetWidth(image: &Image) -> usize;
    fn CGImageGetHeight(image: &Image) -> usize;
    fn CGImageGetAlphaInfo(image: &Image) -> AlphaInfo;
    fn CGImageGetUTType(image: &Image) -> Option<&cf::String>;
    fn CGImageGetColorSpace(image: &Image) -> Option<&cg::ColorSpace>;
}
