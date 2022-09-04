fn main() { // Primitive Data types

    // these are the fundamental data types used to declare a variable

    // We have 2 categories of PDT in RUST
    // 1. Scaler: Each value can be compared to any other values as either equal, greater or less
    // Ex: int, floar, char

    // 2. Compound: aka Composite data type i.e. multiple data types.
    // Ex: tuple, array.

    let x: i32 = 2; // Explicitly defined integer of 32 bits. It is also the default type for the impllicit
    // i8, i16, i32, i64, i128

    // u8 2^8 so range is 0 to (2^8 - 1) i.e 0 to 255
    // i8 2^8 but range is -2^7 to 2^7 - 1 i.e -128 to 127

    // f32 aka single precision and f64 aka double precision

    // f64 is the default precision when implicit conversion

    let float = 10.92;

    // boolean 0 or 1

    let t_or_f = false;

    // char

    let letter: char = 'a';




}

