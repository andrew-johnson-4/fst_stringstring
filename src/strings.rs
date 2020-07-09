
pub struct StringMap;

impl StringMap {
   pub fn new(_fp: &str) -> StringMap {
      StringMap {}
   }
   pub fn get<'a>(&self, _i: usize) -> &'a str {
      ""
   }
}
