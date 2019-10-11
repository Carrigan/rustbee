use core::num::Wrapping;
use super::frame::Frame;

#[derive(Debug)]
pub enum FrameBufferState {
  WaitingForDelimiter,
  ReceivingLengthMsb,
  ReceivingLengthLsb,
  ReceivingData,
  ReceivingChecksum
}

/// A state machine that handles receiving bytes coming from an XBee and
/// alerting the consuming application when a full frame has been received.
pub struct FrameBuffer <'a> {
  // The state of the receive buffer
  state: FrameBufferState,

  // The buffer and current index info
  buffer: &'a mut [u8],
  count: usize,

  // Save how many bytes there are left to read
  left: u16,

  // Checksum related data
  checksum: Wrapping<u8>
}

impl <'a> FrameBuffer <'a> {
  /// Create a new `FrameBuffer` with borrowed array space `buffer`.
  pub fn new(buffer: &'a mut [u8]) -> Self {
    Self {
      state: FrameBufferState::WaitingForDelimiter,
      buffer: buffer,
      count: 0,
      left: 0,
      checksum: Wrapping(0)
    }
  }

  /// Receive a single byte of data from the XBee device and return
  /// a Frame if completed.
  pub fn receive(&mut self, received: u8) -> Option<Frame> {
    match self.state {
      FrameBufferState::WaitingForDelimiter => {
        if received == 0x7E {
          self.state = FrameBufferState::ReceivingLengthMsb;
        }

        None
      },

      FrameBufferState::ReceivingLengthMsb => {
        self.left += (received as u16) << 8;
        self.state = FrameBufferState::ReceivingLengthLsb;

        None
      },

      FrameBufferState::ReceivingLengthLsb => {
        self.left += received as u16;
        self.state = FrameBufferState::ReceivingData;

        None
      },

      FrameBufferState::ReceivingData => {
        self.left -= 1;
        if self.left == 0 {
          self.state = FrameBufferState::ReceivingChecksum;
        }

        self.buffer[self.count] = received;
        self.checksum += Wrapping(received);
        self.count += 1;

        None
      },

      FrameBufferState::ReceivingChecksum => {
        let return_val = if received == 0xFF - self.checksum.0 {
          Some(Frame::new(&self.buffer[0..self.count]))
        } else {
          None
        };

        // Reset everything
        self.state = FrameBufferState::WaitingForDelimiter;
        self.count = 0;
        self.left = 0;
        self.checksum = Wrapping(0);

        return_val
      }
    }
  }
}

#[test]
fn test_buffer_receive() {
  // Setup the buffer we will receive into
  let mut receive_buffer: [u8; 100] = unsafe { core::mem::zeroed() };
  let mut frame_buffer = FrameBuffer::new(&mut receive_buffer);

  // Setup a mock TransmitRequestCommand
  let mut send_buffer: [u8; 100] = unsafe { core::mem::zeroed() };
  let data: [u8; 8] = [0x54, 0x78, 0x44, 0x61, 0x74, 0x61, 0x30, 0x41];
  let request = super::commands::TransmitRequestCommand::to_destination(
    1, 0x0013_A200_400A_0127, &data[..]
  );

  let send_frame = Frame::from_command(request, &mut send_buffer[..]).unwrap();

  // Iterate through and check that they are equal
  for character in send_frame.serialize() {
    if let Some(received_frame) = frame_buffer.receive(character) {
      assert_eq!(send_frame.data, received_frame.data);
      return;
    }
  }

  // If we hit here, the frame was never built
  assert!(false);
}
