// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use Accessible;
use Adjustment;
use Align;
use Buildable;
use LayoutManager;
use Orientable;
use Orientation;
use Overflow;
use ScrollType;
use Widget;

glib_wrapper! {
    pub struct Range(Object<gtk_sys::GtkRange, gtk_sys::GtkRangeClass, RangeClass>) @extends Widget, @implements Accessible, Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_range_get_type(),
    }
}

#[derive(Clone, Default)]
pub struct RangeBuilder {
    adjustment: Option<Adjustment>,
    fill_level: Option<f64>,
    inverted: Option<bool>,
    restrict_to_fill_level: Option<bool>,
    round_digits: Option<i32>,
    show_fill_level: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    //accessible-role: /*Unknown type*/,
    orientation: Option<Orientation>,
}

impl RangeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Range {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref adjustment) = self.adjustment {
            properties.push(("adjustment", adjustment));
        }
        if let Some(ref fill_level) = self.fill_level {
            properties.push(("fill-level", fill_level));
        }
        if let Some(ref inverted) = self.inverted {
            properties.push(("inverted", inverted));
        }
        if let Some(ref restrict_to_fill_level) = self.restrict_to_fill_level {
            properties.push(("restrict-to-fill-level", restrict_to_fill_level));
        }
        if let Some(ref round_digits) = self.round_digits {
            properties.push(("round-digits", round_digits));
        }
        if let Some(ref show_fill_level) = self.show_fill_level {
            properties.push(("show-fill-level", show_fill_level));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        let ret = glib::Object::new(Range::static_type(), &properties)
            .expect("object new")
            .downcast::<Range>()
            .expect("downcast");
        ret
    }

    pub fn adjustment<P: IsA<Adjustment>>(mut self, adjustment: &P) -> Self {
        self.adjustment = Some(adjustment.clone().upcast());
        self
    }

    pub fn fill_level(mut self, fill_level: f64) -> Self {
        self.fill_level = Some(fill_level);
        self
    }

    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = Some(inverted);
        self
    }

    pub fn restrict_to_fill_level(mut self, restrict_to_fill_level: bool) -> Self {
        self.restrict_to_fill_level = Some(restrict_to_fill_level);
        self
    }

    pub fn round_digits(mut self, round_digits: i32) -> Self {
        self.round_digits = Some(round_digits);
        self
    }

    pub fn show_fill_level(mut self, show_fill_level: bool) -> Self {
        self.show_fill_level = Some(show_fill_level);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub const NONE_RANGE: Option<&Range> = None;

pub trait RangeExt: 'static {
    fn get_adjustment(&self) -> Adjustment;

    fn get_fill_level(&self) -> f64;

    fn get_flippable(&self) -> bool;

    fn get_inverted(&self) -> bool;

    fn get_range_rect(&self) -> gdk::Rectangle;

    fn get_restrict_to_fill_level(&self) -> bool;

    fn get_round_digits(&self) -> i32;

    fn get_show_fill_level(&self) -> bool;

    fn get_slider_range(&self) -> (i32, i32);

    fn get_slider_size_fixed(&self) -> bool;

    fn get_value(&self) -> f64;

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P);

    fn set_fill_level(&self, fill_level: f64);

    fn set_flippable(&self, flippable: bool);

    fn set_increments(&self, step: f64, page: f64);

    fn set_inverted(&self, setting: bool);

    fn set_range(&self, min: f64, max: f64);

    fn set_restrict_to_fill_level(&self, restrict_to_fill_level: bool);

    fn set_round_digits(&self, round_digits: i32);

    fn set_show_fill_level(&self, show_fill_level: bool);

    fn set_slider_size_fixed(&self, size_fixed: bool);

    fn set_value(&self, value: f64);

    fn connect_adjust_bounds<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_change_value<F: Fn(&Self, ScrollType, f64) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_move_slider<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_slider(&self, step: ScrollType);

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fill_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_restrict_to_fill_level_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_round_digits_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_show_fill_level_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Range>> RangeExt for O {
    fn get_adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(gtk_sys::gtk_range_get_adjustment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_fill_level(&self) -> f64 {
        unsafe { gtk_sys::gtk_range_get_fill_level(self.as_ref().to_glib_none().0) }
    }

    fn get_flippable(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_range_get_flippable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_range_get_inverted(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_range_rect(&self) -> gdk::Rectangle {
        unsafe {
            let mut range_rect = gdk::Rectangle::uninitialized();
            gtk_sys::gtk_range_get_range_rect(
                self.as_ref().to_glib_none().0,
                range_rect.to_glib_none_mut().0,
            );
            range_rect
        }
    }

    fn get_restrict_to_fill_level(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_range_get_restrict_to_fill_level(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_round_digits(&self) -> i32 {
        unsafe { gtk_sys::gtk_range_get_round_digits(self.as_ref().to_glib_none().0) }
    }

    fn get_show_fill_level(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_range_get_show_fill_level(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_slider_range(&self) -> (i32, i32) {
        unsafe {
            let mut slider_start = mem::MaybeUninit::uninit();
            let mut slider_end = mem::MaybeUninit::uninit();
            gtk_sys::gtk_range_get_slider_range(
                self.as_ref().to_glib_none().0,
                slider_start.as_mut_ptr(),
                slider_end.as_mut_ptr(),
            );
            let slider_start = slider_start.assume_init();
            let slider_end = slider_end.assume_init();
            (slider_start, slider_end)
        }
    }

    fn get_slider_size_fixed(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_range_get_slider_size_fixed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_value(&self) -> f64 {
        unsafe { gtk_sys::gtk_range_get_value(self.as_ref().to_glib_none().0) }
    }

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P) {
        unsafe {
            gtk_sys::gtk_range_set_adjustment(
                self.as_ref().to_glib_none().0,
                adjustment.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_fill_level(&self, fill_level: f64) {
        unsafe {
            gtk_sys::gtk_range_set_fill_level(self.as_ref().to_glib_none().0, fill_level);
        }
    }

    fn set_flippable(&self, flippable: bool) {
        unsafe {
            gtk_sys::gtk_range_set_flippable(self.as_ref().to_glib_none().0, flippable.to_glib());
        }
    }

    fn set_increments(&self, step: f64, page: f64) {
        unsafe {
            gtk_sys::gtk_range_set_increments(self.as_ref().to_glib_none().0, step, page);
        }
    }

    fn set_inverted(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_range_set_inverted(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_range(&self, min: f64, max: f64) {
        unsafe {
            gtk_sys::gtk_range_set_range(self.as_ref().to_glib_none().0, min, max);
        }
    }

    fn set_restrict_to_fill_level(&self, restrict_to_fill_level: bool) {
        unsafe {
            gtk_sys::gtk_range_set_restrict_to_fill_level(
                self.as_ref().to_glib_none().0,
                restrict_to_fill_level.to_glib(),
            );
        }
    }

    fn set_round_digits(&self, round_digits: i32) {
        unsafe {
            gtk_sys::gtk_range_set_round_digits(self.as_ref().to_glib_none().0, round_digits);
        }
    }

    fn set_show_fill_level(&self, show_fill_level: bool) {
        unsafe {
            gtk_sys::gtk_range_set_show_fill_level(
                self.as_ref().to_glib_none().0,
                show_fill_level.to_glib(),
            );
        }
    }

    fn set_slider_size_fixed(&self, size_fixed: bool) {
        unsafe {
            gtk_sys::gtk_range_set_slider_size_fixed(
                self.as_ref().to_glib_none().0,
                size_fixed.to_glib(),
            );
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            gtk_sys::gtk_range_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn connect_adjust_bounds<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn adjust_bounds_trampoline<P, F: Fn(&P, f64) + 'static>(
            this: *mut gtk_sys::GtkRange,
            value: libc::c_double,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Range>,
        {
            let f: &F = &*(f as *const F);
            f(&Range::from_glib_borrow(this).unsafe_cast_ref(), value)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"adjust-bounds\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    adjust_bounds_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_change_value<F: Fn(&Self, ScrollType, f64) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn change_value_trampoline<
            P,
            F: Fn(&P, ScrollType, f64) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut gtk_sys::GtkRange,
            scroll: gtk_sys::GtkScrollType,
            value: libc::c_double,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<Range>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Range::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(scroll),
                value,
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"change-value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    change_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_move_slider<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn move_slider_trampoline<P, F: Fn(&P, ScrollType) + 'static>(
            this: *mut gtk_sys::GtkRange,
            step: gtk_sys::GtkScrollType,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Range>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Range::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(step),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-slider\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_slider_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_slider(&self, step: ScrollType) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("move-slider", &[&step])
                .unwrap()
        };
    }

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn value_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRange,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Range>,
        {
            let f: &F = &*(f as *const F);
            f(&Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"value-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    value_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_adjustment_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRange,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Range>,
        {
            let f: &F = &*(f as *const F);
            f(&Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_adjustment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_fill_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fill_level_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRange,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Range>,
        {
            let f: &F = &*(f as *const F);
            f(&Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::fill-level\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fill_level_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRange,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Range>,
        {
            let f: &F = &*(f as *const F);
            f(&Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_restrict_to_fill_level_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_restrict_to_fill_level_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRange,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Range>,
        {
            let f: &F = &*(f as *const F);
            f(&Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::restrict-to-fill-level\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_restrict_to_fill_level_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_round_digits_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_round_digits_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRange,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Range>,
        {
            let f: &F = &*(f as *const F);
            f(&Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::round-digits\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_round_digits_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_fill_level_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_fill_level_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRange,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Range>,
        {
            let f: &F = &*(f as *const F);
            f(&Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-fill-level\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_fill_level_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Range")
    }
}
