use fontkit::{Error, FontKey};
use std::fs;
use std::io::Read;

#[test]
pub fn test_font_loading() -> Result<(), Error> {
    let mut buf = vec![];
    let mut f = fs::File::open("examples/OpenSans-Italic.ttf")?;
    f.read_to_end(&mut buf)?;
    let fontkit = fontkit::FontKit::new();
    let _ = fontkit.add_font_from_buffer(buf)?;
    Ok(())
}

#[test]
pub fn test_variable_font_loading() -> Result<(), Error> {
    let mut buf = vec![];
    let mut f = fs::File::open("examples/AlimamaFangYuanTiVF.ttf")?;
    f.read_to_end(&mut buf)?;
    let fontkit = fontkit::FontKit::new();
    let _ = fontkit.add_font_from_buffer(buf)?;
    let mut key = FontKey::default();
    key.family = "AlimamaFangYuanTiVF-Medium-Round".into();
    assert!(fontkit.query(&key).is_some());
    Ok(())
}
