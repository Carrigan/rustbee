use super::{ Command, BufferSizeError };

const COMMAND_ID: u8 = 0x10;
const BROADCAST_DESTINATION: u64 =   0x0000_0000_0000_FFFF;
const COORDINATOR_DESTINATION: u64 = 0x0000_0000_0000_0000;
const BROADCAST_NETWORK_ADDRESS: u16 = 0xFFFE;
const MAXIMUM_HOPS: u8 = 0x00;

pub struct TransmitRequestCommand<'a> {
  frame_id: u8,
  destination: u64,
  network_address: u16,
  radius: u8,
  disable_retries: Option<bool>,
  enable_encryption: Option<bool>,
  use_extended_timeout: Option<bool>,
  data: &'a [u8]
}

impl <'a> Default for TransmitRequestCommand<'a> {
  fn default() -> Self {
    Self {
      frame_id: 0x00,
      destination: BROADCAST_DESTINATION,
      network_address: BROADCAST_NETWORK_ADDRESS,
      radius: MAXIMUM_HOPS,
      disable_retries: None,
      enable_encryption: None,
      use_extended_timeout: None,
      data: &[]
    }
  }
}

impl <'a> TransmitRequestCommand <'a> {
  pub fn broadcast(frame_id: u8, data: &'a [u8]) -> Self {
    Self { frame_id, data, ..Default::default() }
  }

  pub fn to_destination(frame_id: u8, destination: u64, data: &'a [u8]) -> Self {
    Self { frame_id, destination, data, ..Default::default() }
  }

  pub fn to_coordinator(frame_id: u8, data: &'a [u8]) -> Self {
    Self { frame_id, destination: COORDINATOR_DESTINATION, data, ..Default::default() }
  }
}

impl <'a> Command for TransmitRequestCommand <'a> {
  fn fill_buffer<'b>(&self, buffer: &'b mut [u8]) -> Result<&'b [u8], BufferSizeError> {
    if buffer.len() < 14 + self.data.len() { return Err(BufferSizeError); }

    buffer[0] = COMMAND_ID;
    buffer[1] = self.frame_id;

    // Fill the destination address, MSB first
    buffer[2] = ((self.destination >> 56) & 0xFF) as u8;
    buffer[3] = ((self.destination >> 48) & 0xFF) as u8;
    buffer[4] = ((self.destination >> 40) & 0xFF) as u8;
    buffer[5] = ((self.destination >> 32) & 0xFF) as u8;
    buffer[6] = ((self.destination >> 24) & 0xFF) as u8;
    buffer[7] = ((self.destination >> 16) & 0xFF) as u8;
    buffer[8] = ((self.destination >> 8) & 0xFF) as u8;
    buffer[9] = (self.destination & 0xFF) as u8;

    // Fill the network address, MSB first
    buffer[10] = (self.network_address / 256) as u8;
    buffer[11] = (self.network_address & 0xFF) as u8;

    // Number of hops
    buffer[12] = self.radius;

    // The options byte
    buffer[13] = 0x00;
    if let Some(option) = self.disable_retries {
      if option { buffer[13] |= 0x01; }
    }

    if let Some(option) = self.enable_encryption {
      if option { buffer[13] |= 0x20; }
    }

    if let Some(option) = self.use_extended_timeout {
      if option { buffer[13] |= 0x40; }
    }

    // Fill in data
    for i in 0..self.data.len() { buffer[14 + i] = self.data[i]; }

    // Return the data
    Ok(&buffer[0..14 + self.data.len()])
  }
}

#[test]
fn test_transmit_request_success() {
  let mut buffer: [u8; 22] = unsafe { core::mem::zeroed() };
  let frame_id = 1;
  let data: [u8; 8] = [0x54, 0x78, 0x44, 0x61, 0x74, 0x61, 0x30, 0x41];

  let request = TransmitRequestCommand::to_destination(
    frame_id, 0x0013_A200_400A_0127, &data[..]
  );

  let command = request.fill_buffer(&mut buffer[..]);
  assert_eq!(
    command.unwrap(),
    [
      0x10,
      0x01,
      0x00, 0x13, 0xA2, 0x00, 0x40, 0x0A, 0x01, 0x27,
      0xFF, 0xFE,
      0x00,
      0x00,
      0x54, 0x78, 0x44, 0x61, 0x74, 0x61, 0x30, 0x41
    ]
  );
}

#[test]
fn test_transmit_request_failure() {
  let mut buffer: [u8; 21] = unsafe { core::mem::zeroed() };
  let frame_id = 1;
  let data: [u8; 8] = [0x54, 0x78, 0x44, 0x61, 0x74, 0x61, 0x30, 0x41];

  let request = TransmitRequestCommand::to_destination(
    frame_id, 0x0013_A200_400A_0127, &data[..]
  );

  let command = request.fill_buffer(&mut buffer[..]);
  assert!(command.is_err());
}
