extern crate protobuf;
extern crate byteorder;
use std::io::prelude::*;
use protobuf::{MessageStatic, Message, parse_from_bytes};
use byteorder::{ReadBytesExt, LittleEndian, WriteBytesExt};
use std::io::Cursor;

pub fn read_pbuf<T>(reader: &mut Box<BufRead>) -> Vec<T>
  where T: MessageStatic {
  let mut event: T;
  let mut events: Vec<T> = Vec::new();
  let mut head = vec![0; 2];

  loop {
    match reader.read_exact(&mut head) {
      Ok(n) => n,
      Err(_) => break
    };
    let h = header(&head);
    let mut proto = vec![0; h];
    match reader.read_exact(&mut proto) {
      Ok(n) => n,
      Err(e) => panic!("{:?}", e)
    };
    event = match parse_from_bytes::<T>(&mut proto) {
              Ok(n) => n,
              Err(e) => panic!("{:?}", e)
            };
    events.push(event);
  }
  events
}

fn header(sz: &Vec<u8>) -> usize {
  let mut rdr = Cursor::new(sz);
  match rdr.read_i16::<LittleEndian>() {
    Ok(x) => x as usize,
    Err(e) => panic!("Something wicked this way comes: {}", e)
  }
}

pub fn write_pbuf<T>(event: &T, writer: &mut Box<Write>) 
  where T: Message {
  let bts = Message::write_to_bytes(event).unwrap();
  let l = bts.len();
  match writer.write_u16::<LittleEndian>(l as u16) {
    Ok(n) => n,
    Err(e) => panic!("{:?}", e)
  };
  match writer.write(&bts[..]) {
    Ok(n) => assert_eq!(l, n),
    Err(e) => panic!("{:?}", e)
  }
}