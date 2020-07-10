use fst_stringstring::builder::StringBuilder;
use fst_stringstring::strings::StringMap;

#[test]
fn builder1() -> std::io::Result<()> {
   let mut builder = StringBuilder::new("testy.fmm")?;
   let i1 = builder.insert("abcd")?;
   let i2 = builder.insert("efgh")?;
   builder.finish();
   assert_eq!(i1, 0);
   assert_eq!(i2, 5);

   let map = StringMap::new("testy.fmm")?;
   assert_eq!( map.get(0), "abcd" );
   assert_eq!( map.get(1), "bcd" );
   assert_eq!( map.get(2), "cd" );
   assert_eq!( map.get(3), "d" );
   assert_eq!( map.get(4), "" );
   assert_eq!( map.get(5), "efgh" );
   assert_eq!( map.get(6), "fgh" );
   assert_eq!( map.get(7), "gh" );
   assert_eq!( map.get(8), "h" );
   assert_eq!( map.get(9), "" );
   assert_eq!( map.get(10), "" );

   std::fs::remove_file("testy.fmm")?;

   Ok(())
}
