// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::error::ErrorDomain;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::Quark;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskBlendMode")]
pub enum BlendMode {
    #[doc(alias = "GSK_BLEND_MODE_DEFAULT")]
    Default,
    #[doc(alias = "GSK_BLEND_MODE_MULTIPLY")]
    Multiply,
    #[doc(alias = "GSK_BLEND_MODE_SCREEN")]
    Screen,
    #[doc(alias = "GSK_BLEND_MODE_OVERLAY")]
    Overlay,
    #[doc(alias = "GSK_BLEND_MODE_DARKEN")]
    Darken,
    #[doc(alias = "GSK_BLEND_MODE_LIGHTEN")]
    Lighten,
    #[doc(alias = "GSK_BLEND_MODE_COLOR_DODGE")]
    ColorDodge,
    #[doc(alias = "GSK_BLEND_MODE_COLOR_BURN")]
    ColorBurn,
    #[doc(alias = "GSK_BLEND_MODE_HARD_LIGHT")]
    HardLight,
    #[doc(alias = "GSK_BLEND_MODE_SOFT_LIGHT")]
    SoftLight,
    #[doc(alias = "GSK_BLEND_MODE_DIFFERENCE")]
    Difference,
    #[doc(alias = "GSK_BLEND_MODE_EXCLUSION")]
    Exclusion,
    #[doc(alias = "GSK_BLEND_MODE_COLOR")]
    Color,
    #[doc(alias = "GSK_BLEND_MODE_HUE")]
    Hue,
    #[doc(alias = "GSK_BLEND_MODE_SATURATION")]
    Saturation,
    #[doc(alias = "GSK_BLEND_MODE_LUMINOSITY")]
    Luminosity,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BlendMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Default => "Default",
                Self::Multiply => "Multiply",
                Self::Screen => "Screen",
                Self::Overlay => "Overlay",
                Self::Darken => "Darken",
                Self::Lighten => "Lighten",
                Self::ColorDodge => "ColorDodge",
                Self::ColorBurn => "ColorBurn",
                Self::HardLight => "HardLight",
                Self::SoftLight => "SoftLight",
                Self::Difference => "Difference",
                Self::Exclusion => "Exclusion",
                Self::Color => "Color",
                Self::Hue => "Hue",
                Self::Saturation => "Saturation",
                Self::Luminosity => "Luminosity",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for BlendMode {
    type GlibType = ffi::GskBlendMode;

    fn into_glib(self) -> ffi::GskBlendMode {
        match self {
            Self::Default => ffi::GSK_BLEND_MODE_DEFAULT,
            Self::Multiply => ffi::GSK_BLEND_MODE_MULTIPLY,
            Self::Screen => ffi::GSK_BLEND_MODE_SCREEN,
            Self::Overlay => ffi::GSK_BLEND_MODE_OVERLAY,
            Self::Darken => ffi::GSK_BLEND_MODE_DARKEN,
            Self::Lighten => ffi::GSK_BLEND_MODE_LIGHTEN,
            Self::ColorDodge => ffi::GSK_BLEND_MODE_COLOR_DODGE,
            Self::ColorBurn => ffi::GSK_BLEND_MODE_COLOR_BURN,
            Self::HardLight => ffi::GSK_BLEND_MODE_HARD_LIGHT,
            Self::SoftLight => ffi::GSK_BLEND_MODE_SOFT_LIGHT,
            Self::Difference => ffi::GSK_BLEND_MODE_DIFFERENCE,
            Self::Exclusion => ffi::GSK_BLEND_MODE_EXCLUSION,
            Self::Color => ffi::GSK_BLEND_MODE_COLOR,
            Self::Hue => ffi::GSK_BLEND_MODE_HUE,
            Self::Saturation => ffi::GSK_BLEND_MODE_SATURATION,
            Self::Luminosity => ffi::GSK_BLEND_MODE_LUMINOSITY,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskBlendMode> for BlendMode {
    unsafe fn from_glib(value: ffi::GskBlendMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Self::Default,
            1 => Self::Multiply,
            2 => Self::Screen,
            3 => Self::Overlay,
            4 => Self::Darken,
            5 => Self::Lighten,
            6 => Self::ColorDodge,
            7 => Self::ColorBurn,
            8 => Self::HardLight,
            9 => Self::SoftLight,
            10 => Self::Difference,
            11 => Self::Exclusion,
            12 => Self::Color,
            13 => Self::Hue,
            14 => Self::Saturation,
            15 => Self::Luminosity,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for BlendMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_blend_mode_get_type()) }
    }
}

impl glib::value::ValueType for BlendMode {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for BlendMode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for BlendMode {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskCorner")]
pub enum Corner {
    #[doc(alias = "GSK_CORNER_TOP_LEFT")]
    TopLeft,
    #[doc(alias = "GSK_CORNER_TOP_RIGHT")]
    TopRight,
    #[doc(alias = "GSK_CORNER_BOTTOM_RIGHT")]
    BottomRight,
    #[doc(alias = "GSK_CORNER_BOTTOM_LEFT")]
    BottomLeft,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Corner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::TopLeft => "TopLeft",
                Self::TopRight => "TopRight",
                Self::BottomRight => "BottomRight",
                Self::BottomLeft => "BottomLeft",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for Corner {
    type GlibType = ffi::GskCorner;

    fn into_glib(self) -> ffi::GskCorner {
        match self {
            Self::TopLeft => ffi::GSK_CORNER_TOP_LEFT,
            Self::TopRight => ffi::GSK_CORNER_TOP_RIGHT,
            Self::BottomRight => ffi::GSK_CORNER_BOTTOM_RIGHT,
            Self::BottomLeft => ffi::GSK_CORNER_BOTTOM_LEFT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskCorner> for Corner {
    unsafe fn from_glib(value: ffi::GskCorner) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Self::TopLeft,
            1 => Self::TopRight,
            2 => Self::BottomRight,
            3 => Self::BottomLeft,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for Corner {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_corner_get_type()) }
    }
}

impl glib::value::ValueType for Corner {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for Corner {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Corner {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskGLUniformType")]
pub enum GLUniformType {
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_NONE")]
    None,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_FLOAT")]
    Float,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_INT")]
    Int,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_UINT")]
    Uint,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_BOOL")]
    Bool,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_VEC2")]
    Vec2,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_VEC3")]
    Vec3,
    #[doc(alias = "GSK_GL_UNIFORM_TYPE_VEC4")]
    Vec4,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for GLUniformType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::None => "None",
                Self::Float => "Float",
                Self::Int => "Int",
                Self::Uint => "Uint",
                Self::Bool => "Bool",
                Self::Vec2 => "Vec2",
                Self::Vec3 => "Vec3",
                Self::Vec4 => "Vec4",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for GLUniformType {
    type GlibType = ffi::GskGLUniformType;

    fn into_glib(self) -> ffi::GskGLUniformType {
        match self {
            Self::None => ffi::GSK_GL_UNIFORM_TYPE_NONE,
            Self::Float => ffi::GSK_GL_UNIFORM_TYPE_FLOAT,
            Self::Int => ffi::GSK_GL_UNIFORM_TYPE_INT,
            Self::Uint => ffi::GSK_GL_UNIFORM_TYPE_UINT,
            Self::Bool => ffi::GSK_GL_UNIFORM_TYPE_BOOL,
            Self::Vec2 => ffi::GSK_GL_UNIFORM_TYPE_VEC2,
            Self::Vec3 => ffi::GSK_GL_UNIFORM_TYPE_VEC3,
            Self::Vec4 => ffi::GSK_GL_UNIFORM_TYPE_VEC4,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskGLUniformType> for GLUniformType {
    unsafe fn from_glib(value: ffi::GskGLUniformType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Self::None,
            1 => Self::Float,
            2 => Self::Int,
            3 => Self::Uint,
            4 => Self::Bool,
            5 => Self::Vec2,
            6 => Self::Vec3,
            7 => Self::Vec4,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for GLUniformType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_gl_uniform_type_get_type()) }
    }
}

impl glib::value::ValueType for GLUniformType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for GLUniformType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for GLUniformType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskRenderNodeType")]
pub enum RenderNodeType {
    #[doc(alias = "GSK_NOT_A_RENDER_NODE")]
    NotARenderNode,
    #[doc(alias = "GSK_CONTAINER_NODE")]
    ContainerNode,
    #[doc(alias = "GSK_CAIRO_NODE")]
    CairoNode,
    #[doc(alias = "GSK_COLOR_NODE")]
    ColorNode,
    #[doc(alias = "GSK_LINEAR_GRADIENT_NODE")]
    LinearGradientNode,
    #[doc(alias = "GSK_REPEATING_LINEAR_GRADIENT_NODE")]
    RepeatingLinearGradientNode,
    #[doc(alias = "GSK_RADIAL_GRADIENT_NODE")]
    RadialGradientNode,
    #[doc(alias = "GSK_REPEATING_RADIAL_GRADIENT_NODE")]
    RepeatingRadialGradientNode,
    #[doc(alias = "GSK_CONIC_GRADIENT_NODE")]
    ConicGradientNode,
    #[doc(alias = "GSK_BORDER_NODE")]
    BorderNode,
    #[doc(alias = "GSK_TEXTURE_NODE")]
    TextureNode,
    #[doc(alias = "GSK_INSET_SHADOW_NODE")]
    InsetShadowNode,
    #[doc(alias = "GSK_OUTSET_SHADOW_NODE")]
    OutsetShadowNode,
    #[doc(alias = "GSK_TRANSFORM_NODE")]
    TransformNode,
    #[doc(alias = "GSK_OPACITY_NODE")]
    OpacityNode,
    #[doc(alias = "GSK_COLOR_MATRIX_NODE")]
    ColorMatrixNode,
    #[doc(alias = "GSK_REPEAT_NODE")]
    RepeatNode,
    #[doc(alias = "GSK_CLIP_NODE")]
    ClipNode,
    #[doc(alias = "GSK_ROUNDED_CLIP_NODE")]
    RoundedClipNode,
    #[doc(alias = "GSK_SHADOW_NODE")]
    ShadowNode,
    #[doc(alias = "GSK_BLEND_NODE")]
    BlendNode,
    #[doc(alias = "GSK_CROSS_FADE_NODE")]
    CrossFadeNode,
    #[doc(alias = "GSK_TEXT_NODE")]
    TextNode,
    #[doc(alias = "GSK_BLUR_NODE")]
    BlurNode,
    #[doc(alias = "GSK_DEBUG_NODE")]
    DebugNode,
    #[doc(alias = "GSK_GL_SHADER_NODE")]
    GlShaderNode,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for RenderNodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::NotARenderNode => "NotARenderNode",
                Self::ContainerNode => "ContainerNode",
                Self::CairoNode => "CairoNode",
                Self::ColorNode => "ColorNode",
                Self::LinearGradientNode => "LinearGradientNode",
                Self::RepeatingLinearGradientNode => "RepeatingLinearGradientNode",
                Self::RadialGradientNode => "RadialGradientNode",
                Self::RepeatingRadialGradientNode => "RepeatingRadialGradientNode",
                Self::ConicGradientNode => "ConicGradientNode",
                Self::BorderNode => "BorderNode",
                Self::TextureNode => "TextureNode",
                Self::InsetShadowNode => "InsetShadowNode",
                Self::OutsetShadowNode => "OutsetShadowNode",
                Self::TransformNode => "TransformNode",
                Self::OpacityNode => "OpacityNode",
                Self::ColorMatrixNode => "ColorMatrixNode",
                Self::RepeatNode => "RepeatNode",
                Self::ClipNode => "ClipNode",
                Self::RoundedClipNode => "RoundedClipNode",
                Self::ShadowNode => "ShadowNode",
                Self::BlendNode => "BlendNode",
                Self::CrossFadeNode => "CrossFadeNode",
                Self::TextNode => "TextNode",
                Self::BlurNode => "BlurNode",
                Self::DebugNode => "DebugNode",
                Self::GlShaderNode => "GlShaderNode",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for RenderNodeType {
    type GlibType = ffi::GskRenderNodeType;

    fn into_glib(self) -> ffi::GskRenderNodeType {
        match self {
            Self::NotARenderNode => ffi::GSK_NOT_A_RENDER_NODE,
            Self::ContainerNode => ffi::GSK_CONTAINER_NODE,
            Self::CairoNode => ffi::GSK_CAIRO_NODE,
            Self::ColorNode => ffi::GSK_COLOR_NODE,
            Self::LinearGradientNode => ffi::GSK_LINEAR_GRADIENT_NODE,
            Self::RepeatingLinearGradientNode => ffi::GSK_REPEATING_LINEAR_GRADIENT_NODE,
            Self::RadialGradientNode => ffi::GSK_RADIAL_GRADIENT_NODE,
            Self::RepeatingRadialGradientNode => ffi::GSK_REPEATING_RADIAL_GRADIENT_NODE,
            Self::ConicGradientNode => ffi::GSK_CONIC_GRADIENT_NODE,
            Self::BorderNode => ffi::GSK_BORDER_NODE,
            Self::TextureNode => ffi::GSK_TEXTURE_NODE,
            Self::InsetShadowNode => ffi::GSK_INSET_SHADOW_NODE,
            Self::OutsetShadowNode => ffi::GSK_OUTSET_SHADOW_NODE,
            Self::TransformNode => ffi::GSK_TRANSFORM_NODE,
            Self::OpacityNode => ffi::GSK_OPACITY_NODE,
            Self::ColorMatrixNode => ffi::GSK_COLOR_MATRIX_NODE,
            Self::RepeatNode => ffi::GSK_REPEAT_NODE,
            Self::ClipNode => ffi::GSK_CLIP_NODE,
            Self::RoundedClipNode => ffi::GSK_ROUNDED_CLIP_NODE,
            Self::ShadowNode => ffi::GSK_SHADOW_NODE,
            Self::BlendNode => ffi::GSK_BLEND_NODE,
            Self::CrossFadeNode => ffi::GSK_CROSS_FADE_NODE,
            Self::TextNode => ffi::GSK_TEXT_NODE,
            Self::BlurNode => ffi::GSK_BLUR_NODE,
            Self::DebugNode => ffi::GSK_DEBUG_NODE,
            Self::GlShaderNode => ffi::GSK_GL_SHADER_NODE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskRenderNodeType> for RenderNodeType {
    unsafe fn from_glib(value: ffi::GskRenderNodeType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Self::NotARenderNode,
            1 => Self::ContainerNode,
            2 => Self::CairoNode,
            3 => Self::ColorNode,
            4 => Self::LinearGradientNode,
            5 => Self::RepeatingLinearGradientNode,
            6 => Self::RadialGradientNode,
            7 => Self::RepeatingRadialGradientNode,
            8 => Self::ConicGradientNode,
            9 => Self::BorderNode,
            10 => Self::TextureNode,
            11 => Self::InsetShadowNode,
            12 => Self::OutsetShadowNode,
            13 => Self::TransformNode,
            14 => Self::OpacityNode,
            15 => Self::ColorMatrixNode,
            16 => Self::RepeatNode,
            17 => Self::ClipNode,
            18 => Self::RoundedClipNode,
            19 => Self::ShadowNode,
            20 => Self::BlendNode,
            21 => Self::CrossFadeNode,
            22 => Self::TextNode,
            23 => Self::BlurNode,
            24 => Self::DebugNode,
            25 => Self::GlShaderNode,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for RenderNodeType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_render_node_type_get_type()) }
    }
}

impl glib::value::ValueType for RenderNodeType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RenderNodeType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for RenderNodeType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskScalingFilter")]
pub enum ScalingFilter {
    #[doc(alias = "GSK_SCALING_FILTER_LINEAR")]
    Linear,
    #[doc(alias = "GSK_SCALING_FILTER_NEAREST")]
    Nearest,
    #[doc(alias = "GSK_SCALING_FILTER_TRILINEAR")]
    Trilinear,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ScalingFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Linear => "Linear",
                Self::Nearest => "Nearest",
                Self::Trilinear => "Trilinear",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for ScalingFilter {
    type GlibType = ffi::GskScalingFilter;

    fn into_glib(self) -> ffi::GskScalingFilter {
        match self {
            Self::Linear => ffi::GSK_SCALING_FILTER_LINEAR,
            Self::Nearest => ffi::GSK_SCALING_FILTER_NEAREST,
            Self::Trilinear => ffi::GSK_SCALING_FILTER_TRILINEAR,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskScalingFilter> for ScalingFilter {
    unsafe fn from_glib(value: ffi::GskScalingFilter) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Self::Linear,
            1 => Self::Nearest,
            2 => Self::Trilinear,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for ScalingFilter {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_scaling_filter_get_type()) }
    }
}

impl glib::value::ValueType for ScalingFilter {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ScalingFilter {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ScalingFilter {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskSerializationError")]
pub enum SerializationError {
    #[doc(alias = "GSK_SERIALIZATION_UNSUPPORTED_FORMAT")]
    UnsupportedFormat,
    #[doc(alias = "GSK_SERIALIZATION_UNSUPPORTED_VERSION")]
    UnsupportedVersion,
    #[doc(alias = "GSK_SERIALIZATION_INVALID_DATA")]
    InvalidData,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::UnsupportedFormat => "UnsupportedFormat",
                Self::UnsupportedVersion => "UnsupportedVersion",
                Self::InvalidData => "InvalidData",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for SerializationError {
    type GlibType = ffi::GskSerializationError;

    fn into_glib(self) -> ffi::GskSerializationError {
        match self {
            Self::UnsupportedFormat => ffi::GSK_SERIALIZATION_UNSUPPORTED_FORMAT,
            Self::UnsupportedVersion => ffi::GSK_SERIALIZATION_UNSUPPORTED_VERSION,
            Self::InvalidData => ffi::GSK_SERIALIZATION_INVALID_DATA,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskSerializationError> for SerializationError {
    unsafe fn from_glib(value: ffi::GskSerializationError) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Self::UnsupportedFormat,
            1 => Self::UnsupportedVersion,
            2 => Self::InvalidData,
            value => Self::__Unknown(value),
        }
    }
}

impl ErrorDomain for SerializationError {
    fn domain() -> Quark {
        skip_assert_initialized!();

        unsafe { from_glib(ffi::gsk_serialization_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            0 => Some(Self::UnsupportedFormat),
            1 => Some(Self::UnsupportedVersion),
            2 => Some(Self::InvalidData),
            value => Some(Self::__Unknown(value)),
        }
    }
}

impl StaticType for SerializationError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_serialization_error_get_type()) }
    }
}

impl glib::value::ValueType for SerializationError {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for SerializationError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for SerializationError {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GskTransformCategory")]
pub enum TransformCategory {
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_UNKNOWN")]
    Unknown,
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_ANY")]
    Any,
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_3D")]
    _3d,
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_2D")]
    _2d,
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_2D_AFFINE")]
    _2dAffine,
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_2D_TRANSLATE")]
    _2dTranslate,
    #[doc(alias = "GSK_TRANSFORM_CATEGORY_IDENTITY")]
    Identity,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TransformCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Any => "Any",
                Self::_3d => "_3d",
                Self::_2d => "_2d",
                Self::_2dAffine => "_2dAffine",
                Self::_2dTranslate => "_2dTranslate",
                Self::Identity => "Identity",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for TransformCategory {
    type GlibType = ffi::GskTransformCategory;

    fn into_glib(self) -> ffi::GskTransformCategory {
        match self {
            Self::Unknown => ffi::GSK_TRANSFORM_CATEGORY_UNKNOWN,
            Self::Any => ffi::GSK_TRANSFORM_CATEGORY_ANY,
            Self::_3d => ffi::GSK_TRANSFORM_CATEGORY_3D,
            Self::_2d => ffi::GSK_TRANSFORM_CATEGORY_2D,
            Self::_2dAffine => ffi::GSK_TRANSFORM_CATEGORY_2D_AFFINE,
            Self::_2dTranslate => ffi::GSK_TRANSFORM_CATEGORY_2D_TRANSLATE,
            Self::Identity => ffi::GSK_TRANSFORM_CATEGORY_IDENTITY,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GskTransformCategory> for TransformCategory {
    unsafe fn from_glib(value: ffi::GskTransformCategory) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Self::Unknown,
            1 => Self::Any,
            2 => Self::_3d,
            3 => Self::_2d,
            4 => Self::_2dAffine,
            5 => Self::_2dTranslate,
            6 => Self::Identity,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for TransformCategory {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_transform_category_get_type()) }
    }
}

impl glib::value::ValueType for TransformCategory {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for TransformCategory {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for TransformCategory {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
