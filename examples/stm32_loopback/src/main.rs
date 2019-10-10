#![no_main]
#![no_std]

extern crate panic_halt;
use core::num::Wrapping;

use rustbee::{
    buffer::{ FrameBuffer },
    frame::{ Frame },
    commands::{ TransmitRequestCommand },
    responses::{ Response, ZigbeeReceivePacket }
};

use cortex_m::asm;

use nb::block;

use stm32f1xx_hal::{
    prelude::*,
    pac,
    serial::{Config, Serial},
};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let p = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Prepare the alternate function I/O registers
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    // Prepare the GPIOA peripheral
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

    // USART2
    let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx = gpioa.pa3;

    // Set up the usart device. Taks ownership over the USART register and tx/rx pins. The rest of
    // the registers are used to enable and configure the device.
    let serial = Serial::usart2(
        p.USART2,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        clocks,
        &mut rcc.apb1,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx, mut rx) = serial.split();

    // Set up the Zigbee buffer
    let mut incoming_buffer: [u8; 100] = unsafe { core::mem::zeroed() };
    let mut outgoing_buffer: [u8; 100] = unsafe { core::mem::zeroed() };
    let mut frame_buffer = FrameBuffer::new(&mut incoming_buffer);
    let mut frame_id = Wrapping(0);

    loop {
        let received = block!(rx.read()).unwrap();

        if let Some(received_frame) = frame_buffer.push(received) {
            // Don't respond to anything except a received packet
            if !ZigbeeReceivePacket::respond_to(received_frame.data[0]) {
                continue;
            }

            // If we can't parse, keep going
            let received_message = ZigbeeReceivePacket::parse(received_frame.data);
            if received_message.is_err() {
                continue;
            }
            let received_message = received_message.unwrap();

            // Build and send the response
            let response_msg = TransmitRequestCommand::broadcast(frame_id.0, received_message.data);
            frame_id += Wrapping(1);

            for character in Frame::from_command(response_msg, &mut outgoing_buffer).unwrap().serialize() {
                block!(tx.write(character)).ok();
            }
        }
    }
}
