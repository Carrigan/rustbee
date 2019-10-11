# Rustbee

This project provides a way to access an XBee module in API mode. In the API mode, all commands to and from the module are represented as `Frame`s. This project provides a convenient way to work with `Frame`s that is independent from how that data is transferred.

## Receiving and Parsing Packets

```rust
use rustbee::{
  FrameBuffer,
  responses::ZigbeeReceivePacket
};

let mut incoming_buffer: [u8; 100] = unsafe { core::mem::zeroed() };
let mut frame_buffer = FrameBuffer::new(&mut incoming_buffer);

loop {
  let received: u8 = receive_byte();

  if let Some(received_frame) = frame_buffer.receive(received) {
    // Don't respond to anything except a received packet
    if !ZigbeeReceivePacket::respond_to(received_frame.data[0]) {
        continue;
    }

    // If we can't parse, keep going
    let received_message = match ZigbeeReceivePacket::parse(received_frame.data) {
        Ok(msg) => msg,
        _ => continue
    }
  }
}
```

## Sending Packets

```rust
use rustbee::{
    Frame,
    commands::TransmitRequestCommand,
};

// Set up the Zigbee buffer
let mut outgoing_buffer: [u8; 100] = unsafe { core::mem::zeroed() };
let frame_id: u8 = 0;

// Build the response payload
let outgoing_message = TransmitRequestCommand::broadcast(frame_id, received_message.data);

// Build the response payload into a full frame
let frame = Frame::from_command(response_msg, &mut outgoing_buffer).unwrap();

// Send our frame byte by byte
for character in frame.serialize() {
    send_byte(character);
}
```

## Status

- [X] Ability to round-trip messages on XBee devices in API mode.
- [X] Ability to send AT command messages and read their responses.
- [ ] Support for other delimiter modes
- [ ] Support for additional messages
- [ ] Switch to a state-machine based method of iterating through commands similar to how `Frame` does it.

### Message Implementation

- [X] 0x08 - AT Command
- [ ] 0x09 - AT Command - Queue Parameter Value
- [X] 0x10 - Zigbee Transmit Request
- [ ] 0x11 - Explicit Addressing Zigbee Command Frame
- [ ] 0x17 - Remote Command Request
- [ ] 0x21 - Create Source Route
- [X] 0x88 - AT Command Response
- [ ] 0x8A - Modem Status
- [ ] 0x8B - Zigbee Transmit Status
- [X] 0x90 - Zigbee Receive Packet
- [ ] 0x91 - Zigbee Explicit RX Indicator
- [ ] 0x92 - Zigbee IO Data Sample RX Indicator
- [ ] 0x94 - XBee Sensor Read Indicator
- [ ] 0x95 - Node Identification Indicator
- [ ] 0x97 - Remote Command Response
