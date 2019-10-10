pub mod at;

#[derive(Debug, PartialEq)]
pub enum ResponseError {
  IdError,
  SizeIncorrectError,
  EnumComparisonError,
}

pub trait Response<T> {
  fn respond_to(id: u8) -> bool;
  fn parse<'a>(buffer: &'a [u8]) -> Result<T, ResponseError>;
}
