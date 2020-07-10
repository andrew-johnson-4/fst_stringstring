use crate::strings::StringMap;
use fst::Map;
use memmap::Mmap;

pub struct StringStringMap {
   fst_map: Map<Mmap>,
   strings: StringMap,
}

impl StringStringMap {
   pub fn new(fst_map: Map<Mmap>, strings: StringMap) -> StringStringMap {
      StringStringMap {
         fst_map: fst_map,
         strings: strings
      }
   }
   pub fn get(&self, k: &str) -> Option<String> {
      if let Some(si) = self.fst_map.get(k) {
         Some(self.strings.get(si as usize))
      } else { None }
   }
   pub fn len(&self) -> usize {
      self.fst_map.len()
   }
   pub fn is_empty(&self) -> bool {
      self.fst_map.is_empty()
   }
}
