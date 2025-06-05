//primitive data types: int, float (f32, f64), bool, char
// i8, i16, i32, i64, i128: signed - both: - and +
// u8, u16, u32, u64, u128: unsigned - only: +

fn primitive_types() {
    let x: i32 = -56;
    let y: u64 = 100;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);
    let p: f32 = 3.14;
    let is_rainy: bool = false;
    println!("float data type: {}", p);
    println!("Is it rainy: {}", is_rainy);
    let z: char = 'a';
    println!("char: {}", z);
}


