use crate::{arc, av, cg, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "cm")]
use crate::cm;

#[cfg(feature = "dispatch")]
use crate::dispatch;

/// Constants indicating video orientation, for use with
/// [`av::CaptureVideoPreviewLayer`] and [`av::CaptureConnection`].
///
/// [Apple Documentation](https://developer.apple.com/documentation/avfoundation/avcapturevideoorientation?language=objc)
#[doc(alias = "AVCaptureVideoOrientation")]
// #[deprecated(since = "0.1.0", note = "See videoRotationAngle instead.")]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(isize)]
pub enum VideoOrienation {
    /// Indicates that video should be oriented vertically, home button on the bottom.
    Portrait = 1,

    /// Indicates that video should be oriented vertically, home button on the top.
    PortraitUpsideDown = 2,

    /// Indicates that video should be oriented horizontally, home button on the right.
    LandscapeRight = 3,

    /// Indicates that video should be oriented horizontally, home button on the left.
    LandscapeLeft = 4,
}

/// Constants indicating interruption reason. One of these is returned with the
/// AVCaptureSessionWasInterruptedNotification (see [`InterruptionResason::key()`]).
#[cfg(not(any(target_os = "macos", target_os = "watchos")))]
#[doc(alias = "AVCaptureSessionInterruptionReason")]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(isize)]
pub enum InterruptionReason {
    VideoDeviceNotAvailableInBackground = 1,
    AudioDeviceInUseByAnotherClient = 2,
    VideoDeviceInUseByAnotherClient = 3,
    VideoDeviceNotAvailableWithMultipleForegroundApps = 4,
    VideoDeviceNotAvailableDueToSystemPressure = 5,
}

#[cfg(not(any(target_os = "macos", target_os = "watchos")))]
impl InterruptionReason {
    #[doc(alias = "AVCaptureSessionInterruptionReasonKey")]
    pub fn key() -> &'static ns::String {
        #[link(name = "AVFoundation", kind = "framework")]
        unsafe extern "C" {
            static AVCaptureSessionInterruptionReasonKey: &'static ns::String;
        }

        unsafe { AVCaptureSessionInterruptionReasonKey }
    }
}

define_obj_type!(
    /// An object that configures capture behavior and coordinates the flow of data
    /// from input devices to capture outputs.
    #[doc(alias = "AVCaptureSession")]
    pub Session(ns::Id),
    AV_CAPTURE_SESSION
);

impl Session {
    #[objc::msg_send(canSetSessionPreset:)]
    pub fn can_set_session_preset(&self, preset: &av::CaptureSessionPreset) -> bool;

    #[objc::msg_send(sessionPreset)]
    pub fn session_preset(&self) -> &av::CaptureSessionPreset;

    #[objc::msg_send(setSessionPreset:)]
    pub unsafe fn set_session_preset_throws(&self, val: &av::CaptureSessionPreset);

    pub fn set_session_preset<'ear>(&self, val: &av::CaptureSessionPreset) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.set_session_preset_throws(val) })
    }

    #[objc::msg_send(inputs)]
    pub fn inputs(&self) -> arc::R<ns::Array<av::CaptureInput>>;

    #[objc::msg_send(canAddInput:)]
    pub fn can_add_input(&self, input: &av::CaptureInput) -> bool;

    #[objc::msg_send(addInput:)]
    pub fn add_input(&mut self, input: &av::CaptureInput);

    #[objc::msg_send(removeInput:)]
    pub fn remove_input(&mut self, input: &av::CaptureInput);

    #[objc::msg_send(outputs)]
    pub fn outputs(&self) -> arc::R<ns::Array<av::CaptureOutput>>;

    #[objc::msg_send(canAddOutput:)]
    pub fn can_add_output(&self, output: &av::CaptureOutput) -> bool;

    #[objc::msg_send(addOutput:)]
    pub fn add_output(&mut self, output: &av::CaptureOutput);

    #[objc::msg_send(removeOutput:)]
    pub fn remove_output(&mut self, output: &av::CaptureOutput);

    #[objc::msg_send(addInputWithNoConnections:)]
    #[objc::available(ios = 8.0, maccatalyst = 14.0, tvos = 17.0, visionos = 2.1)]
    pub fn add_input_without_connections(&mut self, input: &av::CaptureInput);

    #[objc::msg_send(addOutputWithNoConnections:)]
    #[objc::available(ios = 8.0, maccatalyst = 14.0, tvos = 17.0, visionos = 2.1)]
    pub fn add_output_without_connections(&mut self, output: &av::CaptureOutput);

    #[objc::msg_send(connections)]
    pub fn connections(&self) -> arc::R<ns::Array<av::CaptureConnection>>;

    #[objc::msg_send(canAddConnection:)]
    #[objc::available(ios = 8.0, maccatalyst = 14.0, tvos = 17.0, visionos = 2.1)]
    pub fn can_add_connection(&self, connection: &av::CaptureConnection) -> bool;

    #[objc::msg_send(addConnection:)]
    #[objc::available(ios = 8.0, maccatalyst = 14.0, tvos = 17.0, visionos = 2.1)]
    pub fn add_connection(&mut self, connection: &av::CaptureConnection);

    #[objc::msg_send(removeConnection:)]
    #[objc::available(ios = 8.0, maccatalyst = 14.0, tvos = 17.0, visionos = 2.1)]
    pub fn remove_connection(&mut self, connection: &av::CaptureConnection);

    /// Indicates whether session controls are supported on this platform.
    #[objc::msg_send(supportsControls)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn supports_controls(&self) -> bool;

    /// Specifies the maximum number of controls that can be added to a session.
    #[objc::msg_send(maxControlsCount)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn max_controls_count(&self) -> isize;

    #[objc::msg_send(controlsDelegate)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn controls_delegate(&self) -> Option<arc::R<AnyControlsDelegate>>;

    #[cfg(feature = "dispatch")]
    #[objc::msg_send(setControlsDelegate:queue:)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn set_controls_delegate<D: ControlsDelegate>(
        &mut self,
        val: Option<&D>,
        queue: Option<&dispatch::Queue>,
    );

    #[cfg(feature = "dispatch")]
    #[objc::msg_send(controlsDelegateCallbackQueue)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn controls_delegate_cb_queue(&self) -> Option<arc::R<dispatch::Queue>>;

    #[objc::msg_send(controls)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn controls(&self) -> arc::R<ns::Array<av::CaptureControl>>;

    /// Returns whether the proposed control can be added to the session.
    #[objc::msg_send(canAddControl:)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn can_add_control(&self, val: &av::CaptureControl) -> bool;

    #[objc::msg_send(addControl:)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub unsafe fn add_control_throws(&mut self, val: &av::CaptureControl);

    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn add_control<'ear>(&mut self, val: &av::CaptureControl) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.add_control_throws(val) })
    }

    #[objc::msg_send(removeControl:)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn remove_control(&mut self, val: &av::CaptureControl);

    #[objc::msg_send(beginConfiguration)]
    pub fn begin_cfg(&mut self);

    #[objc::msg_send(commitConfiguration)]
    pub fn commit_cfg(&mut self);

    /// Batched configuration
    pub fn configure<F: FnMut(&mut Self)>(&mut self, mut config: F) {
        self.begin_cfg();
        config(self);
        self.commit_cfg();
    }

    /// Indicates whether the session is currently running.
    ///
    /// The value of this property is a BOOL indicating whether the receiver is running.
    /// Clients can key value observe the value of this property to be notified when
    /// the session automatically starts or stops running.
    #[objc::msg_send(isRunning)]
    pub fn is_running(&self) -> bool;

    /// Indicates whether the session is being interrupted.
    ///
    /// The value of this property is a BOOL indicating whether the receiver is currently
    /// being interrupted, such as by a phone call or alarm. Clients can key value observe
    /// the value of this property to be notified when the session ceases to be interrupted
    /// and again has access to needed hardware resources.
    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(isInterrupted)]
    pub fn is_interrupted(&self) -> bool;

    /// Returns whether the session can be configured to use the camera while multitasking.
    ///
    /// This property can be used to determine whether multitasking_camera_access_enabled may be set to true.
    /// When this property changes from true to false, multitasking_camera_access_enabled also reverts to false.
    /// Prior to iOS 18, this property returns true on iPads that support Stage Manager with an extended display.
    /// In applications linked on or after iOS 18, this property returns true for video conferencing applications
    /// (apps that use "voip" as one of their UIBackgroundModes).
    /// This property also returns true for iOS applications that have the
    /// com.apple.developer.avfoundation.multitasking-camera-access entitlement.
    /// This property returns true on Apple TV.
    /// This property is key-value observable.
    #[objc::msg_send(isMultitaskingCameraAccessSupported)]
    #[objc::available(ios = 16.0, tvos = 17.0)]
    pub fn is_multitasking_camera_access_supported(&self) -> bool;

    #[objc::msg_send(isMultitaskingCameraAccessEnabled)]
    #[objc::available(ios = 16.0, tvos = 17.0)]
    pub fn is_multitasking_camera_access_enabled(&self) -> bool;

    #[objc::msg_send(setMultitaskingCameraAccessEnabled:)]
    #[objc::available(ios = 16.0, tvos = 17.0)]
    pub fn set_multitasking_camera_access_enabled(&mut self, val: bool);

    #[objc::msg_send(usesApplicationAudioSession)]
    #[objc::available(ios = 7.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn uses_app_audio_session(&self) -> bool;

    #[objc::msg_send(setUsesApplicationAudioSession:)]
    #[objc::available(ios = 7.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn set_uses_app_audio_session(&mut self, value: bool);

    #[objc::msg_send(automaticallyConfiguresApplicationAudioSession)]
    #[objc::available(ios = 7.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn automatically_configures_app_audio_session(&self) -> bool;

    #[objc::msg_send(setAutomaticallyConfiguresApplicationAudioSession:)]
    #[objc::available(ios = 7.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn set_automatically_configures_app_audio_session(&mut self, val: bool);

    /// Indicates whether the receiver should configure the application's audio session to mix with others.
    ///
    /// The value of this property is a bool indicating whether the receiver should configure
    /// the application's audio session to mix with, instead of interrupting, any ongoing audio
    /// sessions. It has no effect when uses_application_audio_session is set to false. It also
    /// has no effect on Live Photo movie complement capture (where music is always mixed with).
    /// The default value is false.
    #[objc::msg_send(configuresApplicationAudioSessionToMixWithOthers)]
    #[objc::available(ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn configures_application_audio_session_to_mix_with_others(&self) -> bool;

    #[objc::msg_send(setConfiguresApplicationAudioSessionToMixWithOthers:)]
    #[objc::available(ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn set_configures_application_audio_session_to_mix_with_others(&mut self, val: bool);

    #[objc::msg_send(automaticallyConfiguresCaptureDeviceForWideColor)]
    #[objc::available(ios = 10.0, maccatalyst = 14.0, tvos = 17.0)]
    pub fn automatically_configures_capture_device_for_wide_color(&self) -> bool;

    #[objc::msg_send(setAutomaticallyConfiguresCaptureDeviceForWideColor:)]
    #[objc::available(ios = 10.0, maccatalyst = 14.0, tvos = 17.0)]
    pub fn set_automatically_configures_capture_device_for_wide_color(&mut self, val: bool);

    #[objc::msg_send(startRunning)]
    pub fn start_running(&mut self);

    #[objc::msg_send(stopRunning)]
    pub fn stop_running(&mut self);

    #[cfg(feature = "cm")]
    #[objc::msg_send(synchronizationClock)]
    pub fn sync_clock(&self) -> Option<&cm::Clock>;

    /// Indicates the percentage of the session's available hardware budget currently in use.
    ///
    /// The value of this property is a float from 0.0 => 1.0 indicating how much of the
    /// session's available hardware is in use as a percentage, given the currently connected
    /// inputs and outputs and the features for which you've opted in. When your hw_cost is greater
    /// than 1.0, the capture session cannot run your desired configuration due to hardware
    /// constraints, so you receive an AVCaptureSessionRuntimeErrorNotification when attempting
    /// to start it running. Default value is 0.
    #[objc::msg_send(hardwareCost)]
    #[objc::available(ios = 16.0, maccatalyst = 16.0, tvos = 17.0)]
    pub fn hw_cost(&self) -> f32;
}

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_CAPTURE_SESSION: &'static objc::Class<Session>;
    static AV_CAPTURE_MULTI_CAM_SESSION: &'static objc::Class<MultiCamSession>;
    static AV_CAPTURE_CONNECTION: &'static objc::Class<Connection>;
}

#[objc::protocol(AVCaptureSessionControlsDelegate)]
pub trait ControlsDelegate: objc::Obj {
    /// Called when the controls of an `av::CaptureSession` instance become active and are
    /// available for interaction.
    ///
    /// Delegates receive this message when the controls of an `av::CaptureSession` instance become active and are available for interaction.
    #[objc::msg_send(sessionControlsDidBecomeActive:)]
    fn session_controls_did_become_active(&mut self, session: &mut av::CaptureSession);

    /// Called when the controls of an `av::CaptureSession` instance will enter a fullscreen
    /// appearance.
    ///
    /// When the controls enter a fullscreen appearance, applications are encouraged to hide portions of their user interface including zoom or
    /// exposure sliders and shutter buttons. Few on-screen elements should be visible so users can focus on the controls they are interacting with and view the camera preview unobstructed.
    #[objc::msg_send(sessionControlsWillEnterFullscreenAppearance:)]
    fn session_controls_will_enter_fullscreen_appearance(
        &mut self,
        session: &mut av::CaptureSession,
    );

    /// Called when the controls of an `av::CaptureSession` instance will exit a fullscreen
    /// appearance.
    #[objc::msg_send(sessionControlsWillExitFullscreenAppearance:)]
    fn session_controls_will_exit_fullscreen_appearance(
        &mut self,
        session: &mut av::CaptureSession,
    );

    #[objc::msg_send(sessionControlsDidBecomeInactive:)]
    fn session_controls_did_become_inactive(&mut self, session: &mut av::CaptureSession);
}

define_obj_type!(pub AnyControlsDelegate(ns::Id));

impl ControlsDelegate for AnyControlsDelegate {}

define_obj_type!(
    /// A subclass of [`av::CaptureSession`] which supports simultaneous capture from
    /// multiple inputs of the same media type.
    #[doc(alias = "AVCaptureMultiCamSession")]
    pub MultiCamSession(Session),
    AV_CAPTURE_MULTI_CAM_SESSION
);

impl MultiCamSession {
    /// ```no_run
    /// use cidre::av;
    ///
    /// assert!(!av::CaptureMultiCamSession::is_multicam_supported());
    /// ```
    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(isMultiCamSupported)]
    pub fn is_multicam_supported() -> bool;

    #[cfg(target_os = "macos")]
    pub fn is_multicam_supported() -> bool {
        false
    }

    /// The value of this property is a float from 0.0 => 1.0 indicating
    /// how much of the session's available hardware is in use as a percentage,
    /// given the currently connected inputs and outputs and the features for
    /// which you've opted in. When your hardwareCost is greater than 1.0,
    /// the capture session cannot run your desired configuration due to
    /// hardware constraints, so you receive an AVCaptureSessionRuntimeErrorNotification
    /// when attempting to start it running. Default value is 0.
    /// Contributors to hardwareCost include:
    /// - Whether the source devices' active formats use the full
    ///   sensor (4:3) or a crop (16:9). Cropped formats require lower
    ///   hardware bandwidth, and therefore lower the cost.
    /// - The max frame rate supported by the source devices' active formats.
    ///   The higher the max frame rate, the higher the cost.
    /// - Whether the source devices' active formats are binned or not.
    ///   Binned formats require substantially less hardware bandwidth,
    ///   and therefore result in a lower cost.
    /// - The number of sources configured to deliver streaming
    ///   disparity / depth via AVCaptureDepthDataOutput. The higher the number
    ///   of cameras configured to produce depth, the higher the cost.
    ///   In order to reduce hardwareCost, consider picking a sensor-cropped
    ///   activeFormat, or a binned format.
    ///   You may also use [`av::CaptureDeviceInput`]'s videoMinFrameDurationOverride
    ///   property to artificially limit the max frame rate (which is the
    ///   reciprocal of the min frame duration) of a source device to a lower value.
    ///   By doing so, you only pay the hardware cost for the max frame rate you intend to use.
    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(hardwareCost)]
    pub fn hw_cost(&self) -> f32;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(systemPressureCost)]
    pub fn sys_pressure_cost(&self) -> f32;
}

define_obj_type!(
    /// An object that represents a connection from a capture input to a capture output.
    #[doc(alias = "AVCaptureConnection")]
    pub Connection(ns::Id)
);

impl arc::A<Connection> {
    #[objc::msg_send(initWithInputPorts:output:)]
    pub fn init_with_ports(
        self,
        input: &ns::Array<av::CaptureInputPort>,
        output: &av::CaptureOutput,
    ) -> arc::R<Connection>;

    #[objc::msg_send(initWithInputPort:videoPreviewLayer:)]
    pub fn init_with_port_preview_layer(
        self,
        input: &av::CaptureInputPort,
        layer: &av::CaptureVideoPreviewLayer,
    ) -> arc::R<Connection>;
}

impl Connection {
    define_cls!(AV_CAPTURE_CONNECTION);

    pub fn with_ports(
        input: &ns::Array<av::CaptureInputPort>,
        output: &av::CaptureOutput,
    ) -> arc::R<Self> {
        Self::alloc().init_with_ports(input, output)
    }

    pub fn with_preview_layer(
        input: &av::CaptureInputPort,
        layer: &av::CaptureVideoPreviewLayer,
    ) -> arc::R<Connection> {
        Self::alloc().init_with_port_preview_layer(input, layer)
    }

    #[objc::msg_send(inputPorts)]
    pub fn input_ports(&self) -> arc::R<ns::Array<av::CaptureInputPort>>;

    #[objc::msg_send(output)]
    pub fn output(&self) -> Option<arc::R<av::CaptureOutput>>;

    #[objc::msg_send(videoPreviewLayer)]
    pub fn video_preview_layer(&self) -> Option<arc::R<av::CaptureVideoPreviewLayer>>;

    /// An array of audio channels that the connection provides.
    #[objc::msg_send(audioChannels)]
    pub fn audio_channels(&self) -> arc::R<ns::Array<AudioChannel>>;

    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    #[objc::msg_send(isActive)]
    pub fn is_active(&self) -> bool;

    #[objc::msg_send(isVideoMirroringSupported)]
    pub fn is_video_mirroring_supported(&self) -> bool;

    #[objc::msg_send(isVideoMirrored)]
    pub fn is_video_mirrored(&self) -> bool;

    #[objc::msg_send(setVideoMirrored:)]
    pub fn set_video_mirrored(&mut self, val: bool);

    #[objc::msg_send(automaticallyAdjustsVideoMirroring)]
    pub fn automatically_adjusts_video_mirroring(&self) -> bool;

    #[objc::msg_send(setAutomaticallyAdjustsVideoMirroring:)]
    pub fn set_automatically_adjusts_video_mirroring(&mut self, val: bool);

    #[objc::msg_send(isVideoRotationAngleSupported:)]
    pub fn is_video_rotation_angle_supported(&self, angle: cg::Float) -> bool;

    #[objc::msg_send(videoRotationAngle)]
    pub fn video_rotation_angle(&self) -> cg::Float;

    #[objc::msg_send(setVideoRotationAngle:)]
    pub unsafe fn set_video_rotation_angle_throws(&mut self, val: cg::Float);

    pub fn set_video_rotation_angle<'ear>(&mut self, val: cg::Float) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.set_video_rotation_angle_throws(val) })
    }

    #[cfg(target_os = "macos")]
    #[objc::msg_send(isVideoFieldModeSupported)]
    pub fn is_video_field_mode_supported(&self) -> bool;

    //...

    #[objc::msg_send(videoMaxScaleAndCropFactor)]
    pub fn video_max_scale_and_crop_factor(&self) -> cg::Float;

    #[objc::msg_send(videoScaleAndCropFactor)]
    pub fn video_scale_and_crop_factor(&self) -> cg::Float;

    #[objc::msg_send(setVideoScaleAndCropFactor:)]
    pub fn set_video_scale_and_crop_factor(&mut self, val: cg::Float);

    #[objc::msg_send(preferredVideoStabilizationMode)]
    pub fn preferred_video_stabilization_mode(&self) -> av::CaptureVideoStabilizationMode;

    #[cfg(any(target_os = "ios", target_os = "tvos"))]
    #[objc::msg_send(setPreferredVideoStabilizationMode:)]
    pub fn set_preferred_video_stabilization_mode(
        &mut self,
        val: av::CaptureVideoStabilizationMode,
    );

    #[cfg(any(target_os = "ios", target_os = "tvos"))]
    #[objc::msg_send(activeVideoStabilizationMode)]
    pub fn active_video_stabilization_mode(&self) -> av::CaptureVideoStabilizationMode;

    #[cfg(any(target_os = "ios", target_os = "tvos"))]
    #[objc::msg_send(setActiveVideoStabilizationMode:)]
    pub fn set_active_video_stabilization_mode(&mut self, val: av::CaptureVideoStabilizationMode);

    #[cfg(any(target_os = "ios", target_os = "tvos"))]
    #[objc::msg_send(isCameraIntrinsicMatrixDeliverySupported)]
    pub fn is_camera_intrinsic_matrix_delivery_supported(&self) -> bool;

    #[cfg(any(target_os = "ios", target_os = "tvos"))]
    #[objc::msg_send(isCameraIntrinsicMatrixDeliveryEnabled)]
    pub fn is_camera_intrinsic_matrix_delivery_enabled(&self) -> bool;

    #[cfg(any(target_os = "ios", target_os = "tvos"))]
    #[objc::msg_send(setCameraIntrinsicMatrixDeliveryEnabled:)]
    pub fn set_camera_intrinsic_matrix_delivery_enabled(&mut self, val: bool);
}

define_obj_type!(
    /// Represents a single channel of audio flowing through an [`av::CaptureSession`]
    ///
    /// An [`av::CaptureConnection`] from an input producing audio to an output receiving
    /// audio exposes an array of [`av::CaptureAudioChannel`] objects, one for each channel of audio available.
    /// Iterating through these audio channel objects, a client may poll for audio levels.
    /// Instances of [`av::CaptureAudioChannel`] cannot be created directly.
    #[doc(alias = "AVCaptureAudioChannel")]
    pub AudioChannel(ns::Id)
);

impl AudioChannel {
    /// A measurement of the instantaneous average power level of the audio flowing through
    /// the receiver.
    ///
    /// A client may poll an [`av::CaptureAudioChannel`] object for its current averagePowerLevel
    /// to get its instantaneous average power level in decibels.
    /// This property is not key-value observable.
    #[objc::msg_send(averagePowerLevel)]
    pub fn average_power_level(&self) -> f32;

    /// A measurement of the peak/hold level of the audio flowing through the receiver.
    ///
    /// A client may poll an [`av::CaptureAudioChannel`] object for its current [`peak_hold_level`] to get its most
    /// recent peak hold level in decibels.
    /// This property is not key-value observable.
    #[objc::msg_send(peakHoldLevel)]
    pub fn peak_hold_level(&self) -> f32;

    #[cfg(target_os = "macos")]
    #[objc::msg_send(volume)]
    pub fn volume(&self) -> f32;

    #[cfg(target_os = "macos")]
    #[objc::msg_send(setVolume:)]
    pub fn set_volume(&mut self, val: f32);

    /// A property indicating whether the receiver is currently enabled for data capture.
    #[cfg(target_os = "macos")]
    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    #[cfg(target_os = "macos")]
    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&self, val: bool);
}

#[doc(alias = "AVCaptureSessionErrorKey")]
pub fn err_key() -> &'static ns::String {
    #[link(name = "AVFoundation", kind = "framework")]
    unsafe extern "C" {
        static AVCaptureSessionErrorKey: &'static ns::String;
    }
    unsafe { AVCaptureSessionErrorKey }
}

pub mod notifications {
    use crate::ns;

    /// Posted when an unexpected error occurs while an [`av::CaptureSession`] instance is running.
    #[doc(alias = "AVCaptureSessionRuntimeErrorNotification")]
    #[inline]
    pub fn runtime_error() -> &'static ns::NotificationName {
        unsafe { AVCaptureSessionRuntimeErrorNotification }
    }

    #[doc(alias = "AVCaptureSessionDidStartRunningNotification")]
    #[inline]
    pub fn did_start_running() -> &'static ns::NotificationName {
        unsafe { AVCaptureSessionDidStartRunningNotification }
    }

    #[doc(alias = "AVCaptureSessionDidStopRunningNotification")]
    #[inline]
    pub fn did_stop_running() -> &'static ns::NotificationName {
        unsafe { AVCaptureSessionDidStopRunningNotification }
    }

    #[doc(alias = "AVCaptureSessionWasInterruptedNotification")]
    #[inline]
    pub fn was_interrupted() -> &'static ns::NotificationName {
        unsafe { AVCaptureSessionWasInterruptedNotification }
    }

    #[doc(alias = "AVCaptureSessionInterruptionEndedNotification")]
    #[inline]
    pub fn interruption_ended() -> &'static ns::NotificationName {
        unsafe { AVCaptureSessionInterruptionEndedNotification }
    }

    #[link(name = "AVFoundation", kind = "framework")]
    unsafe extern "C" {
        static AVCaptureSessionRuntimeErrorNotification: &'static ns::NotificationName;
        static AVCaptureSessionDidStartRunningNotification: &'static ns::NotificationName;
        static AVCaptureSessionDidStopRunningNotification: &'static ns::NotificationName;
        static AVCaptureSessionWasInterruptedNotification: &'static ns::NotificationName;
        static AVCaptureSessionInterruptionEndedNotification: &'static ns::NotificationName;
    }
}
