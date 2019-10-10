use super::{ Response, ResponseError };

const COMMAND_ID: u8 = 0x90;

pub struct ZigbeeReceivePacket<'a> {
  destination: u64,
  network_address: u16,
  packet_acknowledged: bool,
  packet_broadcast: bool,
  packet_encrypted: bool,
  packet_end_device: bool,
  data: &'a [u8]
}

fn buffer_to_u64(buffer: &[u8]) -> u64 {
  let mut output: u64 = 0;
  let mut bitshift = 56;

  for x in 0..8 {
    output += (buffer[x] as u64) << bitshift;
    bitshift -= 8;
  }

  output
}

impl <'a> Response<'a, ZigbeeReceivePacket<'a>> for ZigbeeReceivePacket<'a> {
  fn respond_to(id: u8) -> bool {
    COMMAND_ID == id
  }

  fn parse(buffer: &'a [u8]) -> Result<ZigbeeReceivePacket<'a>, ResponseError> {
    if buffer.len() < 12 { return Err(ResponseError::SizeIncorrectError); }
    if buffer[0] != COMMAND_ID { return Err(ResponseError::IdError); }

    let destination = buffer_to_u64(&buffer[1..9]);
    let network_address: u16 = buffer[9] as u16 * 256 + buffer[10] as u16;
    let packet_acknowledged = buffer[11] & 0x01 != 0;
    let packet_broadcast = buffer[11] & 0x02 != 0;
    let packet_encrypted = buffer[11] & 0x20 != 0;
    let packet_end_device = buffer[11] & 0x40 != 0;
    let data = &buffer[12..];

    Ok(Self {
      destination,
      network_address,
      packet_acknowledged,
      packet_broadcast,
      packet_encrypted,
      packet_end_device,
      data
    })
  }
}

#[test]
fn test_at_command_response_parse() {
  let buffer: [u8; 18] = [
    0x90,
    0x00, 0x13, 0xA2, 0x00, 0x40, 0x52, 0x2B, 0xAA,
    0x7D, 0x84,
    0x01,
    0x52, 0x78, 0x44, 0x61, 0x74, 0x61
  ];
  let response = ZigbeeReceivePacket::parse(&buffer[..]).unwrap();

  assert_eq!(response.destination, 0x0013_A200_4052_2BAA);
  assert_eq!(response.network_address, 0x7D84);
  assert_eq!(response.packet_acknowledged, true);
  assert_eq!(response.packet_broadcast, false);
  assert_eq!(response.packet_broadcast, false);
  assert_eq!(response.packet_broadcast, false);
  assert_eq!(response.data, &buffer[12..]);
}
