use crate::font as __with_name0;
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod alibaba {
        #[allow(dead_code)]
        pub mod fontkit {
            #[allow(dead_code, clippy::all)]
            pub mod fontkit_interface {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type FontKey = super::super::super::super::__with_name0::FontKey;
                #[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd)]
                pub struct Name {
                    pub id: u16,
                    pub name: _rt::String,
                    pub language_id: u16,
                }
                impl ::core::fmt::Debug for Name {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Name")
                            .field("id", &self.id)
                            .field("name", &self.name)
                            .field("language-id", &self.language_id)
                            .finish()
                    }
                }
                #[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd)]
                pub struct FontInfo {
                    pub style_names: _rt::Vec<Name>,
                    pub names: _rt::Vec<Name>,
                    pub path: Option<_rt::String>,
                    pub key: FontKey,
                }
                impl ::core::fmt::Debug for FontInfo {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("FontInfo")
                            .field("style-names", &self.style_names)
                            .field("names", &self.names)
                            .field("path", &self.path)
                            .field("key", &self.key)
                            .finish()
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd)]
                pub struct LineMetrics {
                    pub position: i16,
                    pub thickness: i16,
                }
                impl ::core::fmt::Debug for LineMetrics {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("LineMetrics")
                            .field("position", &self.position)
                            .field("thickness", &self.thickness)
                            .finish()
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct TextMetrics {
                    handle: _rt::Resource<TextMetrics>,
                }
                type _TextMetricsRep<T> = Option<T>;
                impl TextMetrics {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `TextMetrics`.
                    pub fn new<T: GuestTextMetrics>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _TextMetricsRep<T> = Some(val);
                        let ptr: *mut _TextMetricsRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestTextMetrics>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestTextMetrics>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestTextMetrics>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _TextMetricsRep<T>);
                    }
                    fn as_ptr<T: GuestTextMetrics>(&self) -> *mut _TextMetricsRep<T> {
                        TextMetrics::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`TextMetrics`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct TextMetricsBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a TextMetrics>,
                }
                impl<'a> TextMetricsBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestTextMetrics>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _TextMetricsRep<T> {
                        TextMetrics::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for TextMetrics {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]alibaba:fontkit/fontkit-interface"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]text-metrics"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct GlyphBitmap {
                    handle: _rt::Resource<GlyphBitmap>,
                }
                type _GlyphBitmapRep<T> = Option<T>;
                impl GlyphBitmap {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `GlyphBitmap`.
                    pub fn new<T: GuestGlyphBitmap>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _GlyphBitmapRep<T> = Some(val);
                        let ptr: *mut _GlyphBitmapRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestGlyphBitmap>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestGlyphBitmap>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestGlyphBitmap>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _GlyphBitmapRep<T>);
                    }
                    fn as_ptr<T: GuestGlyphBitmap>(&self) -> *mut _GlyphBitmapRep<T> {
                        GlyphBitmap::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`GlyphBitmap`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct GlyphBitmapBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a GlyphBitmap>,
                }
                impl<'a> GlyphBitmapBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestGlyphBitmap>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _GlyphBitmapRep<T> {
                        GlyphBitmap::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for GlyphBitmap {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]alibaba:fontkit/fontkit-interface"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]glyph-bitmap"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                /// Instance of a single font
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Font {
                    handle: _rt::Resource<Font>,
                }
                type _FontRep<T> = Option<T>;
                impl Font {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Font`.
                    pub fn new<T: GuestFont>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _FontRep<T> = Some(val);
                        let ptr: *mut _FontRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestFont>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestFont>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestFont>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _FontRep<T>);
                    }
                    fn as_ptr<T: GuestFont>(&self) -> *mut _FontRep<T> {
                        Font::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Font`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct FontBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Font>,
                }
                impl<'a> FontBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestFont>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _FontRep<T> {
                        Font::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Font {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]alibaba:fontkit/fontkit-interface"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]font"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                /// Stores font buffer and provides font-querying APIs
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct FontKit {
                    handle: _rt::Resource<FontKit>,
                }
                type _FontKitRep<T> = Option<T>;
                impl FontKit {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `FontKit`.
                    pub fn new<T: GuestFontKit>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _FontKitRep<T> = Some(val);
                        let ptr: *mut _FontKitRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestFontKit>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestFontKit>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestFontKit>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _FontKitRep<T>);
                    }
                    fn as_ptr<T: GuestFontKit>(&self) -> *mut _FontKitRep<T> {
                        FontKit::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`FontKit`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct FontKitBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a FontKit>,
                }
                impl<'a> FontKitBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestFontKit>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _FontKitRep<T> {
                        FontKit::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for FontKit {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]alibaba:fontkit/fontkit-interface"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]font-kit"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_text_metrics_cabi<T: GuestTextMetrics>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = TextMetrics::new(T::new(_rt::string_lift(bytes0)));
                    (result1).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_duplicate_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::duplicate(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_width_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8, arg1: f32, arg2: f32) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::width(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                        arg1,
                        arg2,
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_height_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8, arg1: f32, arg2: i32, arg3: f32) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::height(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                        arg1,
                        match arg2 {
                            0 => None,
                            1 => {
                                let e = arg3;
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_ascender_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8, arg1: f32) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::ascender(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                        arg1,
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_line_gap_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::line_gap(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_units_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::units(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_slice_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8, arg1: i32, arg2: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::slice(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u32,
                    );
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_value_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::value(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_text_metrics_value<
                    T: GuestTextMetrics,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_is_rtl_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::is_rtl(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    match result0 {
                        true => 1,
                        false => 0,
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_append_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8, arg1: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::append(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                        TextMetrics::from_handle(arg1 as u32),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_count_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::count(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_replace_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::replace(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                        TextMetrics::from_handle(arg1 as u32),
                        _rt::bool_lift(arg2 as u8),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_split_by_width_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8, arg1: f32, arg2: f32, arg3: f32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::split_by_width(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                        arg1,
                        arg2,
                        arg3,
                    );
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_text_metrics_chars_cabi<
                    T: GuestTextMetrics,
                >(arg0: *mut u8) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::chars(
                        TextMetricsBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = result0;
                    let len2 = vec2.len();
                    let layout2 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec2.len() * 4,
                        4,
                    );
                    let result2 = if layout2.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout2).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout2);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec2.into_iter().enumerate() {
                        let base = result2.add(i * 4);
                        {
                            *base.add(0).cast::<i32>() = _rt::as_i32(e);
                        }
                    }
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = result2;
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_text_metrics_chars<
                    T: GuestTextMetrics,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 4, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_glyph_bitmap_width_cabi<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::width(
                        GlyphBitmapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_glyph_bitmap_height_cabi<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::height(
                        GlyphBitmapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_glyph_bitmap_bitmap_cabi<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::bitmap(
                        GlyphBitmapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_glyph_bitmap_bitmap<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_glyph_bitmap_x_min_cabi<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::x_min(
                        GlyphBitmapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_glyph_bitmap_y_max_cabi<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::y_max(
                        GlyphBitmapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_glyph_bitmap_stroke_x_cabi<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::stroke_x(
                        GlyphBitmapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_glyph_bitmap_stroke_y_cabi<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::stroke_y(
                        GlyphBitmapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_glyph_bitmap_stroke_bitmap_cabi<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::stroke_bitmap(
                        GlyphBitmapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let (t2_0, t2_1) = e;
                            let vec3 = (t2_0).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr1.add(8).cast::<usize>() = len3;
                            *ptr1.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                            *ptr1.add(12).cast::<i32>() = _rt::as_i32(t2_1);
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_glyph_bitmap_stroke_bitmap<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_glyph_bitmap_advanced_x_cabi<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::advanced_x(
                        GlyphBitmapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_glyph_bitmap_ascender_cabi<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::ascender(
                        GlyphBitmapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_glyph_bitmap_descender_cabi<
                    T: GuestGlyphBitmap,
                >(arg0: *mut u8) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::descender(
                        GlyphBitmapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_has_glyph_cabi<T: GuestFont>(
                    arg0: *mut u8,
                    arg1: i32,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::has_glyph(
                        FontBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::char_lift(arg1 as u32),
                    );
                    match result0 {
                        true => 1,
                        false => 0,
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_glyph_path_string_cabi<T: GuestFont>(
                    arg0: *mut u8,
                    arg1: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::glyph_path_string(
                        FontBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::char_lift(arg1 as u32),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec2 = (e.into_bytes()).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_font_glyph_path_string<T: GuestFont>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_buffer_cabi<T: GuestFont>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::buffer(
                        FontBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_font_buffer<T: GuestFont>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_path_cabi<T: GuestFont>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::path(FontBorrow::lift(arg0 as u32 as usize).get());
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_font_path<T: GuestFont>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_key_cabi<T: GuestFont>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::key(FontBorrow::lift(arg0 as u32 as usize).get());
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let super::super::super::super::__with_name0::FontKey {
                        weight: weight2,
                        italic: italic2,
                        stretch: stretch2,
                        family: family2,
                        variations: variations2,
                    } = result0;
                    match weight2 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr1.add(2).cast::<u16>() = (_rt::as_i32(e)) as u16;
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    match italic2 {
                        Some(e) => {
                            *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                            *ptr1.add(5).cast::<u8>() = (match e {
                                true => 1,
                                false => 0,
                            }) as u8;
                        }
                        None => {
                            *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    match stretch2 {
                        Some(e) => {
                            *ptr1.add(6).cast::<u8>() = (1i32) as u8;
                            *ptr1.add(8).cast::<u16>() = (_rt::as_i32(e)) as u16;
                        }
                        None => {
                            *ptr1.add(6).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    let vec3 = (family2.into_bytes()).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr1.add(16).cast::<usize>() = len3;
                    *ptr1.add(12).cast::<*mut u8>() = ptr3.cast_mut();
                    let vec6 = variations2;
                    let len6 = vec6.len();
                    let layout6 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec6.len() * 12,
                        4,
                    );
                    let result6 = if layout6.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout6).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout6);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec6.into_iter().enumerate() {
                        let base = result6.add(i * 12);
                        {
                            let (t4_0, t4_1) = e;
                            let vec5 = (t4_0.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *base.add(4).cast::<usize>() = len5;
                            *base.add(0).cast::<*mut u8>() = ptr5.cast_mut();
                            *base.add(8).cast::<f32>() = _rt::as_f32(t4_1);
                        }
                    }
                    *ptr1.add(24).cast::<usize>() = len6;
                    *ptr1.add(20).cast::<*mut u8>() = result6;
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_font_key<T: GuestFont>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(12).cast::<*mut u8>();
                    let l1 = *arg0.add(16).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                    let l2 = *arg0.add(20).cast::<*mut u8>();
                    let l3 = *arg0.add(24).cast::<usize>();
                    let base6 = l2;
                    let len6 = l3;
                    for i in 0..len6 {
                        let base = base6.add(i * 12);
                        {
                            let l4 = *base.add(0).cast::<*mut u8>();
                            let l5 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                    _rt::cabi_dealloc(base6, len6 * 12, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_measure_cabi<T: GuestFont>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::measure(
                        FontBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr2.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_font_measure<T: GuestFont>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_ascender_cabi<T: GuestFont>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::ascender(
                        FontBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_descender_cabi<T: GuestFont>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::descender(
                        FontBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_units_per_em_cabi<T: GuestFont>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::units_per_em(
                        FontBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_bitmap_cabi<T: GuestFont>(
                    arg0: *mut u8,
                    arg1: i32,
                    arg2: f32,
                    arg3: f32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::bitmap(
                        FontBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::char_lift(arg1 as u32),
                        arg2,
                        arg3,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr1.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_underline_metrics_cabi<T: GuestFont>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::underline_metrics(
                        FontBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let LineMetrics {
                                position: position2,
                                thickness: thickness2,
                            } = e;
                            *ptr1.add(2).cast::<u16>() = (_rt::as_i32(position2)) as u16;
                            *ptr1.add(4).cast::<u16>() = (_rt::as_i32(thickness2))
                                as u16;
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_font_kit_cabi<T: GuestFontKit>() -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = FontKit::new(T::new());
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_set_lru_limit_cabi<
                    T: GuestFontKit,
                >(arg0: *mut u8, arg1: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_lru_limit(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_add_font_from_buffer_cabi<
                    T: GuestFontKit,
                >(arg0: *mut u8, arg1: *mut u8, arg2: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    T::add_font_from_buffer(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_add_search_path_cabi<
                    T: GuestFontKit,
                >(arg0: *mut u8, arg1: *mut u8, arg2: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    T::add_search_path(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_query_cabi<T: GuestFontKit>(
                    arg0: *mut u8,
                    arg1: i32,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                    arg6: i32,
                    arg7: *mut u8,
                    arg8: usize,
                    arg9: *mut u8,
                    arg10: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg8;
                    let bytes0 = _rt::Vec::from_raw_parts(arg7.cast(), len0, len0);
                    let base5 = arg9;
                    let len5 = arg10;
                    let mut result5 = _rt::Vec::with_capacity(len5);
                    for i in 0..len5 {
                        let base = base5.add(i * 12);
                        let e5 = {
                            let l1 = *base.add(0).cast::<*mut u8>();
                            let l2 = *base.add(4).cast::<usize>();
                            let len3 = l2;
                            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                            let l4 = *base.add(8).cast::<f32>();
                            (_rt::string_lift(bytes3), l4)
                        };
                        result5.push(e5);
                    }
                    _rt::cabi_dealloc(base5, len5 * 12, 4);
                    let result6 = T::query(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                        super::super::super::super::__with_name0::FontKey {
                            weight: match arg1 {
                                0 => None,
                                1 => {
                                    let e = arg2 as u16;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            italic: match arg3 {
                                0 => None,
                                1 => {
                                    let e = _rt::bool_lift(arg4 as u8);
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            stretch: match arg5 {
                                0 => None,
                                1 => {
                                    let e = arg6 as u16;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            family: _rt::string_lift(bytes0),
                            variations: result5,
                        },
                    );
                    let ptr7 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result6 {
                        Some(e) => {
                            *ptr7.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr7.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        None => {
                            *ptr7.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr7
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_query_font_info_cabi<
                    T: GuestFontKit,
                >(
                    arg0: *mut u8,
                    arg1: i32,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                    arg6: i32,
                    arg7: *mut u8,
                    arg8: usize,
                    arg9: *mut u8,
                    arg10: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg8;
                    let bytes0 = _rt::Vec::from_raw_parts(arg7.cast(), len0, len0);
                    let base5 = arg9;
                    let len5 = arg10;
                    let mut result5 = _rt::Vec::with_capacity(len5);
                    for i in 0..len5 {
                        let base = base5.add(i * 12);
                        let e5 = {
                            let l1 = *base.add(0).cast::<*mut u8>();
                            let l2 = *base.add(4).cast::<usize>();
                            let len3 = l2;
                            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                            let l4 = *base.add(8).cast::<f32>();
                            (_rt::string_lift(bytes3), l4)
                        };
                        result5.push(e5);
                    }
                    _rt::cabi_dealloc(base5, len5 * 12, 4);
                    let result6 = T::query_font_info(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                        super::super::super::super::__with_name0::FontKey {
                            weight: match arg1 {
                                0 => None,
                                1 => {
                                    let e = arg2 as u16;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            italic: match arg3 {
                                0 => None,
                                1 => {
                                    let e = _rt::bool_lift(arg4 as u8);
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            stretch: match arg5 {
                                0 => None,
                                1 => {
                                    let e = arg6 as u16;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            family: _rt::string_lift(bytes0),
                            variations: result5,
                        },
                    );
                    let ptr7 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result6 {
                        Some(e) => {
                            *ptr7.add(0).cast::<u8>() = (1i32) as u8;
                            let vec21 = e;
                            let len21 = vec21.len();
                            let layout21 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec21.len() * 56,
                                4,
                            );
                            let result21 = if layout21.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout21).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout21);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec21.into_iter().enumerate() {
                                let base = result21.add(i * 56);
                                {
                                    let FontInfo {
                                        style_names: style_names8,
                                        names: names8,
                                        path: path8,
                                        key: key8,
                                    } = e;
                                    let vec11 = style_names8;
                                    let len11 = vec11.len();
                                    let layout11 = _rt::alloc::Layout::from_size_align_unchecked(
                                        vec11.len() * 16,
                                        4,
                                    );
                                    let result11 = if layout11.size() != 0 {
                                        let ptr = _rt::alloc::alloc(layout11).cast::<u8>();
                                        if ptr.is_null() {
                                            _rt::alloc::handle_alloc_error(layout11);
                                        }
                                        ptr
                                    } else {
                                        ::core::ptr::null_mut()
                                    };
                                    for (i, e) in vec11.into_iter().enumerate() {
                                        let base = result11.add(i * 16);
                                        {
                                            let Name {
                                                id: id9,
                                                name: name9,
                                                language_id: language_id9,
                                            } = e;
                                            *base.add(0).cast::<u16>() = (_rt::as_i32(id9)) as u16;
                                            let vec10 = (name9.into_bytes()).into_boxed_slice();
                                            let ptr10 = vec10.as_ptr().cast::<u8>();
                                            let len10 = vec10.len();
                                            ::core::mem::forget(vec10);
                                            *base.add(8).cast::<usize>() = len10;
                                            *base.add(4).cast::<*mut u8>() = ptr10.cast_mut();
                                            *base.add(12).cast::<u16>() = (_rt::as_i32(language_id9))
                                                as u16;
                                        }
                                    }
                                    *base.add(4).cast::<usize>() = len11;
                                    *base.add(0).cast::<*mut u8>() = result11;
                                    let vec14 = names8;
                                    let len14 = vec14.len();
                                    let layout14 = _rt::alloc::Layout::from_size_align_unchecked(
                                        vec14.len() * 16,
                                        4,
                                    );
                                    let result14 = if layout14.size() != 0 {
                                        let ptr = _rt::alloc::alloc(layout14).cast::<u8>();
                                        if ptr.is_null() {
                                            _rt::alloc::handle_alloc_error(layout14);
                                        }
                                        ptr
                                    } else {
                                        ::core::ptr::null_mut()
                                    };
                                    for (i, e) in vec14.into_iter().enumerate() {
                                        let base = result14.add(i * 16);
                                        {
                                            let Name {
                                                id: id12,
                                                name: name12,
                                                language_id: language_id12,
                                            } = e;
                                            *base.add(0).cast::<u16>() = (_rt::as_i32(id12)) as u16;
                                            let vec13 = (name12.into_bytes()).into_boxed_slice();
                                            let ptr13 = vec13.as_ptr().cast::<u8>();
                                            let len13 = vec13.len();
                                            ::core::mem::forget(vec13);
                                            *base.add(8).cast::<usize>() = len13;
                                            *base.add(4).cast::<*mut u8>() = ptr13.cast_mut();
                                            *base.add(12).cast::<u16>() = (_rt::as_i32(language_id12))
                                                as u16;
                                        }
                                    }
                                    *base.add(12).cast::<usize>() = len14;
                                    *base.add(8).cast::<*mut u8>() = result14;
                                    match path8 {
                                        Some(e) => {
                                            *base.add(16).cast::<u8>() = (1i32) as u8;
                                            let vec15 = (e.into_bytes()).into_boxed_slice();
                                            let ptr15 = vec15.as_ptr().cast::<u8>();
                                            let len15 = vec15.len();
                                            ::core::mem::forget(vec15);
                                            *base.add(24).cast::<usize>() = len15;
                                            *base.add(20).cast::<*mut u8>() = ptr15.cast_mut();
                                        }
                                        None => {
                                            *base.add(16).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                    let super::super::super::super::__with_name0::FontKey {
                                        weight: weight16,
                                        italic: italic16,
                                        stretch: stretch16,
                                        family: family16,
                                        variations: variations16,
                                    } = key8;
                                    match weight16 {
                                        Some(e) => {
                                            *base.add(28).cast::<u8>() = (1i32) as u8;
                                            *base.add(30).cast::<u16>() = (_rt::as_i32(e)) as u16;
                                        }
                                        None => {
                                            *base.add(28).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                    match italic16 {
                                        Some(e) => {
                                            *base.add(32).cast::<u8>() = (1i32) as u8;
                                            *base.add(33).cast::<u8>() = (match e {
                                                true => 1,
                                                false => 0,
                                            }) as u8;
                                        }
                                        None => {
                                            *base.add(32).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                    match stretch16 {
                                        Some(e) => {
                                            *base.add(34).cast::<u8>() = (1i32) as u8;
                                            *base.add(36).cast::<u16>() = (_rt::as_i32(e)) as u16;
                                        }
                                        None => {
                                            *base.add(34).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                    let vec17 = (family16.into_bytes()).into_boxed_slice();
                                    let ptr17 = vec17.as_ptr().cast::<u8>();
                                    let len17 = vec17.len();
                                    ::core::mem::forget(vec17);
                                    *base.add(44).cast::<usize>() = len17;
                                    *base.add(40).cast::<*mut u8>() = ptr17.cast_mut();
                                    let vec20 = variations16;
                                    let len20 = vec20.len();
                                    let layout20 = _rt::alloc::Layout::from_size_align_unchecked(
                                        vec20.len() * 12,
                                        4,
                                    );
                                    let result20 = if layout20.size() != 0 {
                                        let ptr = _rt::alloc::alloc(layout20).cast::<u8>();
                                        if ptr.is_null() {
                                            _rt::alloc::handle_alloc_error(layout20);
                                        }
                                        ptr
                                    } else {
                                        ::core::ptr::null_mut()
                                    };
                                    for (i, e) in vec20.into_iter().enumerate() {
                                        let base = result20.add(i * 12);
                                        {
                                            let (t18_0, t18_1) = e;
                                            let vec19 = (t18_0.into_bytes()).into_boxed_slice();
                                            let ptr19 = vec19.as_ptr().cast::<u8>();
                                            let len19 = vec19.len();
                                            ::core::mem::forget(vec19);
                                            *base.add(4).cast::<usize>() = len19;
                                            *base.add(0).cast::<*mut u8>() = ptr19.cast_mut();
                                            *base.add(8).cast::<f32>() = _rt::as_f32(t18_1);
                                        }
                                    }
                                    *base.add(52).cast::<usize>() = len20;
                                    *base.add(48).cast::<*mut u8>() = result20;
                                }
                            }
                            *ptr7.add(8).cast::<usize>() = len21;
                            *ptr7.add(4).cast::<*mut u8>() = result21;
                        }
                        None => {
                            *ptr7.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr7
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_font_kit_query_font_info<
                    T: GuestFontKit,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base23 = l1;
                            let len23 = l2;
                            for i in 0..len23 {
                                let base = base23.add(i * 56);
                                {
                                    let l3 = *base.add(0).cast::<*mut u8>();
                                    let l4 = *base.add(4).cast::<usize>();
                                    let base7 = l3;
                                    let len7 = l4;
                                    for i in 0..len7 {
                                        let base = base7.add(i * 16);
                                        {
                                            let l5 = *base.add(4).cast::<*mut u8>();
                                            let l6 = *base.add(8).cast::<usize>();
                                            _rt::cabi_dealloc(l5, l6, 1);
                                        }
                                    }
                                    _rt::cabi_dealloc(base7, len7 * 16, 4);
                                    let l8 = *base.add(8).cast::<*mut u8>();
                                    let l9 = *base.add(12).cast::<usize>();
                                    let base12 = l8;
                                    let len12 = l9;
                                    for i in 0..len12 {
                                        let base = base12.add(i * 16);
                                        {
                                            let l10 = *base.add(4).cast::<*mut u8>();
                                            let l11 = *base.add(8).cast::<usize>();
                                            _rt::cabi_dealloc(l10, l11, 1);
                                        }
                                    }
                                    _rt::cabi_dealloc(base12, len12 * 16, 4);
                                    let l13 = i32::from(*base.add(16).cast::<u8>());
                                    match l13 {
                                        0 => {}
                                        _ => {
                                            let l14 = *base.add(20).cast::<*mut u8>();
                                            let l15 = *base.add(24).cast::<usize>();
                                            _rt::cabi_dealloc(l14, l15, 1);
                                        }
                                    }
                                    let l16 = *base.add(40).cast::<*mut u8>();
                                    let l17 = *base.add(44).cast::<usize>();
                                    _rt::cabi_dealloc(l16, l17, 1);
                                    let l18 = *base.add(48).cast::<*mut u8>();
                                    let l19 = *base.add(52).cast::<usize>();
                                    let base22 = l18;
                                    let len22 = l19;
                                    for i in 0..len22 {
                                        let base = base22.add(i * 12);
                                        {
                                            let l20 = *base.add(0).cast::<*mut u8>();
                                            let l21 = *base.add(4).cast::<usize>();
                                            _rt::cabi_dealloc(l20, l21, 1);
                                        }
                                    }
                                    _rt::cabi_dealloc(base22, len22 * 12, 4);
                                }
                            }
                            _rt::cabi_dealloc(base23, len23 * 56, 4);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_exact_match_cabi<T: GuestFontKit>(
                    arg0: *mut u8,
                    arg1: i32,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                    arg6: i32,
                    arg7: *mut u8,
                    arg8: usize,
                    arg9: *mut u8,
                    arg10: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg8;
                    let bytes0 = _rt::Vec::from_raw_parts(arg7.cast(), len0, len0);
                    let base5 = arg9;
                    let len5 = arg10;
                    let mut result5 = _rt::Vec::with_capacity(len5);
                    for i in 0..len5 {
                        let base = base5.add(i * 12);
                        let e5 = {
                            let l1 = *base.add(0).cast::<*mut u8>();
                            let l2 = *base.add(4).cast::<usize>();
                            let len3 = l2;
                            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                            let l4 = *base.add(8).cast::<f32>();
                            (_rt::string_lift(bytes3), l4)
                        };
                        result5.push(e5);
                    }
                    _rt::cabi_dealloc(base5, len5 * 12, 4);
                    let result6 = T::exact_match(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                        super::super::super::super::__with_name0::FontKey {
                            weight: match arg1 {
                                0 => None,
                                1 => {
                                    let e = arg2 as u16;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            italic: match arg3 {
                                0 => None,
                                1 => {
                                    let e = _rt::bool_lift(arg4 as u8);
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            stretch: match arg5 {
                                0 => None,
                                1 => {
                                    let e = arg6 as u16;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            family: _rt::string_lift(bytes0),
                            variations: result5,
                        },
                    );
                    let ptr7 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result6 {
                        Some(e) => {
                            *ptr7.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr7.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        None => {
                            *ptr7.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr7
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_fonts_info_cabi<T: GuestFontKit>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::fonts_info(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec15 = result0;
                    let len15 = vec15.len();
                    let layout15 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec15.len() * 56,
                        4,
                    );
                    let result15 = if layout15.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout15).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout15);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec15.into_iter().enumerate() {
                        let base = result15.add(i * 56);
                        {
                            let FontInfo {
                                style_names: style_names2,
                                names: names2,
                                path: path2,
                                key: key2,
                            } = e;
                            let vec5 = style_names2;
                            let len5 = vec5.len();
                            let layout5 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec5.len() * 16,
                                4,
                            );
                            let result5 = if layout5.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout5).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout5);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec5.into_iter().enumerate() {
                                let base = result5.add(i * 16);
                                {
                                    let Name {
                                        id: id3,
                                        name: name3,
                                        language_id: language_id3,
                                    } = e;
                                    *base.add(0).cast::<u16>() = (_rt::as_i32(id3)) as u16;
                                    let vec4 = (name3.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                    let len4 = vec4.len();
                                    ::core::mem::forget(vec4);
                                    *base.add(8).cast::<usize>() = len4;
                                    *base.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                                    *base.add(12).cast::<u16>() = (_rt::as_i32(language_id3))
                                        as u16;
                                }
                            }
                            *base.add(4).cast::<usize>() = len5;
                            *base.add(0).cast::<*mut u8>() = result5;
                            let vec8 = names2;
                            let len8 = vec8.len();
                            let layout8 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec8.len() * 16,
                                4,
                            );
                            let result8 = if layout8.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout8).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout8);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec8.into_iter().enumerate() {
                                let base = result8.add(i * 16);
                                {
                                    let Name {
                                        id: id6,
                                        name: name6,
                                        language_id: language_id6,
                                    } = e;
                                    *base.add(0).cast::<u16>() = (_rt::as_i32(id6)) as u16;
                                    let vec7 = (name6.into_bytes()).into_boxed_slice();
                                    let ptr7 = vec7.as_ptr().cast::<u8>();
                                    let len7 = vec7.len();
                                    ::core::mem::forget(vec7);
                                    *base.add(8).cast::<usize>() = len7;
                                    *base.add(4).cast::<*mut u8>() = ptr7.cast_mut();
                                    *base.add(12).cast::<u16>() = (_rt::as_i32(language_id6))
                                        as u16;
                                }
                            }
                            *base.add(12).cast::<usize>() = len8;
                            *base.add(8).cast::<*mut u8>() = result8;
                            match path2 {
                                Some(e) => {
                                    *base.add(16).cast::<u8>() = (1i32) as u8;
                                    let vec9 = (e.into_bytes()).into_boxed_slice();
                                    let ptr9 = vec9.as_ptr().cast::<u8>();
                                    let len9 = vec9.len();
                                    ::core::mem::forget(vec9);
                                    *base.add(24).cast::<usize>() = len9;
                                    *base.add(20).cast::<*mut u8>() = ptr9.cast_mut();
                                }
                                None => {
                                    *base.add(16).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let super::super::super::super::__with_name0::FontKey {
                                weight: weight10,
                                italic: italic10,
                                stretch: stretch10,
                                family: family10,
                                variations: variations10,
                            } = key2;
                            match weight10 {
                                Some(e) => {
                                    *base.add(28).cast::<u8>() = (1i32) as u8;
                                    *base.add(30).cast::<u16>() = (_rt::as_i32(e)) as u16;
                                }
                                None => {
                                    *base.add(28).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            match italic10 {
                                Some(e) => {
                                    *base.add(32).cast::<u8>() = (1i32) as u8;
                                    *base.add(33).cast::<u8>() = (match e {
                                        true => 1,
                                        false => 0,
                                    }) as u8;
                                }
                                None => {
                                    *base.add(32).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            match stretch10 {
                                Some(e) => {
                                    *base.add(34).cast::<u8>() = (1i32) as u8;
                                    *base.add(36).cast::<u16>() = (_rt::as_i32(e)) as u16;
                                }
                                None => {
                                    *base.add(34).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec11 = (family10.into_bytes()).into_boxed_slice();
                            let ptr11 = vec11.as_ptr().cast::<u8>();
                            let len11 = vec11.len();
                            ::core::mem::forget(vec11);
                            *base.add(44).cast::<usize>() = len11;
                            *base.add(40).cast::<*mut u8>() = ptr11.cast_mut();
                            let vec14 = variations10;
                            let len14 = vec14.len();
                            let layout14 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec14.len() * 12,
                                4,
                            );
                            let result14 = if layout14.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout14).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout14);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec14.into_iter().enumerate() {
                                let base = result14.add(i * 12);
                                {
                                    let (t12_0, t12_1) = e;
                                    let vec13 = (t12_0.into_bytes()).into_boxed_slice();
                                    let ptr13 = vec13.as_ptr().cast::<u8>();
                                    let len13 = vec13.len();
                                    ::core::mem::forget(vec13);
                                    *base.add(4).cast::<usize>() = len13;
                                    *base.add(0).cast::<*mut u8>() = ptr13.cast_mut();
                                    *base.add(8).cast::<f32>() = _rt::as_f32(t12_1);
                                }
                            }
                            *base.add(52).cast::<usize>() = len14;
                            *base.add(48).cast::<*mut u8>() = result14;
                        }
                    }
                    *ptr1.add(4).cast::<usize>() = len15;
                    *ptr1.add(0).cast::<*mut u8>() = result15;
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_font_kit_fonts_info<T: GuestFontKit>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base22 = l0;
                    let len22 = l1;
                    for i in 0..len22 {
                        let base = base22.add(i * 56);
                        {
                            let l2 = *base.add(0).cast::<*mut u8>();
                            let l3 = *base.add(4).cast::<usize>();
                            let base6 = l2;
                            let len6 = l3;
                            for i in 0..len6 {
                                let base = base6.add(i * 16);
                                {
                                    let l4 = *base.add(4).cast::<*mut u8>();
                                    let l5 = *base.add(8).cast::<usize>();
                                    _rt::cabi_dealloc(l4, l5, 1);
                                }
                            }
                            _rt::cabi_dealloc(base6, len6 * 16, 4);
                            let l7 = *base.add(8).cast::<*mut u8>();
                            let l8 = *base.add(12).cast::<usize>();
                            let base11 = l7;
                            let len11 = l8;
                            for i in 0..len11 {
                                let base = base11.add(i * 16);
                                {
                                    let l9 = *base.add(4).cast::<*mut u8>();
                                    let l10 = *base.add(8).cast::<usize>();
                                    _rt::cabi_dealloc(l9, l10, 1);
                                }
                            }
                            _rt::cabi_dealloc(base11, len11 * 16, 4);
                            let l12 = i32::from(*base.add(16).cast::<u8>());
                            match l12 {
                                0 => {}
                                _ => {
                                    let l13 = *base.add(20).cast::<*mut u8>();
                                    let l14 = *base.add(24).cast::<usize>();
                                    _rt::cabi_dealloc(l13, l14, 1);
                                }
                            }
                            let l15 = *base.add(40).cast::<*mut u8>();
                            let l16 = *base.add(44).cast::<usize>();
                            _rt::cabi_dealloc(l15, l16, 1);
                            let l17 = *base.add(48).cast::<*mut u8>();
                            let l18 = *base.add(52).cast::<usize>();
                            let base21 = l17;
                            let len21 = l18;
                            for i in 0..len21 {
                                let base = base21.add(i * 12);
                                {
                                    let l19 = *base.add(0).cast::<*mut u8>();
                                    let l20 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l19, l20, 1);
                                }
                            }
                            _rt::cabi_dealloc(base21, len21 * 12, 4);
                        }
                    }
                    _rt::cabi_dealloc(base22, len22 * 56, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_len_cabi<T: GuestFontKit>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::len(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_remove_cabi<T: GuestFontKit>(
                    arg0: *mut u8,
                    arg1: i32,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                    arg6: i32,
                    arg7: *mut u8,
                    arg8: usize,
                    arg9: *mut u8,
                    arg10: usize,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg8;
                    let bytes0 = _rt::Vec::from_raw_parts(arg7.cast(), len0, len0);
                    let base5 = arg9;
                    let len5 = arg10;
                    let mut result5 = _rt::Vec::with_capacity(len5);
                    for i in 0..len5 {
                        let base = base5.add(i * 12);
                        let e5 = {
                            let l1 = *base.add(0).cast::<*mut u8>();
                            let l2 = *base.add(4).cast::<usize>();
                            let len3 = l2;
                            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                            let l4 = *base.add(8).cast::<f32>();
                            (_rt::string_lift(bytes3), l4)
                        };
                        result5.push(e5);
                    }
                    _rt::cabi_dealloc(base5, len5 * 12, 4);
                    T::remove(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                        super::super::super::super::__with_name0::FontKey {
                            weight: match arg1 {
                                0 => None,
                                1 => {
                                    let e = arg2 as u16;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            italic: match arg3 {
                                0 => None,
                                1 => {
                                    let e = _rt::bool_lift(arg4 as u8);
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            stretch: match arg5 {
                                0 => None,
                                1 => {
                                    let e = arg6 as u16;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            family: _rt::string_lift(bytes0),
                            variations: result5,
                        },
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_measure_cabi<T: GuestFontKit>(
                    arg0: *mut u8,
                    arg1: i32,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
                    arg6: i32,
                    arg7: *mut u8,
                    arg8: usize,
                    arg9: *mut u8,
                    arg10: usize,
                    arg11: *mut u8,
                    arg12: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg8;
                    let bytes0 = _rt::Vec::from_raw_parts(arg7.cast(), len0, len0);
                    let base5 = arg9;
                    let len5 = arg10;
                    let mut result5 = _rt::Vec::with_capacity(len5);
                    for i in 0..len5 {
                        let base = base5.add(i * 12);
                        let e5 = {
                            let l1 = *base.add(0).cast::<*mut u8>();
                            let l2 = *base.add(4).cast::<usize>();
                            let len3 = l2;
                            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                            let l4 = *base.add(8).cast::<f32>();
                            (_rt::string_lift(bytes3), l4)
                        };
                        result5.push(e5);
                    }
                    _rt::cabi_dealloc(base5, len5 * 12, 4);
                    let len6 = arg12;
                    let bytes6 = _rt::Vec::from_raw_parts(arg11.cast(), len6, len6);
                    let result7 = T::measure(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                        super::super::super::super::__with_name0::FontKey {
                            weight: match arg1 {
                                0 => None,
                                1 => {
                                    let e = arg2 as u16;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            italic: match arg3 {
                                0 => None,
                                1 => {
                                    let e = _rt::bool_lift(arg4 as u8);
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            stretch: match arg5 {
                                0 => None,
                                1 => {
                                    let e = arg6 as u16;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            family: _rt::string_lift(bytes0),
                            variations: result5,
                        },
                        _rt::string_lift(bytes6),
                    );
                    let ptr8 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result7 {
                        Some(e) => {
                            *ptr8.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr8.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        None => {
                            *ptr8.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr8
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_write_data_cabi<T: GuestFontKit>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::write_data(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_font_kit_write_data<T: GuestFontKit>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_read_data_cabi<T: GuestFontKit>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    T::read_data(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_font_kit_buffer_size_cabi<T: GuestFontKit>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::buffer_size(
                        FontKitBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_str_width_to_number_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::str_width_to_number(_rt::string_lift(bytes0));
                    _rt::as_i32(result1)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_number_width_to_str_cabi<T: Guest>(
                    arg0: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::number_width_to_str(arg0 as u16);
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_number_width_to_str<T: Guest>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    type TextMetrics: GuestTextMetrics;
                    type GlyphBitmap: GuestGlyphBitmap;
                    type Font: GuestFont;
                    type FontKit: GuestFontKit;
                    fn str_width_to_number(width: _rt::String) -> u16;
                    fn number_width_to_str(width: u16) -> _rt::String;
                }
                pub trait GuestTextMetrics: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]alibaba:fontkit/fontkit-interface"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]text-metrics"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]alibaba:fontkit/fontkit-interface"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]text-metrics"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(value: _rt::String) -> Self;
                    fn duplicate(&self) -> TextMetrics;
                    fn width(&self, font_size: f32, letter_spacing: f32) -> f32;
                    fn height(&self, font_size: f32, line_height: Option<f32>) -> f32;
                    fn ascender(&self, font_size: f32) -> f32;
                    fn line_gap(&self) -> f32;
                    fn units(&self) -> f32;
                    fn slice(&self, start: u32, count: u32) -> TextMetrics;
                    fn value(&self) -> _rt::String;
                    fn is_rtl(&self) -> bool;
                    fn append(&self, other: TextMetrics);
                    fn count(&self) -> u32;
                    /// replace this metrics with another, allowing fallback logic
                    fn replace(&self, other: TextMetrics, fallback: bool);
                    fn split_by_width(
                        &self,
                        font_size: f32,
                        letter_spacing: f32,
                        width: f32,
                    ) -> TextMetrics;
                    fn chars(&self) -> _rt::Vec<char>;
                }
                pub trait GuestGlyphBitmap: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]alibaba:fontkit/fontkit-interface"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]glyph-bitmap"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]alibaba:fontkit/fontkit-interface"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]glyph-bitmap"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn width(&self) -> u32;
                    fn height(&self) -> u32;
                    fn bitmap(&self) -> _rt::Vec<u8>;
                    fn x_min(&self) -> f32;
                    fn y_max(&self) -> f32;
                    fn stroke_x(&self) -> f32;
                    fn stroke_y(&self) -> f32;
                    fn stroke_bitmap(&self) -> Option<(_rt::Vec<u8>, u32)>;
                    fn advanced_x(&self) -> f32;
                    fn ascender(&self) -> f32;
                    fn descender(&self) -> f32;
                }
                pub trait GuestFont: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]alibaba:fontkit/fontkit-interface"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]font"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]alibaba:fontkit/fontkit-interface"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]font"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    /// Check if the font has valid data for a character
                    fn has_glyph(&self, c: char) -> bool;
                    /// Output the svg path string of a glyph
                    fn glyph_path_string(&self, c: char) -> Option<_rt::String>;
                    /// Return the font buffer
                    fn buffer(&self) -> _rt::Vec<u8>;
                    /// Return the path if this font is added from searching a path
                    fn path(&self) -> _rt::String;
                    /// Return the key of this font
                    fn key(&self) -> FontKey;
                    /// Measure text using this font
                    fn measure(
                        &self,
                        text: _rt::String,
                    ) -> Result<TextMetrics, _rt::String>;
                    fn ascender(&self) -> i16;
                    fn descender(&self) -> i16;
                    fn units_per_em(&self) -> u16;
                    fn bitmap(
                        &self,
                        c: char,
                        font_size: f32,
                        stroke_width: f32,
                    ) -> Option<GlyphBitmap>;
                    fn underline_metrics(&self) -> Option<LineMetrics>;
                }
                pub trait GuestFontKit: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]alibaba:fontkit/fontkit-interface"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]font-kit"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]alibaba:fontkit/fontkit-interface"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]font-kit"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new() -> Self;
                    /// add an LRU limit for font buffer registry, `limit`'s unit is KB, 0 means caching is disabled
                    fn set_lru_limit(&self, limit: u32);
                    /// Register a font (or several fonts in case of ttc), return the keys of added fonts.
                    /// The file type is extracted from the buffer by checking magic numbers
                    fn add_font_from_buffer(&self, buffer: _rt::Vec<u8>);
                    /// Search and add fonts from a path
                    fn add_search_path(&self, path: _rt::String);
                    /// Query font using a key, this API returns valid result only if a single result is found
                    fn query(&self, key: FontKey) -> Option<Font>;
                    /// Query the info of font, even if the font is unloaded and `query` returns `None`
                    fn query_font_info(
                        &self,
                        key: FontKey,
                    ) -> Option<_rt::Vec<FontInfo>>;
                    /// Using exact-match method to directly obtain a font, skipping the querying logic
                    fn exact_match(&self, key: FontKey) -> Option<Font>;
                    /// Get detailed info of all fonts registered
                    fn fonts_info(&self) -> _rt::Vec<FontInfo>;
                    /// Get number of registered fonts
                    fn len(&self) -> u32;
                    /// Remove a font matching the given key
                    fn remove(&self, key: FontKey);
                    /// Measure a text with some fallback logic
                    fn measure(
                        &self,
                        key: FontKey,
                        text: _rt::String,
                    ) -> Option<TextMetrics>;
                    /// Export all font data into a JSON string
                    fn write_data(&self) -> _rt::String;
                    /// Load font data from JSON
                    fn read_data(&self, data: _rt::String);
                    fn buffer_size(&self) -> u32;
                }
                #[doc(hidden)]
                macro_rules! __export_alibaba_fontkit_fontkit_interface_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "alibaba:fontkit/fontkit-interface#[constructor]text-metrics"]
                        unsafe extern "C" fn export_constructor_text_metrics(arg0 : * mut
                        u8, arg1 : usize,) -> i32 { $($path_to_types)*::
                        _export_constructor_text_metrics_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0, arg1) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.duplicate"]
                        unsafe extern "C" fn export_method_text_metrics_duplicate(arg0 :
                        * mut u8,) -> i32 { $($path_to_types)*::
                        _export_method_text_metrics_duplicate_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.width"]
                        unsafe extern "C" fn export_method_text_metrics_width(arg0 : *
                        mut u8, arg1 : f32, arg2 : f32,) -> f32 { $($path_to_types)*::
                        _export_method_text_metrics_width_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0, arg1, arg2) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.height"]
                        unsafe extern "C" fn export_method_text_metrics_height(arg0 : *
                        mut u8, arg1 : f32, arg2 : i32, arg3 : f32,) -> f32 {
                        $($path_to_types)*::
                        _export_method_text_metrics_height_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.ascender"]
                        unsafe extern "C" fn export_method_text_metrics_ascender(arg0 : *
                        mut u8, arg1 : f32,) -> f32 { $($path_to_types)*::
                        _export_method_text_metrics_ascender_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0, arg1) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.line-gap"]
                        unsafe extern "C" fn export_method_text_metrics_line_gap(arg0 : *
                        mut u8,) -> f32 { $($path_to_types)*::
                        _export_method_text_metrics_line_gap_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.units"]
                        unsafe extern "C" fn export_method_text_metrics_units(arg0 : *
                        mut u8,) -> f32 { $($path_to_types)*::
                        _export_method_text_metrics_units_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.slice"]
                        unsafe extern "C" fn export_method_text_metrics_slice(arg0 : *
                        mut u8, arg1 : i32, arg2 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_text_metrics_slice_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0, arg1, arg2) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.value"]
                        unsafe extern "C" fn export_method_text_metrics_value(arg0 : *
                        mut u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_text_metrics_value_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0) }
                        #[export_name =
                        "cabi_post_alibaba:fontkit/fontkit-interface#[method]text-metrics.value"]
                        unsafe extern "C" fn _post_return_method_text_metrics_value(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_text_metrics_value::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.is-rtl"]
                        unsafe extern "C" fn export_method_text_metrics_is_rtl(arg0 : *
                        mut u8,) -> i32 { $($path_to_types)*::
                        _export_method_text_metrics_is_rtl_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.append"]
                        unsafe extern "C" fn export_method_text_metrics_append(arg0 : *
                        mut u8, arg1 : i32,) { $($path_to_types)*::
                        _export_method_text_metrics_append_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0, arg1) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.count"]
                        unsafe extern "C" fn export_method_text_metrics_count(arg0 : *
                        mut u8,) -> i32 { $($path_to_types)*::
                        _export_method_text_metrics_count_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.replace"]
                        unsafe extern "C" fn export_method_text_metrics_replace(arg0 : *
                        mut u8, arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_text_metrics_replace_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0, arg1, arg2) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.split-by-width"]
                        unsafe extern "C" fn
                        export_method_text_metrics_split_by_width(arg0 : * mut u8, arg1 :
                        f32, arg2 : f32, arg3 : f32,) -> i32 { $($path_to_types)*::
                        _export_method_text_metrics_split_by_width_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]text-metrics.chars"]
                        unsafe extern "C" fn export_method_text_metrics_chars(arg0 : *
                        mut u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_text_metrics_chars_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0) }
                        #[export_name =
                        "cabi_post_alibaba:fontkit/fontkit-interface#[method]text-metrics.chars"]
                        unsafe extern "C" fn _post_return_method_text_metrics_chars(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_text_metrics_chars::<<$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.width"]
                        unsafe extern "C" fn export_method_glyph_bitmap_width(arg0 : *
                        mut u8,) -> i32 { $($path_to_types)*::
                        _export_method_glyph_bitmap_width_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.height"]
                        unsafe extern "C" fn export_method_glyph_bitmap_height(arg0 : *
                        mut u8,) -> i32 { $($path_to_types)*::
                        _export_method_glyph_bitmap_height_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.bitmap"]
                        unsafe extern "C" fn export_method_glyph_bitmap_bitmap(arg0 : *
                        mut u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_glyph_bitmap_bitmap_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "cabi_post_alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.bitmap"]
                        unsafe extern "C" fn _post_return_method_glyph_bitmap_bitmap(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_glyph_bitmap_bitmap::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.x-min"]
                        unsafe extern "C" fn export_method_glyph_bitmap_x_min(arg0 : *
                        mut u8,) -> f32 { $($path_to_types)*::
                        _export_method_glyph_bitmap_x_min_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.y-max"]
                        unsafe extern "C" fn export_method_glyph_bitmap_y_max(arg0 : *
                        mut u8,) -> f32 { $($path_to_types)*::
                        _export_method_glyph_bitmap_y_max_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.stroke-x"]
                        unsafe extern "C" fn export_method_glyph_bitmap_stroke_x(arg0 : *
                        mut u8,) -> f32 { $($path_to_types)*::
                        _export_method_glyph_bitmap_stroke_x_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.stroke-y"]
                        unsafe extern "C" fn export_method_glyph_bitmap_stroke_y(arg0 : *
                        mut u8,) -> f32 { $($path_to_types)*::
                        _export_method_glyph_bitmap_stroke_y_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.stroke-bitmap"]
                        unsafe extern "C" fn
                        export_method_glyph_bitmap_stroke_bitmap(arg0 : * mut u8,) -> *
                        mut u8 { $($path_to_types)*::
                        _export_method_glyph_bitmap_stroke_bitmap_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "cabi_post_alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.stroke-bitmap"]
                        unsafe extern "C" fn
                        _post_return_method_glyph_bitmap_stroke_bitmap(arg0 : * mut u8,)
                        { $($path_to_types)*::
                        __post_return_method_glyph_bitmap_stroke_bitmap::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.advanced-x"]
                        unsafe extern "C" fn export_method_glyph_bitmap_advanced_x(arg0 :
                        * mut u8,) -> f32 { $($path_to_types)*::
                        _export_method_glyph_bitmap_advanced_x_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.ascender"]
                        unsafe extern "C" fn export_method_glyph_bitmap_ascender(arg0 : *
                        mut u8,) -> f32 { $($path_to_types)*::
                        _export_method_glyph_bitmap_ascender_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]glyph-bitmap.descender"]
                        unsafe extern "C" fn export_method_glyph_bitmap_descender(arg0 :
                        * mut u8,) -> f32 { $($path_to_types)*::
                        _export_method_glyph_bitmap_descender_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (arg0) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font.has-glyph"]
                        unsafe extern "C" fn export_method_font_has_glyph(arg0 : * mut
                        u8, arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_font_has_glyph_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Font > (arg0, arg1) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font.glyph-path-string"]
                        unsafe extern "C" fn export_method_font_glyph_path_string(arg0 :
                        * mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_font_glyph_path_string_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Font > (arg0, arg1) } #[export_name
                        =
                        "cabi_post_alibaba:fontkit/fontkit-interface#[method]font.glyph-path-string"]
                        unsafe extern "C" fn
                        _post_return_method_font_glyph_path_string(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_font_glyph_path_string::<<$ty as
                        $($path_to_types)*:: Guest >::Font > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font.buffer"] unsafe
                        extern "C" fn export_method_font_buffer(arg0 : * mut u8,) -> *
                        mut u8 { $($path_to_types)*::
                        _export_method_font_buffer_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Font > (arg0) } #[export_name =
                        "cabi_post_alibaba:fontkit/fontkit-interface#[method]font.buffer"]
                        unsafe extern "C" fn _post_return_method_font_buffer(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_font_buffer::<<$ty as $($path_to_types)*::
                        Guest >::Font > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font.path"] unsafe
                        extern "C" fn export_method_font_path(arg0 : * mut u8,) -> * mut
                        u8 { $($path_to_types)*:: _export_method_font_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Font > (arg0) } #[export_name =
                        "cabi_post_alibaba:fontkit/fontkit-interface#[method]font.path"]
                        unsafe extern "C" fn _post_return_method_font_path(arg0 : * mut
                        u8,) { $($path_to_types)*:: __post_return_method_font_path::<<$ty
                        as $($path_to_types)*:: Guest >::Font > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font.key"] unsafe
                        extern "C" fn export_method_font_key(arg0 : * mut u8,) -> * mut
                        u8 { $($path_to_types)*:: _export_method_font_key_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Font > (arg0) } #[export_name =
                        "cabi_post_alibaba:fontkit/fontkit-interface#[method]font.key"]
                        unsafe extern "C" fn _post_return_method_font_key(arg0 : * mut
                        u8,) { $($path_to_types)*:: __post_return_method_font_key::<<$ty
                        as $($path_to_types)*:: Guest >::Font > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font.measure"] unsafe
                        extern "C" fn export_method_font_measure(arg0 : * mut u8, arg1 :
                        * mut u8, arg2 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_font_measure_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Font > (arg0, arg1, arg2) } #[export_name =
                        "cabi_post_alibaba:fontkit/fontkit-interface#[method]font.measure"]
                        unsafe extern "C" fn _post_return_method_font_measure(arg0 : *
                        mut u8,) { $($path_to_types)*::
                        __post_return_method_font_measure::<<$ty as $($path_to_types)*::
                        Guest >::Font > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font.ascender"] unsafe
                        extern "C" fn export_method_font_ascender(arg0 : * mut u8,) ->
                        i32 { $($path_to_types)*::
                        _export_method_font_ascender_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Font > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font.descender"]
                        unsafe extern "C" fn export_method_font_descender(arg0 : * mut
                        u8,) -> i32 { $($path_to_types)*::
                        _export_method_font_descender_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Font > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font.units-per-em"]
                        unsafe extern "C" fn export_method_font_units_per_em(arg0 : * mut
                        u8,) -> i32 { $($path_to_types)*::
                        _export_method_font_units_per_em_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Font > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font.bitmap"] unsafe
                        extern "C" fn export_method_font_bitmap(arg0 : * mut u8, arg1 :
                        i32, arg2 : f32, arg3 : f32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_font_bitmap_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Font > (arg0, arg1, arg2, arg3) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font.underline-metrics"]
                        unsafe extern "C" fn export_method_font_underline_metrics(arg0 :
                        * mut u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_font_underline_metrics_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Font > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[constructor]font-kit"] unsafe
                        extern "C" fn export_constructor_font_kit() -> i32 {
                        $($path_to_types)*:: _export_constructor_font_kit_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > () } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.set-lru-limit"]
                        unsafe extern "C" fn export_method_font_kit_set_lru_limit(arg0 :
                        * mut u8, arg1 : i32,) { $($path_to_types)*::
                        _export_method_font_kit_set_lru_limit_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0, arg1) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.add-font-from-buffer"]
                        unsafe extern "C" fn
                        export_method_font_kit_add_font_from_buffer(arg0 : * mut u8, arg1
                        : * mut u8, arg2 : usize,) { $($path_to_types)*::
                        _export_method_font_kit_add_font_from_buffer_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0, arg1, arg2) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.add-search-path"]
                        unsafe extern "C" fn export_method_font_kit_add_search_path(arg0
                        : * mut u8, arg1 : * mut u8, arg2 : usize,) {
                        $($path_to_types)*::
                        _export_method_font_kit_add_search_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0, arg1, arg2) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.query"]
                        unsafe extern "C" fn export_method_font_kit_query(arg0 : * mut
                        u8, arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 : i32,
                        arg6 : i32, arg7 : * mut u8, arg8 : usize, arg9 : * mut u8, arg10
                        : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_font_kit_query_cabi::<<$ty as $($path_to_types)*::
                        Guest >::FontKit > (arg0, arg1, arg2, arg3, arg4, arg5, arg6,
                        arg7, arg8, arg9, arg10) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.query-font-info"]
                        unsafe extern "C" fn export_method_font_kit_query_font_info(arg0
                        : * mut u8, arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5
                        : i32, arg6 : i32, arg7 : * mut u8, arg8 : usize, arg9 : * mut
                        u8, arg10 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_font_kit_query_font_info_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0, arg1, arg2, arg3,
                        arg4, arg5, arg6, arg7, arg8, arg9, arg10) } #[export_name =
                        "cabi_post_alibaba:fontkit/fontkit-interface#[method]font-kit.query-font-info"]
                        unsafe extern "C" fn
                        _post_return_method_font_kit_query_font_info(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_font_kit_query_font_info::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.exact-match"]
                        unsafe extern "C" fn export_method_font_kit_exact_match(arg0 : *
                        mut u8, arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 :
                        i32, arg6 : i32, arg7 : * mut u8, arg8 : usize, arg9 : * mut u8,
                        arg10 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_font_kit_exact_match_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0, arg1, arg2, arg3,
                        arg4, arg5, arg6, arg7, arg8, arg9, arg10) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.fonts-info"]
                        unsafe extern "C" fn export_method_font_kit_fonts_info(arg0 : *
                        mut u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_font_kit_fonts_info_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0) } #[export_name =
                        "cabi_post_alibaba:fontkit/fontkit-interface#[method]font-kit.fonts-info"]
                        unsafe extern "C" fn _post_return_method_font_kit_fonts_info(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_font_kit_fonts_info::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.len"] unsafe
                        extern "C" fn export_method_font_kit_len(arg0 : * mut u8,) -> i32
                        { $($path_to_types)*:: _export_method_font_kit_len_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.remove"]
                        unsafe extern "C" fn export_method_font_kit_remove(arg0 : * mut
                        u8, arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 : i32,
                        arg6 : i32, arg7 : * mut u8, arg8 : usize, arg9 : * mut u8, arg10
                        : usize,) { $($path_to_types)*::
                        _export_method_font_kit_remove_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0, arg1, arg2, arg3,
                        arg4, arg5, arg6, arg7, arg8, arg9, arg10) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.measure"]
                        unsafe extern "C" fn export_method_font_kit_measure(arg0 : * mut
                        u8, arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 : i32,
                        arg6 : i32, arg7 : * mut u8, arg8 : usize, arg9 : * mut u8, arg10
                        : usize, arg11 : * mut u8, arg12 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_font_kit_measure_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::FontKit > (arg0, arg1, arg2,
                        arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.write-data"]
                        unsafe extern "C" fn export_method_font_kit_write_data(arg0 : *
                        mut u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_font_kit_write_data_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0) } #[export_name =
                        "cabi_post_alibaba:fontkit/fontkit-interface#[method]font-kit.write-data"]
                        unsafe extern "C" fn _post_return_method_font_kit_write_data(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_font_kit_write_data::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.read-data"]
                        unsafe extern "C" fn export_method_font_kit_read_data(arg0 : *
                        mut u8, arg1 : * mut u8, arg2 : usize,) { $($path_to_types)*::
                        _export_method_font_kit_read_data_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0, arg1, arg2) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#[method]font-kit.buffer-size"]
                        unsafe extern "C" fn export_method_font_kit_buffer_size(arg0 : *
                        mut u8,) -> i32 { $($path_to_types)*::
                        _export_method_font_kit_buffer_size_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FontKit > (arg0) } #[export_name =
                        "alibaba:fontkit/fontkit-interface#str-width-to-number"] unsafe
                        extern "C" fn export_str_width_to_number(arg0 : * mut u8, arg1 :
                        usize,) -> i32 { $($path_to_types)*::
                        _export_str_width_to_number_cabi::<$ty > (arg0, arg1) }
                        #[export_name =
                        "alibaba:fontkit/fontkit-interface#number-width-to-str"] unsafe
                        extern "C" fn export_number_width_to_str(arg0 : i32,) -> * mut u8
                        { $($path_to_types)*:: _export_number_width_to_str_cabi::<$ty >
                        (arg0) } #[export_name =
                        "cabi_post_alibaba:fontkit/fontkit-interface#number-width-to-str"]
                        unsafe extern "C" fn _post_return_number_width_to_str(arg0 : *
                        mut u8,) { $($path_to_types)*::
                        __post_return_number_width_to_str::<$ty > (arg0) } const _ : () =
                        { #[doc(hidden)] #[export_name =
                        "alibaba:fontkit/fontkit-interface#[dtor]text-metrics"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: TextMetrics::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::TextMetrics > (rep) } }; const _ :
                        () = { #[doc(hidden)] #[export_name =
                        "alibaba:fontkit/fontkit-interface#[dtor]glyph-bitmap"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: GlyphBitmap::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::GlyphBitmap > (rep) } }; const _ :
                        () = { #[doc(hidden)] #[export_name =
                        "alibaba:fontkit/fontkit-interface#[dtor]font"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Font::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Font > (rep) } }; const _ : () = {
                        #[doc(hidden)] #[export_name =
                        "alibaba:fontkit/fontkit-interface#[dtor]font-kit"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: FontKit::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::FontKit > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_alibaba_fontkit_fontkit_interface_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 28]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 28],
                );
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::boxed::Box;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }
    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }
    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }
    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub use alloc_crate::alloc;
    pub unsafe fn char_lift(val: u32) -> char {
        if cfg!(debug_assertions) {
            core::char::from_u32(val).unwrap()
        } else {
            core::char::from_u32_unchecked(val)
        }
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_fontkit_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::alibaba::fontkit::fontkit_interface::__export_alibaba_fontkit_fontkit_interface_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::alibaba::fontkit::fontkit_interface);
    };
}
#[doc(inline)]
pub(crate) use __export_fontkit_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:alibaba:fontkit:fontkit:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 2933] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xf7\x15\x01A\x02\x01\
A\x05\x01B\x06\x01k{\x01k\x7f\x01o\x02sv\x01p\x02\x01r\x05\x06weight\0\x06italic\
\x01\x07stretch\0\x06familys\x0avariations\x03\x04\0\x08font-key\x03\0\x04\x03\0\
\x17alibaba:fontkit/commons\x05\0\x02\x03\0\0\x08font-key\x01B\x81\x01\x02\x03\x02\
\x01\x01\x04\0\x08font-key\x03\0\0\x01r\x03\x02id{\x04names\x0blanguage-id{\x04\0\
\x04name\x03\0\x02\x01p\x03\x01ks\x01r\x04\x0bstyle-names\x04\x05names\x04\x04pa\
th\x05\x03key\x01\x04\0\x09font-info\x03\0\x06\x01r\x02\x08position|\x09thicknes\
s|\x04\0\x0cline-metrics\x03\0\x08\x04\0\x0ctext-metrics\x03\x01\x04\0\x0cglyph-\
bitmap\x03\x01\x04\0\x04font\x03\x01\x04\0\x08font-kit\x03\x01\x01i\x0a\x01@\x01\
\x05values\0\x0e\x04\0\x19[constructor]text-metrics\x01\x0f\x01h\x0a\x01@\x01\x04\
self\x10\0\x0e\x04\0\x1e[method]text-metrics.duplicate\x01\x11\x01@\x03\x04self\x10\
\x09font-sizev\x0eletter-spacingv\0v\x04\0\x1a[method]text-metrics.width\x01\x12\
\x01kv\x01@\x03\x04self\x10\x09font-sizev\x0bline-height\x13\0v\x04\0\x1b[method\
]text-metrics.height\x01\x14\x01@\x02\x04self\x10\x09font-sizev\0v\x04\0\x1d[met\
hod]text-metrics.ascender\x01\x15\x01@\x01\x04self\x10\0v\x04\0\x1d[method]text-\
metrics.line-gap\x01\x16\x04\0\x1a[method]text-metrics.units\x01\x16\x01@\x03\x04\
self\x10\x05starty\x05county\0\x0e\x04\0\x1a[method]text-metrics.slice\x01\x17\x01\
@\x01\x04self\x10\0s\x04\0\x1a[method]text-metrics.value\x01\x18\x01@\x01\x04sel\
f\x10\0\x7f\x04\0\x1b[method]text-metrics.is-rtl\x01\x19\x01@\x02\x04self\x10\x05\
other\x0e\x01\0\x04\0\x1b[method]text-metrics.append\x01\x1a\x01@\x01\x04self\x10\
\0y\x04\0\x1a[method]text-metrics.count\x01\x1b\x01@\x03\x04self\x10\x05other\x0e\
\x08fallback\x7f\x01\0\x04\0\x1c[method]text-metrics.replace\x01\x1c\x01@\x04\x04\
self\x10\x09font-sizev\x0eletter-spacingv\x05widthv\0\x0e\x04\0#[method]text-met\
rics.split-by-width\x01\x1d\x01pt\x01@\x01\x04self\x10\0\x1e\x04\0\x1a[method]te\
xt-metrics.chars\x01\x1f\x01h\x0b\x01@\x01\x04self\x20\0y\x04\0\x1a[method]glyph\
-bitmap.width\x01!\x04\0\x1b[method]glyph-bitmap.height\x01!\x01p}\x01@\x01\x04s\
elf\x20\0\"\x04\0\x1b[method]glyph-bitmap.bitmap\x01#\x01@\x01\x04self\x20\0v\x04\
\0\x1a[method]glyph-bitmap.x-min\x01$\x04\0\x1a[method]glyph-bitmap.y-max\x01$\x04\
\0\x1d[method]glyph-bitmap.stroke-x\x01$\x04\0\x1d[method]glyph-bitmap.stroke-y\x01\
$\x01o\x02\"y\x01k%\x01@\x01\x04self\x20\0&\x04\0\"[method]glyph-bitmap.stroke-b\
itmap\x01'\x04\0\x1f[method]glyph-bitmap.advanced-x\x01$\x04\0\x1d[method]glyph-\
bitmap.ascender\x01$\x04\0\x1e[method]glyph-bitmap.descender\x01$\x01h\x0c\x01@\x02\
\x04self(\x01ct\0\x7f\x04\0\x16[method]font.has-glyph\x01)\x01@\x02\x04self(\x01\
ct\0\x05\x04\0\x1e[method]font.glyph-path-string\x01*\x01@\x01\x04self(\0\"\x04\0\
\x13[method]font.buffer\x01+\x01@\x01\x04self(\0s\x04\0\x11[method]font.path\x01\
,\x01@\x01\x04self(\0\x01\x04\0\x10[method]font.key\x01-\x01j\x01\x0e\x01s\x01@\x02\
\x04self(\x04texts\0.\x04\0\x14[method]font.measure\x01/\x01@\x01\x04self(\0|\x04\
\0\x15[method]font.ascender\x010\x04\0\x16[method]font.descender\x010\x01@\x01\x04\
self(\0{\x04\0\x19[method]font.units-per-em\x011\x01i\x0b\x01k2\x01@\x04\x04self\
(\x01ct\x09font-sizev\x0cstroke-widthv\03\x04\0\x13[method]font.bitmap\x014\x01k\
\x09\x01@\x01\x04self(\05\x04\0\x1e[method]font.underline-metrics\x016\x01i\x0d\x01\
@\0\07\x04\0\x15[constructor]font-kit\x018\x01h\x0d\x01@\x02\x04self9\x05limity\x01\
\0\x04\0\x1e[method]font-kit.set-lru-limit\x01:\x01@\x02\x04self9\x06buffer\"\x01\
\0\x04\0%[method]font-kit.add-font-from-buffer\x01;\x01@\x02\x04self9\x04paths\x01\
\0\x04\0\x20[method]font-kit.add-search-path\x01<\x01i\x0c\x01k=\x01@\x02\x04sel\
f9\x03key\x01\0>\x04\0\x16[method]font-kit.query\x01?\x01p\x07\x01k\xc0\0\x01@\x02\
\x04self9\x03key\x01\0\xc1\0\x04\0\x20[method]font-kit.query-font-info\x01B\x04\0\
\x1c[method]font-kit.exact-match\x01?\x01@\x01\x04self9\0\xc0\0\x04\0\x1b[method\
]font-kit.fonts-info\x01C\x01@\x01\x04self9\0y\x04\0\x14[method]font-kit.len\x01\
D\x01@\x02\x04self9\x03key\x01\x01\0\x04\0\x17[method]font-kit.remove\x01E\x01k\x0e\
\x01@\x03\x04self9\x03key\x01\x04texts\0\xc6\0\x04\0\x18[method]font-kit.measure\
\x01G\x01@\x01\x04self9\0s\x04\0\x1b[method]font-kit.write-data\x01H\x01@\x02\x04\
self9\x04datas\x01\0\x04\0\x1a[method]font-kit.read-data\x01I\x04\0\x1c[method]f\
ont-kit.buffer-size\x01D\x01@\x01\x05widths\0{\x04\0\x13str-width-to-number\x01J\
\x01@\x01\x05width{\0s\x04\0\x13number-width-to-str\x01K\x04\0!alibaba:fontkit/f\
ontkit-interface\x05\x02\x04\0\x17alibaba:fontkit/fontkit\x04\0\x0b\x0d\x01\0\x07\
fontkit\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.22\
0.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
