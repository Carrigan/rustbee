pub mod at;

#[derive(Debug)]
pub struct BufferSizeError;

pub trait Command {
  fn fill_buffer<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], BufferSizeError>;
}
