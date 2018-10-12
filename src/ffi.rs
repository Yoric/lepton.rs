use std::os::raw::c_char;

use libc::c_void;


#[repr(C)]
pub struct PixColormap {
    pub array: *mut c_void,
    pub depth: i32,
    pub nalloc: i32,
    pub n: i32
}

#[repr(C)]
pub struct Pix {
    pub w: u32,
    pub h: u32,
    pub d: u32,
    pub wpl: u32,
    pub refcount: u32,
    pub xres: i32,
    pub yres: i32,
    pub informat: Format,
    pub text: *mut c_char,
    pub colormap: *mut PixColormap,
    pub data: *mut u32,
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(i32)]
#[derive(Clone, Copy)]
pub enum Format {
  Unknown = 0,
  BMP = 1,
  JFIF_JPEG = 2,
  PNG = 3,
  TIFF = 4,
  TIFF_PACKBITS = 5,
  TIFF_RLE = 6,
  TIFF_G3 = 7,
  TIFF_G4 = 8,
  TIFF_LZW = 9,
  TIFF_ZIP = 10,
  PNM = 11,
  PS = 12,
  GIF = 13,
  JP2 = 14,
  WEBP = 15,
  LPDF = 16,
  DEFAULT = 17,
  SPIX = 18
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(C)]
pub enum Background {
    WHITE = 1,        /* bring in white pixels from the outside */
    BLACK = 2         /* bring in black pixels from the outside */
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(C)]
pub enum Rotation {
    AREA_MAP = 1,       /* use area map rotation, if possible     */
    SHEAR = 2,          /* use shear rotation                     */
    SAMPLING = 3        /* use sampling                           */
}

extern {
//    pub fn pixFree(pix: *mut Pix);
    pub fn pixDestroy(pix: *mut *mut Pix); // I'd like to use pixFree, but it's apparently not exported in 1.7.3.
    pub fn pixRead(filename: *const c_char) -> *mut Pix;
    pub fn pixWrite(filename: *const c_char, pix: *mut Pix, format: Format) -> i32;
//    pub fn pixDeskewGeneral(pix: *mut Pix, redsweep: i32, sweeprange: f32, sweepdelta: f32, redsearch: i32,
//        thresh: i32, angle: *mut f32, conf: *mut f32) -> *mut Pix;
    pub fn pixFindSkew(pix: *mut Pix, angle: *mut f32, conf: *mut f32) -> u32;
    //pub fn pixRotateBinaryNice(pix: *mut Pix, angle: f32, incolor: BringInColor) -> *mut Pix;
    pub fn pixConvertTo1(pix: *mut Pix, threshold: i32) -> *mut Pix;
    pub fn pixRotate(pix: *mut Pix, angle: f32, type_: Rotation, incolor: Background, width: i32, height: i32) -> *mut Pix;
}