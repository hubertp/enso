use crate::traits::*;
use enso_prelude::*;

use crate::Result;



// ===================
// === MockDefault ===
// ===================

/// Default value provider. Similar to [`Default`] but with additional implementations.
pub trait MockDefault {
    fn mock_default() -> Self;
}

/// [`MockDefault::mock_default`] accessor.
pub fn mock_default<T: MockDefault>() -> T {
    T::mock_default()
}

impl MockDefault for () {
    fn mock_default() -> Self {}
}

impl<T: MockDefault, E> MockDefault for std::result::Result<T, E> {
    fn mock_default() -> Self {
        Ok(mock_default())
    }
}

/// Macro which generates [`MockDefault`] impls which redirect the call to [`Default::default`].
macro_rules! auto_impl_mock_default {
    ( $($tp:ident $(< $($arg:ident),* >)? ),* ) => {
        $(
            impl $(<$($arg),*>)? MockDefault for $tp $(<$($arg),*>)? {
                fn mock_default() -> Self {
                    default()
                }
            }
        )*
    };
}

auto_impl_mock_default!(bool, i16, i32, u32, f64, String, Option<T>);



// ================
// === MockData ===
// ================

/// Every mock structure implements this trait.
pub trait MockData {}

/// Macro used to generate mock structures. See the expansion of generated structures to learn more.
macro_rules! mock_struct {
    ( $([$opt:ident])?
        $name:ident $(<$( $param:ident $(: ?$param_tp:ident)? ),*>)? $(=> $deref:ident)?
    ) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_snake_case)]
        pub struct $name $(<$($param $(:?$param_tp)?),*>)? {
            $($( $param : PhantomData<$param> ),*)?
        }

        /// # Safety
        /// The usage of [`mem::transmute`] is safe here as we transmute ZST types.
        impl$(<$($param $(:?$param_tp)?),*>)?
        $name $(<$($param),*>)? {
            pub const fn const_new() -> Self {
                unsafe { mem::transmute(()) }
            }
        }

        impl$(<$($param $(:?$param_tp)?),*>)?
        Debug for $name $(<$($param),*>)? {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, stringify!($name))
            }
        }

        #[allow(unsafe_code)]
        impl $(<$($param $(:?$param_tp)?),*>)?
        Default for $name $(<$($param),*>)? {
            fn default() -> Self {
                Self::const_new()
            }
        }

        impl $(<$($param $(:?$param_tp)?),*>)?
        MockDefault for $name $(<$($param),*>)? {
            fn mock_default() -> Self {
                default()
            }
        }

        impl $(<$($param $(:?$param_tp)?),*>)?
        Clone for $name $(<$($param),*>)? {
            fn clone(&self) -> Self {
                default()
            }
        }

        impl $(<$($param $(:?$param_tp)?),*>)?
        CloneRef for $name $(<$($param),*>)? {
            fn clone_ref(&self) -> Self {
                default()
            }
        }

        impl $(<$($param $(:?$param_tp)?),*>)?
        MockData for $name $(<$($param),*>)? {}

        mock_struct_deref! {[$($deref)?] $name $(<$( $param $(:?$param_tp)?),*>)?}
        mock_struct_as_ref! {[$($opt)?] $name $(<$( $param $(:?$param_tp)?),*>)? $(=> $deref)?}
    };
}

macro_rules! mock_struct_as_ref {
    ([NO_AS_REF] $($ts:tt)*) => {};
    ([] $name:ident $(<$( $param:ident $(: ?$param_tp:ident)? ),*>)?
        $(=> $deref:ident)?
    ) => {
        /// # Safety
        /// The usage of [`mem::transmute`] is safe here as we transmute ZST types.
        #[allow(unsafe_code)]
        impl<__T__: MockData, $($($param $(:?$param_tp)? ),*)?>
        AsRef<__T__> for $name $(<$($param),*>)? {
            fn as_ref(&self) -> &__T__ {
                unsafe { mem::transmute(self) }
            }
        }
    };
}

macro_rules! mock_struct_deref {
    ([] $($ts:tt)*) => {};
    ([$deref:ident] $name:ident $(<$( $param:ident $(: ?$param_tp:ident)? ),*>)?) => {
        impl $(<$($param $(:?$param_tp)?),*>)?
        Deref for $name $(<$($param),*>)? {
            type Target = $deref;
            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }

        impl $(<$($param $(:?$param_tp)?),*>)?
        From<$name $(<$($param),*>)?> for $deref {
            fn from(_: $name) -> Self {
                default()
            }
        }
    };
}



// ===============
// === mock_pub_fn ===
// ===============

macro_rules! mock_fn {
    ( $($ts:tt)* ) => {
        mock_fn_gen! {[] $($ts)*}
    };
}

macro_rules! mock_pub_fn {
    ( $($ts:tt)* ) => {
        mock_fn_gen! {[pub] $($ts)*}
    };
}

/// Macro used to generate mock methods. Methods look just like their provided signature with a body
/// returning `mock_default()`. There are two special cases: for functions returning `&Self`, and
/// `&mut Self`, which just pass `&self` and `&mut self` to the output, respectively.
macro_rules! mock_fn_gen {
    ([$($viz:ident)?] $name:ident $(<$($fn_tp:ident),*>)?
    (&self $(,$arg:ident : $arg_tp:ty)* $(,)? ) -> &Self ) => {
        $($viz)? fn $name $(<$($fn_tp),*>)? (&self $(,$arg : $arg_tp)*) -> &Self {
            self
        }
    };

    ([$($viz:ident)?] $name:ident $(<$($fn_tp:ident),*>)?
    (&mut self $(,$arg:ident : $arg_tp:ty)* $(,)? ) -> &mut Self ) => {
        $($viz)? fn $name $(<$($fn_tp),*>)? (&mut self $(,$arg : $arg_tp)*) -> &mut Self {
            self
        }
    };

    ([$($viz:ident)?] $name:ident $(<$($fn_tp:ident),*>)?
    (&self $(,$arg:ident : $arg_tp:ty)* $(,)? ) -> &$out:ty ) => {
        $($viz)? fn $name $(<$($fn_tp),*>)? (&self $(,$arg : $arg_tp)*) -> &$out {
            self.as_ref()
        }
    };

    ([$($viz:ident)?] $name:ident $(<$($fn_tp:ident),*>)?
    (&self $(,$arg:ident : $arg_tp:ty)* $(,)? ) $(-> $out:ty)? ) => {
        $($viz)? fn $name $(<$($fn_tp),*>)? (&self $(,$arg : $arg_tp)*) $(-> $out)? {
            mock_default()
        }
    };

    ([$($viz:ident)?] $name:ident $(<$($fn_tp:ident),*>)?
    ($($arg:ident : $arg_tp:ty)* $(,)? ) $(-> $out:ty)? ) => {
        $($viz)? fn $name $(<$($fn_tp),*>)? ($($arg : $arg_tp)*) $(-> $out)? {
            mock_default()
        }
    };
}

/// Combination of [`mock_struct`] and [`mock_pub_fn`].
macro_rules! mock_data {
    ( $([$opt:ident])?
        $name:ident $(<$( $param:ident $(: ?$param_tp:ident)? ),*>)? $(=> $deref:ident)?
        $(
            fn $fn_name:ident $(<$($fn_tp:ident),*>)? ($($args:tt)*) $(-> $out:ty)?;
        )*
    ) => {
        mock_struct!{$([$opt])? $name $(<$($param $(:?$param_tp)?),*>)? $(=> $deref)?}
        impl $(<$($param $(:?$param_tp)?),*>)? $name $(<$($param),*>)? {
            $(
                mock_pub_fn!{$fn_name $(<$($fn_tp),*>)? ($($args)*) $(-> $out)?}
            )*
        }
    };
}



// ==============
// === JsCast ===
// ==============

/// Mock of [`JsCast`] is implemented for all mocked types.
impl<T: MockData + MockDefault + AsRef<JsValue> + Into<JsValue>> JsCast for T {}

/// Mock of [`wasm_bindgen::JsCast`].
pub trait JsCast
where Self: MockData + MockDefault + AsRef<JsValue> + Into<JsValue> {
    fn has_type<T>(&self) -> bool {
        true
    }

    fn dyn_into<T>(self) -> std::result::Result<T, Self>
    where T: JsCast {
        Ok(self.unchecked_into())
    }

    fn dyn_ref<T>(&self) -> Option<&T>
    where T: JsCast {
        Some(self.unchecked_ref())
    }

    fn unchecked_into<T>(self) -> T
    where T: JsCast {
        T::unchecked_from_js(self.into())
    }

    fn unchecked_ref<T>(&self) -> &T
    where T: JsCast {
        T::unchecked_from_js_ref(self.as_ref())
    }

    fn is_instance_of<T>(&self) -> bool {
        true
    }
    fn instanceof(_val: &JsValue) -> bool {
        true
    }
    fn is_type_of(_val: &JsValue) -> bool {
        true
    }
    fn unchecked_from_js(_val: JsValue) -> Self {
        mock_default()
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        val.as_ref()
    }
}


// ===============
// === JsValue ===
// ===============

/// Mock of [`wasm_bindgen::JsValue`]. All JS types can be converted to `JsValue` and thus it
/// implements a generic conversion trait.
mock_data! { JsValue }

auto trait IsNotJsValue {}
impl !IsNotJsValue for JsValue {}
impl<A: IsNotJsValue> From<A> for JsValue {
    default fn from(_: A) -> Self {
        default()
    }
}



// ===============
// === Closure ===
// ===============

/// The generated structure does not implement a generic [`AsRef`] impl, as the usages base on the
/// fact that there exist exactly one such an impl (provided below), so the type inferencer can
/// monomoprhise more free variables.
mock_data! { [NO_AS_REF] Closure<T: ?Sized>
    fn new<F>(_t: F) -> Closure<T>;
    fn wrap(_data: Box<T>) -> Closure<T>;
    fn once<F>(_fn_once: F) -> Closure<F>;
}

#[allow(unsafe_code)]
impl<T: ?Sized> AsRef<JsValue> for Closure<T> {
    fn as_ref(&self) -> &JsValue {
        unsafe { mem::transmute(self) }
    }
}



// ====================
// === DOM Elements ===
// ====================

// === WebGl2RenderingContext ===
/// The [`WebGl2RenderingContext`] is not a mocked structure because it defines tons of
/// constants that we use heavily. Instead, the rendering engine runs context-less when
/// compiled to native tests.
pub use web_sys::WebGl2RenderingContext;

// === Object ===
mock_data! { Object => JsValue
    fn value_of(&self) -> Object;
}


// === EventTarget ===
mock_data! { EventTarget => Object
    fn remove_event_listener_with_callback
        (&self, _tp:&str, _f:&Function) -> std::result::Result<(), JsValue>;
    fn add_event_listener_with_callback
        (&self, _tp:&str, _f:&Function) -> std::result::Result<(), JsValue>;
    fn add_event_listener_with_callback_and_bool
        (&self, _tp:&str, _f:&Function, _opt:bool) -> std::result::Result<(), JsValue>;
    fn add_event_listener_with_callback_and_add_event_listener_options
        (&self, _tp:&str, _f:&Function, _opt:&AddEventListenerOptions)
        -> std::result::Result<(), JsValue>;
}


// === Document ===
mock_data! { Document => EventTarget }


// === Window ===
mock_data! { Window => EventTarget
    fn open_with_url_and_target(&self,_url: &str,_target: &str)
        -> std::result::Result<Option<Window>, JsValue>;
}


// === Function ===
mock_data! { Function }


// === AddEventListenerOptions ===
mock_data! { AddEventListenerOptions
    fn new() -> Self;
}
impl AddEventListenerOptions {
    mock_pub_fn!(capture(&mut self, _val:bool) -> &mut Self);
    mock_pub_fn!(passive(&mut self, _val:bool) -> &mut Self);
}


// === Event ===
mock_data! { Event => Object
    fn prevent_default(&self);
    fn stop_propagation(&self);
    fn current_target(&self) -> Option<EventTarget>;
}


// === KeyboardEvent ===
mock_data! { KeyboardEvent => Event
    fn key(&self) -> String;
    fn code(&self) -> String;
    fn alt_key(&self) -> bool;
    fn ctrl_key(&self) -> bool;
}


// === MouseEvent ===
mock_data! { MouseEvent => Event
    fn button(&self) -> i16;
    fn alt_key(&self) -> bool;
    fn ctrl_key(&self) -> bool;
    fn client_x(&self) -> i32;
    fn client_y(&self) -> i32;
    fn offset_x(&self) -> i32;
    fn offset_y(&self) -> i32;
    fn screen_x(&self) -> i32;
    fn screen_y(&self) -> i32;
}


// === WheelEvent ===
mock_data! { WheelEvent => MouseEvent
    fn delta_x(&self) -> f64;
    fn delta_y(&self) -> f64;
}


// === HtmlCollection ===
mock_data! { HtmlCollection
    fn length(&self) -> u32;
}


// === DomRect ===
mock_data! { DomRect
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    fn left(&self) -> f64;
    fn right(&self) -> f64;
    fn top(&self) -> f64;
    fn bottom(&self) -> f64;
}


// === Element ===
mock_data! { Element => Node
    fn remove(&self);
    fn children(&self) -> HtmlCollection;
    fn get_bounding_client_rect(&self) -> DomRect;
}

// === HtmlElement ===
mock_data! { HtmlElement => Element
    fn set_class_name(&self, _n: &str);
}
impl From<HtmlElement> for EventTarget {
    fn from(_: HtmlElement) -> Self {
        default()
    }
}


// === HtmlDivElement ===
mock_data! { HtmlDivElement => HtmlElement }
impl From<HtmlDivElement> for EventTarget {
    fn from(_: HtmlDivElement) -> Self {
        default()
    }
}


// === HtmlCanvasElement ===
mock_data! { HtmlCanvasElement => HtmlElement }


// === CanvasRenderingContext2d ===
mock_data! { CanvasRenderingContext2d }


// === Node ===
mock_data! { Node => EventTarget }



// =============
// === Utils ===
// =============

pub static document: Document = Document::const_new();
pub static window: Window = Window {};

impl WindowApi for Window {
    mock_fn! { forward_panic_hook_to_console(&self) }
}

impl DocumentApi for Document {
    mock_fn! { body(&self) -> &HtmlElement }
    mock_fn! { create_div(&self) -> HtmlDivElement }
    mock_fn! { create_canvas(&self) -> HtmlCanvasElement }
    mock_fn! { get_element_by_id(&self, _id: &str) -> Result<HtmlElement> }
    mock_fn! { get_webgl2_context(&self, _canvas: &HtmlCanvasElement)
    -> Option<WebGl2RenderingContext> }
}

// mock_pub_fn! { body() -> HtmlElement }
// mock_pub_fn! { create_div() -> HtmlDivElement }
// mock_pub_fn! { create_canvas() -> HtmlCanvasElement }
// mock_pub_fn! { get_html_element_by_id(_id: &str) -> Result<HtmlElement> }
// mock_pub_fn! { get_webgl2_context(_canvas: &HtmlCanvasElement) -> Option<WebGl2RenderingContext>
// }
// mock_pub_fn! { forward_panic_hook_to_console() }

pub trait Test {
    fn test() -> String;
}
