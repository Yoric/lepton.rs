use error::Error;
use ffi;
use pix::*;

pub use ffi::Rotation;
pub use ffi::Background;

pub trait TRotate {
    fn rotate(&mut self, angle: f32, kind: Rotation, background: Background, embed: Option<(i32, i32)>) -> Result<Pix, Error>;
}

impl TRotate for Pix {
    fn rotate(&mut self, angle: f32, kind: Rotation, background: Background, embed: Option<(i32, i32)>) -> Result<Pix, Error> {
        let (width, height) = match embed {
            None => (0, 0),
            Some(wh) => wh
        };
        let pix = unsafe { ffi::pixRotate(as_c(self), angle, kind, background, width, height )};
        from_c(pix)
    }
}

//pub fn pixFindSkew(pix: *mut Pix, angle: *mut f32, conf: *mut f32) -> u32;
