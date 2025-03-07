use crate::{define_obj_type, ns, objc};

define_obj_type!(pub Level(ns::String));

impl Level {
    #[inline]
    pub fn nominal() -> &'static Self {
        unsafe { AVCaptureSystemPressureLevelNominal }
    }

    #[inline]
    pub fn fair() -> &'static Self {
        unsafe { AVCaptureSystemPressureLevelFair }
    }

    #[inline]
    pub fn serious() -> &'static Self {
        unsafe { AVCaptureSystemPressureLevelSerious }
    }

    #[inline]
    pub fn critical() -> &'static Self {
        unsafe { AVCaptureSystemPressureLevelCritical }
    }

    #[inline]
    pub fn shutdown() -> &'static Self {
        unsafe { AVCaptureSystemPressureLevelShutdown }
    }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum Factors {
    None = 0,
    SystemTemperature = 1 << 0,
    PeakPower = 1 << 1,
    DepthModuleTemperature = 1 << 2,
}

define_obj_type!(pub State(ns::Id));

impl State {
    #[objc::msg_send(level)]
    pub fn level(&self) -> &Level;

    #[objc::msg_send(factors)]
    pub fn factors(&self) -> Factors;
}

#[link(name = "AVFoundation", kind = "framework")]
unsafe extern "C" {
    static AVCaptureSystemPressureLevelNominal: &'static Level;
    static AVCaptureSystemPressureLevelFair: &'static Level;
    static AVCaptureSystemPressureLevelSerious: &'static Level;
    static AVCaptureSystemPressureLevelCritical: &'static Level;
    static AVCaptureSystemPressureLevelShutdown: &'static Level;
}
