pub fn darken_amount(color: &(u8, u8, u8), amount: u8) -> (u8, u8, u8) {
    let new_red: u8;
    let new_green: u8;
    let new_blue: u8;
    new_red = if amount < color.0 {
        color.0 - amount
    } else {
        0
    };
    new_green = if amount < color.1 {
        color.1 - amount
    } else {
        0
    };
    new_blue = if amount < color.2 {
        color.2 - amount
    } else {
        0
    };
    (new_red, new_green, new_blue)
}

pub fn lighten_amount(color: &(u8, u8, u8), amount: u8) -> (u8, u8, u8) {
    let new_red: u8;
    let new_green: u8;
    let new_blue: u8;
    new_red = if 255 - amount > color.0 {
        color.0 + amount
    } else {
        255
    };
    new_green = if 255 - amount > color.1 {
        color.1 + amount
    } else {
        255
    };
    new_blue = if 255 - amount > color.2 {
        color.2 + amount
    } else {
        255
    };
    (new_red, new_green, new_blue)
}
