// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Converter(Interface<gio_sys::GConverter>);

    match fn {
        get_type => || gio_sys::g_converter_get_type(),
    }
}

pub const NONE_CONVERTER: Option<&Converter> = None;

pub trait ConverterExt: 'static {
    fn reset(&self);
}

impl<O: IsA<Converter>> ConverterExt for O {
    fn reset(&self) {
        unsafe {
            gio_sys::g_converter_reset(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for Converter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Converter")
    }
}