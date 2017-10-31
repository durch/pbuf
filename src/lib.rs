extern crate protobuf;
extern crate byteorder;
use std::io::prelude::*;
use protobuf::{MessageStatic, Message, parse_from_bytes};
use byteorder::{ReadBytesExt, LittleEndian, WriteBytesExt};
use std::io::Cursor;

pub mod error;
use error::PBErr;

pub fn read_many<M, R>(reader: &mut R) -> Result<Vec<M>, PBErr>
  where M: MessageStatic, R: Read {
  let mut msg: M;
  let mut msgs: Vec<M> = Vec::new();
  let mut head = vec![0; 2];

  while let Ok(_) = reader.read_exact(&mut head) {
    let h = header(&head)?;
    let mut proto = vec![0; h];
    reader.read_exact(&mut proto)?;
    msg = parse_from_bytes::<M>(&proto)?;
    msgs.push(msg);
  }
  Ok(msgs)
}

pub fn read_one<M, R>(reader: &mut R) -> Result<M, PBErr>
  where M: MessageStatic, R: Read {
    let mut head = vec![0; 2];

    reader.read_exact(&mut head)?;
    let h = header(&head)?;
    let mut proto = vec![0; h];
    reader.read_exact(&mut proto)?;
    Ok(parse_from_bytes::<M>(&proto)?)
}

fn header(sz: &[u8]) -> Result<usize, PBErr> {
  Ok(Cursor::new(sz).read_i16::<LittleEndian>()? as usize)
}

pub fn write<M, W>(msg: &M, writer: &mut W) -> Result<usize, PBErr>
  where M: Message, W: Write {
  let bts = Message::write_to_bytes(msg)?;
  let l = bts.len();
  writer.write_u16::<LittleEndian>(l as u16)?;
  writer.write_all(&bts[..])?;
  Ok(l)
}
