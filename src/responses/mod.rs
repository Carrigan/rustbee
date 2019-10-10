pub mod at;
pub mod receive_packet;

#[derive(Debug, PartialEq)]
pub enum ResponseError {
  IdError,
  SizeIncorrectError,
  EnumComparisonError,
}

pub trait Response<'a, T> {
  fn respond_to(id: u8) -> bool;
  fn parse(buffer: &'a [u8]) -> Result<T, ResponseError>;
}
