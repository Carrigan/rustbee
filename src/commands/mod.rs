pub mod at;
pub mod transmit_request;

pub use transmit_request::TransmitRequestCommand;

#[derive(Debug)]
pub struct BufferSizeError;

pub trait Command {
  fn fill_buffer<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], BufferSizeError>;
}
