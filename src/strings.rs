use std::fs::File;
use memmap::Mmap;

pub struct StringMap {
   mmap: Mmap,
}

impl StringMap {
   pub fn new(fp: &str) -> std::io::Result<StringMap> {
      let f = File::open(fp)?;
      let mmap = unsafe { Mmap::map(&f)? };

      Ok(StringMap {
         mmap: mmap,
      })
   }
   pub fn get(&self, i: usize) -> String {
      if let Some(slice) = self.mmap.get(i..) {
         let mut ai = 0;
         while ai < slice.len() && slice[ai]!=0 {
            ai += 1;
         }
         std::str::from_utf8(&slice[..ai]).unwrap_or("").to_string()
      } else { "".to_string() }
   }
}
