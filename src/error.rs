#[derive(Debug)]
pub struct Error { // FIXME: Wait, that's useless. leptonica actually doesn't return errors...
  pub message: String,
  pub source: String,
  pub detail: Detail,
}

#[derive(Debug)]
pub enum Detail {
    I32(i32)
}