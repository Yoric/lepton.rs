use ffi;
use error::*;

use std::ffi::CString;
use std::path::Path;

/// In-memory representation of an image.
pub struct Pix(*mut ffi::Pix); // Invariant: The pointer is non-null.

pub fn as_c(pix: &mut Pix) -> *mut ffi::Pix {
    pix.0
}
pub fn from_c(pix: *mut ffi::Pix) -> Result<Pix, Error> {
    Pix::import(pix)
}

impl Pix {
    fn import(ptr: *mut ffi::Pix) -> Result<Pix, Error> {
        if ptr.is_null() {
            unimplemented!() // FIXME: Better error reporting.
        } else {
            Ok(Pix(ptr))
        }
    }
}

impl Drop for Pix {
    fn drop(&mut self) {
        unsafe {
            let mut ptr = &mut self.0;
            ffi::pixDestroy(ptr)
        }
    }
}

/// The format of an image.
pub use ffi::Format;

impl Pix {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Pix, Error> {
        let path : &str = path.as_ref().to_str().unwrap(); // FIXME: Better error reporting.
        let c_path = CString::new(path).unwrap().as_ptr(); // FIXME: Better error reporting.
        let pix = unsafe {
            ffi::pixRead(c_path)
        };
        Self::import(pix)
    }

    pub fn write<P: AsRef<Path>>(&mut self, path: P, format: Format) -> Result<(), Error> {
        let path : &str = path.as_ref().to_str().unwrap(); // FIXME: Better error reporting.
        let c_path = CString::new(path).unwrap().as_ptr(); // FIXME: Better error reporting.
        if unsafe { ffi::pixWrite(c_path, self.0, format) } == 0 {
            Ok(())
        } else {
            unimplemented!() // FIXME: Better error reporting.
        }
    }
}

impl Pix {
    /// The width of the image, in pixels.
    pub fn width(&self) -> u32 {
        unsafe { (*self.0).w }
    }

    /// The height of the image, in pixels.
    pub fn height(&self) -> u32 {
        unsafe { (*self.0).h }
    }

    /// The format of the image, if known.
    pub fn format(&self) -> Format {
        unsafe { (*self.0).informat }
    }
}

pub enum Conversion {
    OneBit { threshold: i32, factor: Option<i32> },
}

impl Pix {
    pub fn convert(&self, conversion: Conversion) -> Result<Pix, Error> {
        use self::Conversion::*;
        let pix = match conversion {
            OneBit { threshold, factor: None } => unsafe {
                ffi::pixConvertTo1(self.0, threshold)
            },
            _ => unimplemented!()
        };
        Self::import(pix)
    }
}