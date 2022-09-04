fn main() { // variables

    // variables in RUST is different from other programming languages in general
    // It is staticly and strongly typed

    let x =4; // implicitly assigned by compiler
    //let y: u32 = 5; // expliccitly typed

    println!("x is: {} ", x);

    { // different scope
        let x = x + 2;
        println!("x is: {}", x);
    }

    let x = x+1;
    println!("x is: {} ", x);






}

// by default in Rust all variables defined are immutable i.e. we cannot change them


// Name shadowing means using the same variable name but with different scope
