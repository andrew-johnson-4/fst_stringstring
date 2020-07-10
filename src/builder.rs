use std::fs::{File,remove_file};
use std::path::Path;
use std::io::prelude::*;

pub struct StringBuilder {
   at: usize,
   building: Option<File>,
}

impl StringBuilder {
   pub fn new(fp: &str) -> std::io::Result<StringBuilder> {
      if Path::new(fp).exists() {
         remove_file(fp)?;
      }
      let f = File::create(fp)?;
      Ok(StringBuilder {
         at: 0,
         building: Some(f),
      })
   }
   pub fn insert(&mut self, s: &str) -> std::io::Result<usize> {
      Ok(if let Some(ref mut f) = self.building {
         f.write_all(s.as_bytes())?;
         f.write_all(b"\0")?;
         let at = self.at;
         self.at += s.len() + 1;
         at
      } else { 0 })
   }
   pub fn finish(&mut self) {
      self.building = None;
   }
}
