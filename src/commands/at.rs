use super::{ Command, BufferSizeError };

const AT_COMMAND_ID: u8 = 0x08;

pub struct AtCommand {
  frame_id: u8,
  at_command: [u8; 2],
  parameter_value: Option<u8>,
}

impl AtCommand {
  pub fn new(frame_id: u8, at_command: [u8; 2], parameter_value: Option<u8>) -> Self {
    AtCommand { frame_id, at_command, parameter_value }
  }
}

impl Command for AtCommand {
  fn fill_buffer<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], BufferSizeError> {
    if buffer.len() < 4 { return Err(BufferSizeError); }

    buffer[0] = AT_COMMAND_ID;
    buffer[1] = self.frame_id;
    buffer[2] = self.at_command[0];
    buffer[3] = self.at_command[1];

    match self.parameter_value {
      Some(value) => {
        if buffer.len() < 5 { return Err(BufferSizeError); }

        buffer[4] = value;
        Ok(&buffer[0..5])
      },
      _ => Ok(&buffer[0..4])
    }
  }
}

#[test]
fn test_empty_at_command() {
  let mut buffer: [u8; 5] = unsafe { core::mem::zeroed() };
  let at_command = AtCommand::new(0x52, [b'N', b'J'], None);

  let command = at_command.fill_buffer(&mut buffer[..]);
  assert_eq!(command.unwrap(), [0x08, 0x52, b'N', b'J']);
}

#[test]
fn test_filled_at_command() {
  let mut buffer: [u8; 5] = unsafe { core::mem::zeroed() };
  let at_command = AtCommand::new(0x52, [b'N', b'J'], Some(b'K'));

  let command = at_command.fill_buffer(&mut buffer[..]);
  assert_eq!(command.unwrap(), [0x08, 0x52, b'N', b'J', b'K']);
}
