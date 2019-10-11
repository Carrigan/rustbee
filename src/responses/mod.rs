pub mod at_command_response;
pub mod receive_packet;

pub use receive_packet::ZigbeeReceivePacket;
pub use at_command_response::{ AtCommandResponse, AtCommandResponseStatus };

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
