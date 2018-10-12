use ffi;
use pix::*;

pub struct Skew {
    pub degrees: f32,
    pub confidence: f32
}

pub trait TSkew {
    fn find_skew(&mut self) -> Option<Skew>;
}

impl TSkew for Pix {
    fn find_skew(&mut self) -> Option<Skew> {
        let mut angle = 0.0;
        let mut confidence = 0.0;
        match unsafe { ffi::pixFindSkew(as_c(self), &mut angle, &mut confidence) } {
            0 /* ok */ => Some(Skew {
                degrees: angle,
                confidence: confidence
            }),
            1 /* error */ => None,
            _ => panic!("Unexpected result")
        }
    }
}

//pub fn pixFindSkew(pix: *mut Pix, angle: *mut f32, conf: *mut f32) -> u32;
