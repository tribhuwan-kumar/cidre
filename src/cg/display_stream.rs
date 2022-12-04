use std::ffi::c_void;

use crate::{blocks, cf, cg, define_cf_type, dispatch, io};

define_cf_type!(DisplayStream(cf::Type));
define_cf_type!(Update(cf::Type));

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum UpdateRectType {
    /// The rectangles that were refreshed on the display, not counting moved rectangles
    RefreshedRects,

    /// The rectangles that were simply moved from one part of the display to another
    MovedRects,

    /// The union of both refreshed and moved rects
    DirtyRects,

    /// A possibly simplified (but overstated) array of dirty rectangles
    ReducedDirtyRects,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum FrameStatus {
    /// A new frame has been generated by the Window Server for a particular display at time displayTime
    FrameComplete,

    /// The Window Server did not generate a new frame for displayTime
    FrameIdle,

    /// As of displayTime, the display has gone blank
    FrameBlank,

    /// he display stream has stopped and no more calls will be made to the handler until the stream is started.
    StatusStopped,
}

impl Update {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CGDisplayStreamUpdateGetTypeID() }
    }

    #[inline]
    pub fn merged(first: Option<&Self>, second: Option<&Self>) -> Option<cf::Retained<Self>> {
        unsafe { CGDisplayStreamUpdateCreateMergedUpdate(first, second) }
    }

    /// Returns a pointer to an array of CGRect structs that describe what parts of the frame have changed relative
    /// to the previously delivered frame.   This rectangle list encapsulates both the update rectangles and movement rectangles.
    pub unsafe fn get_rects(
        &self,
        rect_type: UpdateRectType,
        rect_count: *mut usize,
    ) -> *const cg::Rect {
        unsafe { CGDisplayStreamUpdateGetRects(self, rect_type, rect_count) }
    }

    pub unsafe fn moved_rect_delta(&self, x: *mut cg::Float, y: *mut cg::Float) {
        CGDisplayStreamUpdateGetMovedRectsDelta(self, x, y)
    }

    #[inline]
    pub fn drop_count(&self) -> usize {
        unsafe { CGDisplayStreamUpdateGetDropCount(self) }
    }
}

define_cf_type!(PropertyKey(cf::String));

impl PropertyKey {
    /// This may be used to request a subregion of the display to be provided as the source of the display stream.  Use
    /// CGRectCreateDictionaryRepresentation to convert from a cg::Rect to the value used here.   Note: The coordinate system for the
    /// source rectangle is specified in display logical coordinates and not in pixels, in order to match the normal convention on
    /// HiDPI displays.
    pub fn source_rect() -> &'static Self {
        unsafe { kCGDisplayStreamSourceRect }
    }

    /// This may be used to request where within the destination buffer the display updates should be placed. Use
    /// CGRectCreateDictionaryRepresentation to convert from a CGRect to the value used here.   Note: The coordinate system for
    /// the destination rectangle is always specified in output pixels to match the fact that the output buffer size is also
    /// specified in terms of pixels.
    pub fn destination_rect() -> &'static Self {
        unsafe { kCGDisplayStreamDestinationRect }
    }

    /// Enable/disable the work the Window Server will do to preserve the display aspect ratio.  By default the Window Server will
    /// assume that it should preserve the original aspect ratio of the source display rect.  If the aspect ratio of the source display and
    /// he display stream destination rect are not the same, black borders will be inserted at the top/bottom or right/left sides of the destination
    /// in order to preserve the source aspect ratio.
    ///
    /// cf::Boolean - defaults to true
    pub fn preserve_aspect_ratio() -> &'static Self {
        unsafe { kCGDisplayStreamPreserveAspectRatio }
    }

    /// Set the desired cg::ColorSpace of the output frames.  By default the color space will be that of the display.
    ///
    /// Desired output color space (cg::ColorSpaceRef) - defaults to display color space
    pub fn color_space() -> &'static Self {
        unsafe { kCGDisplayStreamColorSpace }
    }

    /// Request that the delta between frame updates be at least as much specified by this value.
    ///
    /// cf::Number in seconds, defaults to zero.
    pub fn minimum_frame_rate() -> &'static Self {
        unsafe { kCGDisplayStreamMinimumFrameTime }
    }

    /// Controls whether the cursor is embedded within the provided buffers or not.
    ///
    /// cf::Boolean - defaults to false
    pub fn show_cursor() -> &'static Self {
        unsafe { kCGDisplayStreamShowCursor }
    }

    /// Controls how many frames deep the frame queue will be.  Defaults to N.
    ///
    /// Queue depth in frames.  Defaults to 3.
    pub fn queue_depth() -> &'static Self {
        unsafe { kCGDisplayStreamQueueDepth }
    }

    /// When outputting frames in 420v or 420f format, this key may be used to control which YCbCrMatrix is used
    pub fn ycbcr_matric() -> &'static Self {
        unsafe { kCGDisplayStreamYCbCrMatrix }
    }
}

define_cf_type!(YCbCrMatrix(cf::String));

/// Supported YCbCr matrices. Note that these strings have identical values to the equivalent CoreVideo strings.
impl YCbCrMatrix {
    pub fn itu_r_709_2() -> &'static Self {
        unsafe { kCGDisplayStreamYCbCrMatrix_ITU_R_709_2 }
    }

    pub fn itu_r_601_4() -> &'static Self {
        unsafe { kCGDisplayStreamYCbCrMatrix_ITU_R_601_4 }
    }

    pub fn smpte_240m_1995() -> &'static Self {
        unsafe { kCGDisplayStreamYCbCrMatrix_SMPTE_240M_1995 }
    }
}

/// Provides a streaming API for capturing display updates in a realtime manner.  It can also provide
/// scaling and color space conversion services, as well as allow capturing sub regions of the display. Callbacks can be targetted
/// at either a traditional cf::RunLoop, or at a dispatch::queue.
impl DisplayStream {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CGDisplayStreamGetTypeID() }
    }

    #[inline]
    pub unsafe fn create(
        display: cg::DirectDisplayID,
        output_width: usize,
        output_height: usize,
        pixel_format: i32,
        properties: Option<&cf::DictionaryOf<PropertyKey, cf::PropertyList>>,
        handler: *mut c_void,
    ) -> Option<cf::Retained<DisplayStream>> {
        unsafe {
            CGDisplayStreamCreate(
                display,
                output_width,
                output_height,
                pixel_format,
                properties,
                handler,
            )
        }
    }

    #[inline]
    pub unsafe fn create_with_queue(
        display: cg::DirectDisplayID,
        output_width: usize,
        output_height: usize,
        pixel_format: i32,
        properties: Option<&cf::DictionaryOf<PropertyKey, cf::PropertyList>>,
        queue: &dispatch::Queue,
        handler: *mut c_void,
    ) -> Option<cf::Retained<DisplayStream>> {
        unsafe {
            CGDisplayStreamCreateWithDispatchQueue(
                display,
                output_width,
                output_height,
                pixel_format,
                properties,
                queue,
                handler,
            )
        }
    }

    pub fn with_runloop<'ar, F>(
        display: cg::DirectDisplayID,
        output_width: usize,
        output_height: usize,
        pixel_format: i32,
        properties: Option<&cf::DictionaryOf<PropertyKey, cf::PropertyList>>,
        handler: &mut blocks::Block<F>,
    ) -> Option<cf::Retained<DisplayStream>>
    where
        F: FnMut(FrameStatus, u64, Option<&'ar io::Surface>, Option<&'ar Update>),
    {
        unsafe {
            Self::create(
                display,
                output_width,
                output_height,
                pixel_format,
                properties,
                handler.as_ptr(),
            )
        }
    }

    /// 'BGRA' Packed Little Endian ARGB8888
    /// 'l10r' Packed Little Endian ARGB2101010
    /// '420v' 2-plane "video" range YCbCr 4:2:0
    /// '420f' 2-plane "full" range YCbCr 4:2:0
    pub fn with_dispatch_queue<'ar, F>(
        display: cg::DirectDisplayID,
        output_width: usize,
        output_height: usize,
        pixel_format: i32,
        properties: Option<&cf::DictionaryOf<PropertyKey, cf::PropertyList>>,
        queue: &dispatch::Queue,
        handler: &mut blocks::Block<F>,
    ) -> Option<cf::Retained<DisplayStream>>
    where
        F: FnMut(FrameStatus, u64, Option<&'ar io::Surface>, Option<&'ar Update>),
    {
        unsafe {
            Self::create_with_queue(
                display,
                output_width,
                output_height,
                pixel_format,
                properties,
                queue,
                handler.as_ptr(),
            )
        }
    }

    #[inline]
    pub unsafe fn stream_start(&self) -> cg::Error {
        CGDisplayStreamStart(self)
    }

    pub fn start(&self) -> Result<(), cg::Error> {
        unsafe {
            match self.stream_start() {
                cg::Error::SUCCESS => Ok(()),
                e => Err(e),
            }
        }
    }

    #[inline]
    pub unsafe fn stream_stop(&self) -> cg::Error {
        CGDisplayStreamStop(self)
    }

    pub fn stop(&self) -> Result<(), cg::Error> {
        unsafe {
            match self.stream_stop() {
                cg::Error::SUCCESS => Ok(()),
                e => Err(e),
            }
        }
    }

    #[inline]
    pub fn run_loop_source(&self) -> Option<&cf::RunLoop> {
        unsafe { CGDisplayStreamGetRunLoopSource(self) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGDisplayStreamUpdateGetTypeID() -> cf::TypeId;

    fn CGDisplayStreamUpdateGetRects(
        update_ref: &Update,
        rect_type: UpdateRectType,
        rect_count: *mut usize,
    ) -> *const cg::Rect;

    fn CGDisplayStreamUpdateCreateMergedUpdate(
        first: Option<&Update>,
        second: Option<&Update>,
    ) -> Option<cf::Retained<Update>>;

    fn CGDisplayStreamUpdateGetMovedRectsDelta(
        update_ref: &Update,
        x: *mut cg::Float,
        y: *mut cg::Float,
    );
    fn CGDisplayStreamUpdateGetDropCount(update_ref: &Update) -> usize;

    static kCGDisplayStreamSourceRect: &'static PropertyKey;
    static kCGDisplayStreamDestinationRect: &'static PropertyKey;
    static kCGDisplayStreamPreserveAspectRatio: &'static PropertyKey;
    static kCGDisplayStreamColorSpace: &'static PropertyKey;
    static kCGDisplayStreamMinimumFrameTime: &'static PropertyKey;
    static kCGDisplayStreamShowCursor: &'static PropertyKey;
    static kCGDisplayStreamQueueDepth: &'static PropertyKey;
    static kCGDisplayStreamYCbCrMatrix: &'static PropertyKey;

    static kCGDisplayStreamYCbCrMatrix_ITU_R_709_2: &'static YCbCrMatrix;
    static kCGDisplayStreamYCbCrMatrix_ITU_R_601_4: &'static YCbCrMatrix;
    static kCGDisplayStreamYCbCrMatrix_SMPTE_240M_1995: &'static YCbCrMatrix;

    fn CGDisplayStreamGetTypeID() -> cf::TypeId;

    fn CGDisplayStreamCreate(
        display: cg::DirectDisplayID,
        output_width: usize,
        output_height: usize,
        pixel_format: i32,
        properties: Option<&cf::DictionaryOf<PropertyKey, cf::PropertyList>>,
        handler: *mut c_void,
    ) -> Option<cf::Retained<DisplayStream>>;

    fn CGDisplayStreamCreateWithDispatchQueue(
        display: cg::DirectDisplayID,
        output_width: usize,
        output_height: usize,
        pixel_format: i32,
        properties: Option<&cf::DictionaryOf<PropertyKey, cf::PropertyList>>,
        queue: &dispatch::Queue,
        handler: *mut c_void,
    ) -> Option<cf::Retained<DisplayStream>>;

    fn CGDisplayStreamStart(stream: &DisplayStream) -> cg::Error;
    fn CGDisplayStreamStop(stream: &DisplayStream) -> cg::Error;

    fn CGDisplayStreamGetRunLoopSource(stream: &DisplayStream) -> Option<&cf::RunLoop>;
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    #[test]
    fn basics() {
        use crate::{blocks, cg, dispatch};

        let mut block = blocks::mut4(|frame_status, timestamp, surface, _update| {
            println!("got! {timestamp:?} {frame_status:?} {surface:?}")
        });

        let queue = dispatch::Queue::global(0).unwrap();

        let stream = cg::DisplayStream::with_dispatch_queue(
            cg::main_display_id(),
            640,
            480,
            i32::from_be_bytes(*b"420f"),
            None,
            queue,
            block.escape(),
        )
        .unwrap();

        stream.start().unwrap();

        sleep(Duration::from_secs(1));

        stream.stop().unwrap();
    }
}
