// Automatically generated rust module for 'Sudoku.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SudokuRow {
    pub row: Vec<i32>,
}

impl<'a> MessageRead<'a> for SudokuRow {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.row = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SudokuRow {
    fn get_size(&self) -> usize {
        0
        + if self.row.is_empty() { 0 } else { 1 + sizeof_len(self.row.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.row, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SudokuBoard {
    pub rows: Vec<SudokuRow>,
}

impl<'a> MessageRead<'a> for SudokuBoard {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.rows.push(r.read_message::<SudokuRow>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SudokuBoard {
    fn get_size(&self) -> usize {
        0
        + self.rows.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.rows { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

