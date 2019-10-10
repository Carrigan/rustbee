use super::{ Response, ResponseError };
use core::convert::TryFrom;

const COMMAND_ID: u8 = 0x88;

pub enum AtCommandResponseStatus {
  Ok = 0,
  Error = 1,
  InvalidCommand = 2,
  InvalidParameter = 3,
  TxFailure = 4
}

impl TryFrom<u8> for AtCommandResponseStatus {
  type Error = ResponseError;

  fn try_from(value: u8) -> Result<Self, ResponseError> {
    match value {
      0 => Ok(AtCommandResponseStatus::Ok),
      1 => Ok(AtCommandResponseStatus::Error),
      2 => Ok(AtCommandResponseStatus::InvalidCommand),
      3 => Ok(AtCommandResponseStatus::InvalidParameter),
      4 => Ok(AtCommandResponseStatus::TxFailure),
      _ => Err(ResponseError::EnumComparisonError)
    }
  }
}

pub struct AtCommandResponse {
  frame_id: u8,
  at_command: [u8; 2],
  command_status: AtCommandResponseStatus,
  command_data: Option<u8>,
}

impl Response<AtCommandResponse> for AtCommandResponse {
  fn respond_to(id: u8) -> bool {
    COMMAND_ID == id
  }

  fn parse<'a>(buffer: &'a [u8]) -> Result<Self, ResponseError> {
    if buffer.len() < 5 || buffer.len() > 6 { return Err(ResponseError::SizeIncorrectError); }
    if buffer[0] != COMMAND_ID { return Err(ResponseError::IdError); }

    let frame_id = buffer[1];
    let at_command = [buffer[2], buffer[3]];

    let command_status = match AtCommandResponseStatus::try_from(buffer[4]) {
      Ok(status) => status,
      Err(error) => return Err(error)
    };

    let command_data = match buffer.len() {
      6 => Some(buffer[5]),
      _ => None
    };

    Ok(Self { frame_id, at_command, command_status, command_data })
  }
}

#[test]
fn test_at_command_response_parse() {
  let buffer: [u8; 5] = [0x88, 0x01, 0x42, 0x44, 0x00];
  let response = AtCommandResponse::parse(&buffer[..]).unwrap();

  assert_eq!(response.frame_id, 1);
  assert_eq!(response.at_command, [b'B', b'D']);
  assert_eq!(response.command_status as u8, AtCommandResponseStatus::Ok as u8);
  assert_eq!(response.command_data, None);
}

#[test]
fn test_bad_enum() {
  let buffer: [u8; 5] = [0x88, 0x01, 0x42, 0x44, 0x10];
  let response = AtCommandResponse::parse(&buffer[..]);

  match response {
    Ok(_) => assert!(false),
    Err(err) => assert_eq!(err, ResponseError::EnumComparisonError)
  };
}

#[test]
fn test_bad_size() {
  let buffer: [u8; 4] = [0x88, 0x01, 0x42, 0x44];
  let response = AtCommandResponse::parse(&buffer[..]);

  match response {
    Ok(_) => assert!(false),
    Err(err) => assert_eq!(err, ResponseError::SizeIncorrectError)
  };
}
