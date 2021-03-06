Fst String-to-String Map
========================

[![Crates.IO](https://img.shields.io/crates/v/fst_stringstring.svg)](https://crates.rs/crates/fst_stringstring)
[![Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/fst_stringstring)
[![Build Nightly](https://github.com/andrew-johnson-4/fst_stringstring/workflows/BuildNightly/badge.svg)](https://github.com/andrew-johnson-4/fst_stringstring)
[![Build](https://github.com/andrew-johnson-4/fst_stringstring/workflows/Build/badge.svg)](https://github.com/andrew-johnson-4/fst_stringstring)

A compact String-to-String Map built on Finite State Transducers (fst).

```rust
use fst_stringstring::builder::StringBuilder;
use fst_stringstring::strings::StringMap;
use fst_stringstring::map::StringStringMap;
use std::fs::{File,remove_file};
use std::path::Path;
use fst::Map;
use memmap::Mmap;

#[test]
fn mbuilder1() -> std::io::Result<()> {
   if Path::new("testy.fst").exists() { remove_file("testy.fst")?; }
   if Path::new("testy.fmm").exists() { remove_file("testy.fmm")?; }

   let mut builder = StringBuilder::new("testy.fmm")?;
   let wtr = std::io::BufWriter::new(File::create("testy.fst")?);
   let mut fst_builder = fst::MapBuilder::new(wtr).expect("Unable to create fst::MapBuilder");

   let i1 = builder.insert("abcd")?;
   fst_builder.insert("a", i1 as u64).expect("fst insert");
   let i2 = builder.insert("efgh")?;
   fst_builder.insert("b", i2 as u64).expect("fst insert");
   builder.finish();
   fst_builder.finish().expect("Unable to finish build of fst::MapBuilder");

   let mmap = unsafe {
      Mmap::map(&File::open("testy.fst")?).expect("unable to mmap file")
   };
   let fmap = Map::new(mmap).expect("Could not create fst::Map from file");
   let smap = StringMap::new("testy.fmm")?;
   let map = StringStringMap::new(fmap, smap);

   assert_eq!( map.get("a"), Some("abcd".to_string()) );
   assert_eq!( map.get("b"), Some("efgh".to_string()) );
   assert_eq!( map.get("c"), None );

   if Path::new("testy.fst").exists() { remove_file("testy.fst")?; }
   if Path::new("testy.fmm").exists() { remove_file("testy.fmm")?; }
   Ok(())
}
```

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in fst_stringstring by you,
shall be dual licensed under the MIT and Apache 2.0 license without any additional terms or conditions.
