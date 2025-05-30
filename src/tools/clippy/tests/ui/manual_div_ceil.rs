#![warn(clippy::manual_div_ceil)]

macro_rules! y {
    () => {
        let x = 33u32;
        let _ = (x + 7) / 8;
        //~^ manual_div_ceil
        let _ = (7 + x) / 8;
        //~^ manual_div_ceil
    };
}

macro_rules! eight {
    () => {
        8
    };
}

fn main() {
    let x = 7_u32;
    let y = 4_u32;
    let z = 11_u32;

    // Lint
    let _ = (x + (y - 1)) / y;
    //~^ manual_div_ceil
    let _ = ((y - 1) + x) / y;
    //~^ manual_div_ceil
    let _ = (x + y - 1) / y;
    //~^ manual_div_ceil

    let _ = (7_u32 + (4 - 1)) / 4;
    //~^ manual_div_ceil
    let _ = (7_i32 as u32 + (4 - 1)) / 4;
    //~^ manual_div_ceil

    // No lint
    let _ = (x + (y - 2)) / y;
    let _ = (x + (y + 1)) / y;

    let _ = (x + (y - 1)) / z;

    let x_i = 7_i32;
    let y_i = 4_i32;
    let z_i = 11_i32;

    // No lint because `int_roundings` feature is not enabled.
    let _ = (z as i32 + (y_i - 1)) / y_i;
    let _ = (7_u32 as i32 + (y_i - 1)) / y_i;
    let _ = (7_u32 as i32 + (4 - 1)) / 4;

    // Test lint with macro
    y!();

    // Also test if RHS should be result of macro expansion
    let _ = (33u32 + 7) / eight!();
    //~^ manual_div_ceil
}

fn issue_13843() {
    let x = 3usize;
    let _ = (2048 + x - 1) / x;
    //~^ manual_div_ceil

    let x = 5usize;
    let _ = (2048usize + x - 1) / x;
    //~^ manual_div_ceil

    let x = 5usize;
    let _ = (2048_usize + x - 1) / x;
    //~^ manual_div_ceil

    let x = 2048usize;
    let _ = (x + 4 - 1) / 4;
    //~^ manual_div_ceil

    let _: u32 = (2048 + 6 - 1) / 6;
    //~^ manual_div_ceil
    let _: usize = (2048 + 6 - 1) / 6;
    //~^ manual_div_ceil
    let _: u32 = (0x2048 + 0x6 - 1) / 0x6;
    //~^ manual_div_ceil

    let _ = (2048 + 6u32 - 1) / 6u32;
    //~^ manual_div_ceil

    let _ = (1_000_000 + 6u32 - 1) / 6u32;
    //~^ manual_div_ceil
}

fn issue_13950() {
    let x = 33u32;
    let _ = (x + 7) / 8;
    //~^ manual_div_ceil
    let _ = (7 + x) / 8;
    //~^ manual_div_ceil

    let y = -33i32;
    let _ = (y + -8) / -7;
    let _ = (-8 + y) / -7;
    let _ = (y - 8) / -7;
}
