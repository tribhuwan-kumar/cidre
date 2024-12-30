mod device;
pub use device::notifications as device_notifications;
pub use device::BatteryState as DeviceBatteryState;
pub use device::Device;
pub use device::Orientation as DeviceOrientation;
pub use device::UserInterfaceIdiom;

mod view;
pub use view::View;

mod responder;
pub use responder::Responder;

mod view_controller;
pub use view_controller::ViewController;

mod color;
pub use color::Color;

mod image;
pub use image::Image;

mod interaction;
pub use interaction::AnyInteraction;
pub use interaction::Interaction;
pub use interaction::InteractionImpl;

mod scene_definitions;
pub use scene_definitions::SceneSessionRole;

mod scene;
pub use scene::AnySceneDelegate;
pub use scene::Scene;
pub use scene::SceneDelegate;

mod scene_session;
pub use scene_session::SceneCfg;
pub use scene_session::SceneSession;

mod scene_options;
pub use scene_options::SceneConnectionOpts;

pub fn app_main(
    principal_class_name: Option<&crate::ns::String>,
    delegate_class_name: Option<&crate::ns::String>,
) -> std::ffi::c_int {
    unsafe {
        UIApplicationMain(
            0,
            std::ptr::null(),
            principal_class_name,
            delegate_class_name,
        )
    }
}

extern "C" {
    fn UIApplicationMain(
        argc: std::ffi::c_int,
        argv: *const *const std::ffi::c_char,
        principal_class_name: Option<&crate::ns::String>,
        delegate_class_name: Option<&crate::ns::String>,
    ) -> std::ffi::c_int;
}
