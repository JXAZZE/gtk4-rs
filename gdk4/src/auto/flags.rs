// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;
use gobject_sys;

bitflags! {
    pub struct AnchorHints: u32 {
        const FLIP_X = 1;
        const FLIP_Y = 2;
        const SLIDE_X = 4;
        const SLIDE_Y = 8;
        const RESIZE_X = 16;
        const RESIZE_Y = 32;
        const FLIP = 3;
        const SLIDE = 12;
        const RESIZE = 48;
    }
}

#[doc(hidden)]
impl ToGlib for AnchorHints {
    type GlibType = gdk_sys::GdkAnchorHints;

    fn to_glib(&self) -> gdk_sys::GdkAnchorHints {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gdk_sys::GdkAnchorHints> for AnchorHints {
    fn from_glib(value: gdk_sys::GdkAnchorHints) -> AnchorHints {
        skip_assert_initialized!();
        AnchorHints::from_bits_truncate(value)
    }
}

impl StaticType for AnchorHints {
    fn static_type() -> Type {
        unsafe { from_glib(gdk_sys::gdk_anchor_hints_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AnchorHints {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AnchorHints {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for AnchorHints {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct AxisFlags: u32 {
        const X = 2;
        const Y = 4;
        const DELTA_X = 8;
        const DELTA_Y = 16;
        const PRESSURE = 32;
        const XTILT = 64;
        const YTILT = 128;
        const WHEEL = 256;
        const DISTANCE = 512;
        const ROTATION = 1024;
        const SLIDER = 2048;
    }
}

#[doc(hidden)]
impl ToGlib for AxisFlags {
    type GlibType = gdk_sys::GdkAxisFlags;

    fn to_glib(&self) -> gdk_sys::GdkAxisFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gdk_sys::GdkAxisFlags> for AxisFlags {
    fn from_glib(value: gdk_sys::GdkAxisFlags) -> AxisFlags {
        skip_assert_initialized!();
        AxisFlags::from_bits_truncate(value)
    }
}

impl StaticType for AxisFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gdk_sys::gdk_axis_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AxisFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AxisFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for AxisFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct DragAction: u32 {
        const COPY = 1;
        const MOVE = 2;
        const LINK = 4;
        const ASK = 8;
    }
}

#[doc(hidden)]
impl ToGlib for DragAction {
    type GlibType = gdk_sys::GdkDragAction;

    fn to_glib(&self) -> gdk_sys::GdkDragAction {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gdk_sys::GdkDragAction> for DragAction {
    fn from_glib(value: gdk_sys::GdkDragAction) -> DragAction {
        skip_assert_initialized!();
        DragAction::from_bits_truncate(value)
    }
}

impl StaticType for DragAction {
    fn static_type() -> Type {
        unsafe { from_glib(gdk_sys::gdk_drag_action_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DragAction {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DragAction {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for DragAction {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct FrameClockPhase: u32 {
        const NONE = 0;
        const FLUSH_EVENTS = 1;
        const BEFORE_PAINT = 2;
        const UPDATE = 4;
        const LAYOUT = 8;
        const PAINT = 16;
        const RESUME_EVENTS = 32;
        const AFTER_PAINT = 64;
    }
}

#[doc(hidden)]
impl ToGlib for FrameClockPhase {
    type GlibType = gdk_sys::GdkFrameClockPhase;

    fn to_glib(&self) -> gdk_sys::GdkFrameClockPhase {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gdk_sys::GdkFrameClockPhase> for FrameClockPhase {
    fn from_glib(value: gdk_sys::GdkFrameClockPhase) -> FrameClockPhase {
        skip_assert_initialized!();
        FrameClockPhase::from_bits_truncate(value)
    }
}

impl StaticType for FrameClockPhase {
    fn static_type() -> Type {
        unsafe { from_glib(gdk_sys::gdk_frame_clock_phase_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FrameClockPhase {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FrameClockPhase {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for FrameClockPhase {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ModifierType: u32 {
        const SHIFT_MASK = 1;
        const LOCK_MASK = 2;
        const CONTROL_MASK = 4;
        const ALT_MASK = 8;
        const BUTTON1_MASK = 256;
        const BUTTON2_MASK = 512;
        const BUTTON3_MASK = 1024;
        const BUTTON4_MASK = 2048;
        const BUTTON5_MASK = 4096;
        const SUPER_MASK = 67108864;
        const HYPER_MASK = 134217728;
        const META_MASK = 268435456;
    }
}

#[doc(hidden)]
impl ToGlib for ModifierType {
    type GlibType = gdk_sys::GdkModifierType;

    fn to_glib(&self) -> gdk_sys::GdkModifierType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gdk_sys::GdkModifierType> for ModifierType {
    fn from_glib(value: gdk_sys::GdkModifierType) -> ModifierType {
        skip_assert_initialized!();
        ModifierType::from_bits_truncate(value)
    }
}

impl StaticType for ModifierType {
    fn static_type() -> Type {
        unsafe { from_glib(gdk_sys::gdk_modifier_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ModifierType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ModifierType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ModifierType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct PaintableFlags: u32 {
        const SIZE = 1;
        const CONTENTS = 2;
    }
}

#[doc(hidden)]
impl ToGlib for PaintableFlags {
    type GlibType = gdk_sys::GdkPaintableFlags;

    fn to_glib(&self) -> gdk_sys::GdkPaintableFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gdk_sys::GdkPaintableFlags> for PaintableFlags {
    fn from_glib(value: gdk_sys::GdkPaintableFlags) -> PaintableFlags {
        skip_assert_initialized!();
        PaintableFlags::from_bits_truncate(value)
    }
}

impl StaticType for PaintableFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gdk_sys::gdk_paintable_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PaintableFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PaintableFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for PaintableFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct SeatCapabilities: u32 {
        const NONE = 0;
        const POINTER = 1;
        const TOUCH = 2;
        const TABLET_STYLUS = 4;
        const KEYBOARD = 8;
        const TABLET_PAD = 16;
        const ALL_POINTING = 7;
        const ALL = 15;
    }
}

#[doc(hidden)]
impl ToGlib for SeatCapabilities {
    type GlibType = gdk_sys::GdkSeatCapabilities;

    fn to_glib(&self) -> gdk_sys::GdkSeatCapabilities {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gdk_sys::GdkSeatCapabilities> for SeatCapabilities {
    fn from_glib(value: gdk_sys::GdkSeatCapabilities) -> SeatCapabilities {
        skip_assert_initialized!();
        SeatCapabilities::from_bits_truncate(value)
    }
}

impl StaticType for SeatCapabilities {
    fn static_type() -> Type {
        unsafe { from_glib(gdk_sys::gdk_seat_capabilities_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SeatCapabilities {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SeatCapabilities {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for SeatCapabilities {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ToplevelState: u32 {
        const WITHDRAWN = 1;
        const MINIMIZED = 2;
        const MAXIMIZED = 4;
        const STICKY = 8;
        const FULLSCREEN = 16;
        const ABOVE = 32;
        const BELOW = 64;
        const FOCUSED = 128;
        const TILED = 256;
        const TOP_TILED = 512;
        const TOP_RESIZABLE = 1024;
        const RIGHT_TILED = 2048;
        const RIGHT_RESIZABLE = 4096;
        const BOTTOM_TILED = 8192;
        const BOTTOM_RESIZABLE = 16384;
        const LEFT_TILED = 32768;
        const LEFT_RESIZABLE = 65536;
    }
}

#[doc(hidden)]
impl ToGlib for ToplevelState {
    type GlibType = gdk_sys::GdkToplevelState;

    fn to_glib(&self) -> gdk_sys::GdkToplevelState {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gdk_sys::GdkToplevelState> for ToplevelState {
    fn from_glib(value: gdk_sys::GdkToplevelState) -> ToplevelState {
        skip_assert_initialized!();
        ToplevelState::from_bits_truncate(value)
    }
}

impl StaticType for ToplevelState {
    fn static_type() -> Type {
        unsafe { from_glib(gdk_sys::gdk_toplevel_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ToplevelState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ToplevelState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ToplevelState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
