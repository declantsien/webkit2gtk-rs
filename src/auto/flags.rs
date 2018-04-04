// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_ffi;

#[cfg(any(feature = "v2_10", feature = "dox"))]
bitflags! {
    pub struct EditorTypingAttributes: u32 {
        const NONE = 2;
        const BOLD = 4;
        const ITALIC = 8;
        const UNDERLINE = 16;
        const STRIKETHROUGH = 32;
    }
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for EditorTypingAttributes {
    type GlibType = ffi::WebKitEditorTypingAttributes;

    fn to_glib(&self) -> ffi::WebKitEditorTypingAttributes {
        self.bits()
    }
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::WebKitEditorTypingAttributes> for EditorTypingAttributes {
    fn from_glib(value: ffi::WebKitEditorTypingAttributes) -> EditorTypingAttributes {
        skip_assert_initialized!();
        EditorTypingAttributes::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
impl StaticType for EditorTypingAttributes {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::webkit_editor_typing_attributes_get_type()) }
    }
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
impl<'a> FromValueOptional<'a> for EditorTypingAttributes {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
impl<'a> FromValue<'a> for EditorTypingAttributes {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
impl SetValue for EditorTypingAttributes {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct FindOptions: u32 {
        const NONE = 0;
        const CASE_INSENSITIVE = 1;
        const AT_WORD_STARTS = 2;
        const TREAT_MEDIAL_CAPITAL_AS_WORD_START = 4;
        const BACKWARDS = 8;
        const WRAP_AROUND = 16;
    }
}

#[doc(hidden)]
impl ToGlib for FindOptions {
    type GlibType = ffi::WebKitFindOptions;

    fn to_glib(&self) -> ffi::WebKitFindOptions {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WebKitFindOptions> for FindOptions {
    fn from_glib(value: ffi::WebKitFindOptions) -> FindOptions {
        skip_assert_initialized!();
        FindOptions::from_bits_truncate(value)
    }
}

impl StaticType for FindOptions {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::webkit_find_options_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FindOptions {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FindOptions {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for FindOptions {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct HitTestResultContext: u32 {
        const DOCUMENT = 2;
        const LINK = 4;
        const IMAGE = 8;
        const MEDIA = 16;
        const EDITABLE = 32;
        const SCROLLBAR = 64;
        const SELECTION = 128;
    }
}

#[doc(hidden)]
impl ToGlib for HitTestResultContext {
    type GlibType = ffi::WebKitHitTestResultContext;

    fn to_glib(&self) -> ffi::WebKitHitTestResultContext {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WebKitHitTestResultContext> for HitTestResultContext {
    fn from_glib(value: ffi::WebKitHitTestResultContext) -> HitTestResultContext {
        skip_assert_initialized!();
        HitTestResultContext::from_bits_truncate(value)
    }
}

impl StaticType for HitTestResultContext {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::webkit_hit_test_result_context_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for HitTestResultContext {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for HitTestResultContext {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for HitTestResultContext {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct SnapshotOptions: u32 {
        const NONE = 0;
        const INCLUDE_SELECTION_HIGHLIGHTING = 1;
        const TRANSPARENT_BACKGROUND = 2;
    }
}

#[doc(hidden)]
impl ToGlib for SnapshotOptions {
    type GlibType = ffi::WebKitSnapshotOptions;

    fn to_glib(&self) -> ffi::WebKitSnapshotOptions {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WebKitSnapshotOptions> for SnapshotOptions {
    fn from_glib(value: ffi::WebKitSnapshotOptions) -> SnapshotOptions {
        skip_assert_initialized!();
        SnapshotOptions::from_bits_truncate(value)
    }
}

impl StaticType for SnapshotOptions {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::webkit_snapshot_options_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SnapshotOptions {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SnapshotOptions {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for SnapshotOptions {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
bitflags! {
    pub struct WebsiteDataTypes: u32 {
        const MEMORY_CACHE = 1;
        const DISK_CACHE = 2;
        const OFFLINE_APPLICATION_CACHE = 4;
        const SESSION_STORAGE = 8;
        const LOCAL_STORAGE = 16;
        const WEBSQL_DATABASES = 32;
        const INDEXEDDB_DATABASES = 64;
        const PLUGIN_DATA = 128;
        const COOKIES = 256;
        const ALL = 511;
    }
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for WebsiteDataTypes {
    type GlibType = ffi::WebKitWebsiteDataTypes;

    fn to_glib(&self) -> ffi::WebKitWebsiteDataTypes {
        self.bits()
    }
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::WebKitWebsiteDataTypes> for WebsiteDataTypes {
    fn from_glib(value: ffi::WebKitWebsiteDataTypes) -> WebsiteDataTypes {
        skip_assert_initialized!();
        WebsiteDataTypes::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
impl StaticType for WebsiteDataTypes {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::webkit_website_data_types_get_type()) }
    }
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
impl<'a> FromValueOptional<'a> for WebsiteDataTypes {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
impl<'a> FromValue<'a> for WebsiteDataTypes {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
impl SetValue for WebsiteDataTypes {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

