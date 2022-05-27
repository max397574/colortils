pub fn complementary(color: &(u8, u8, u8)) -> (u8, u8, u8) {
    (255 - color.0, 255 - color.1, 255 - color.2)
}

pub fn grayscale(color: &(u8, u8, u8)) -> (u8, u8, u8) {
    let red_val: f32 = color.0 as f32;
    let green_val: f32 = color.1 as f32;
    let blue_val: f32 = color.2 as f32;
    let f_value = red_val * 0.2126 + green_val * 0.7152 + blue_val * 0.0722;
    let u_value: u8 = f_value as u8;
    (u_value, u_value, u_value)
}
