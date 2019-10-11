use core::num::Wrapping;
use super::commands::{ Command, BufferSizeError };

/// Representation of a single Frame.
pub struct Frame<'a> {
    pub data: &'a [u8]
}

impl <'a> Frame<'a> {
    /// Builds a new frame given a borrowed slice of an array of u8s.
    pub fn new(data: &'a [u8]) -> Self {
        Frame { data }
    }

    /// Uses a FrameIterator to return an iterator that can be used to
    /// return a serialized message byte by byte.
    pub fn serialize(&self) -> FrameIterator {
        FrameIterator {
            frame: &self,
            state: FrameIteratorState::Delimiter,
            state_index: 0,
            checksum: Wrapping(0)
        }
    }

    /// The `from_command` function is used to fill array `buffer` using
    /// the data contained in command `command`. The only failure condition
    /// is if a buffer is supplied with insufficient space.
    pub fn from_command<T: Command>(command: T, buffer: &'a mut [u8]) -> Result<Self, BufferSizeError> {
        match command.fill_buffer(buffer) {
            Ok(data) => Ok(Frame { data }),
            Err(BufferSizeError) => Err(BufferSizeError)
        }
    }
}

enum FrameIteratorState {
    Delimiter,
    LengthLsb,
    LengthMsb,
    Data,
    Checksum,
    Done
}

/// Since a Frame only contains a single field with a payload, the
/// FrameIterator represents the data needed to iterating through
/// a full frame including the delimiter, length bytes, data, and
/// checksum.
pub struct FrameIterator<'a> {
    frame: &'a Frame<'a>,
    state: FrameIteratorState,
    state_index: usize,
    checksum: Wrapping<u8>
}

impl <'a> Iterator for FrameIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let return_val: u8 = match self.state {
            FrameIteratorState::Delimiter => {
                self.state = FrameIteratorState::LengthMsb;
                0x7E
            },
            FrameIteratorState::LengthMsb => {
                self.state = FrameIteratorState::LengthLsb;
                (self.frame.data.len() / 256) as u8
            },
            FrameIteratorState::LengthLsb => {
                self.state = FrameIteratorState::Data;
                (self.frame.data.len() % 256) as u8
            },
            FrameIteratorState::Data => {
                let current_byte = self.frame.data[self.state_index];

                self.state_index += 1;
                if self.state_index == self.frame.data.len() {
                    self.state_index = 0;
                    self.state = FrameIteratorState::Checksum;
                }

                self.checksum = self.checksum + Wrapping(current_byte);

                current_byte
            },
            FrameIteratorState::Checksum => {
                let checksum = 0xFF - self.checksum.0;
                self.state = FrameIteratorState::Done;

                checksum
            },
            FrameIteratorState::Done => {
                return None;
            }
        };

        Some(return_val)
    }
}



#[test]
fn test_serialize() {
    let at_command: [u8; 5] = [0x08, 0x01, 0x4E, 0x4A, 0xFF];
    let frame = Frame::new(&at_command);

    let expected: [u8; 9] = [0x7E, 0x00, 0x05, 0x08, 0x01, 0x4E, 0x4A, 0xFF, 0x5F];

    let mut index = 0;
    for ch in frame.serialize() {
        assert_eq!(ch, expected[index]);
        index += 1;
    }
}

#[test]
fn test_from_command_success() {
    // Start with a purposefully small buffer
    let mut buffer: [u8; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let frame_id = 0x52;
    let command = super::commands::AtCommand::new(frame_id, [b'N', b'J'], None);
    let frame = Frame::from_command(command, &mut buffer[..]);

    match frame {
        Ok(frame) => {
            let mut index = 0;
            let expected: [u8; 8] = [0x7E, 0x00, 0x04, 0x08, frame_id, 0x4E, 0x4A, 0x0D];
            for ch in frame.serialize() {
                assert_eq!(ch, expected[index]);
                index += 1;
            }
        },
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_from_command_failure() {
    // Start with a purposefully small buffer
    let mut buffer: [u8; 1] = [0];
    let command = super::commands::AtCommand::new(0x52, [b'N', b'J'], None);
    let f = Frame::from_command(command, &mut buffer[..]);

    assert_eq!(f.is_err(), true);
}
