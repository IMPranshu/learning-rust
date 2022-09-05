// prelude is the list of things that RUST automatically imports into every program.
// think of this as an import statement used in C++ or similar in other languages.

// Note that user input is not included in the Prelude that is something unique.
// we have to manually import that.

// A package/library i.e. collection of code is known as "Crate" in Rust lang.
// Within in package/library there are "Module" which has a specific piece of functionality

use std::io; // we are using the "std" crate/package here and bringing only the "IO" module


fn main() {
    // here we are creating a mutable string. String is different in Rust than other languages.
    let mut input = String::new(); // :: is known as the "path separator operator"

    // we are creating a mutable reference for the input varable for modifying the data that is
    // stored in the input variable.
    // if we don't use &mut the read_line function will make a copy of the
    // input variable and run all the operations on that copy and it will not get
    // reflected in the original input variable.

    io::stdin().read_line(&mut input).expect("error: failed to read line."); // the .expect is the method which will throw error if any input is not given or
    // if any unexpected error occurs during the reading input.

    println!("{}", input);



}

