fn main() { //  all the variables are immutable by default and we can use 'mut' keyword to make it muttable.

    // Compund Data types

    // tuple: It is going to be a fixed length
    // of sequence of elements that is immutable

    let mut tup = (1, true, 'b'); // impliccitly assigned to (i32, bool, char)
    let tup2: (i8, bool, char) = (1, true, 'b'); // tup2 is not equal to tup as the datatype is different for both

    tup = (10,false, 'c','e'); // we cannot change the type of the tuple. If it is having 3 elements we can't add 4 elements and likewise.

    println!("{}", tup.0);






}

