macro_rules! include_data {
    ($name:ident, $filename:expr)  => {
        pub static $name: &'static [u8] = include_bytes!($filename);
    }
}

//Put all your data declarations here
include_data!(MURPH, "murph.bmp");
include_data!(BRICK, "brick.bmp");
