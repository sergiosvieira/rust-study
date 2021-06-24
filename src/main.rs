use std::io::Bytes;

fn main() {
    // ***** VARIABLES AND MUTABILITY *****
    //1. by default variables are immutable
    // variable can be infered by value or by type
    // Valid types: Integer: signed (i8, i16, i32, i64, i128 and isize [architecture dependent])
    //                     unsigned (u8, u16, u32, u64, u128 and usize [architecture dependent])
    //              Literal Numbers: Decimal 98_222 or 98222
    //                               Hex 0xff
    //                               Octal 0o77
    //                               Binary 0b1011_0000
    //                               Byte (u8 only) b'A'
    // by default integer is i32
    //2. let i; <-- error: consider giving `i` a type
    let k: i32; //3. warning (unused variable)
    let _w: i32; //4. if you put _ (underscore) before the variable name, it means,
                 //5. ok, I will not annoying you with this warning anymore
    let i = 20;
    // i = 30; <--6. error: the variable is immutable!
    // println!(i); <--7. error: you can't print the value of a variable directly
    println!("i:{}", i);
    //8. if you want change the values of a variable you must use the keyword mut
    //let mut j; <--8. error: consider giving `j` a type
    let mut j: i32; //9. warning (unused variable)
    let mut _m: i32;
    // println!("m:{}", _m); <--10. error: use of possibly-uninitialized `_m`
    _m = 10;
    println!("_m:{}", _m);
    // ***** CONSTANTS *****
    // 10. constants are aways immutables
    // const mut n = 10; // <--11. error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`,
    //                             or an operator. Cannot be mutable
    // const n = 10; //<--12. error: When you use const keyword you must provide a type for the 
    //                        item: `n: i32`
    // const n:i32 = 10;// warning: convert the identifier to upper case
    // const N:i32; //13. error: provide a definition for the constant: `= <expr>;`
    const N:i32 = 5;
    println!("N:{}", N);
    //const M:i32 = _m; // error: attempt to use a non-constant value in a constant
    let _n = N;
    println!("_n:{}", _n);// valid: you can use a constant value in a immutable variable
    // ***** SHADOWING *****
    // you can use the same name for different declarations of immutable variables
    let _n = _n + _m; // _n remains immutable after this transformation
    println!("_n + _m:{}", _n);
    // better usage sample
    let text = "this is a string";
    let text = text.len();
    // you don't need to create two variables to store the string (text_str) and len (text_len) 
    // of the string
    println!("text length:{}", text);
    // let mut text = "this is a string";
    // let mut text = text.len(); // error: expected `&str`, found `usize`
    // ***** DATA TYPES *****
    // Rust is statically typed language, so it must know the type of all variables at compile time.
    // If you want convert a string to other type you can use the method parse
    // but you must to specify the type of variable
    // let guess = "42".parse(); // error: cannot infer type for type parameter `F` declared on the 
    //                              associated function `parse`    
    //let guess: i32 = "42".parse();// error: expected `i32`, found enum `Result`
    // The parse method return an enum `Result`.
    // Result is a type that represents either success (Ok) or failure (Err)
    // So, to get the result of conversion you must use something like this
    let r: Result<i32, std::num::ParseIntError> = "42".parse();
    println!("{:?}", r);
    // From the result of conversion (Result<i32, std::num::ParseIntError>) you can get the converted value
    if r.is_ok() {
        println!("converted:{}", r.unwrap());
    } else {
        println!("Conversion error");
    }
    // Finally, you can do the conversion directly
    let r: i32 = "42".parse().unwrap();
    println!("r:{}", r);
    // you can use the turbofish sintax
    let r = "42".parse::<i32>().unwrap();
    println!("r:{}", r);
    // if you get an invalid integer, that will generate a conversion error
    //let r = "value".parse::<i32>().unwrap();// error: ParseIntError { kind: InvalidDigit }'
    println!("r:{}", r);
    // ***** COMPOUND TYPES *****
    // ===== Tuples
    // The tuple can't shrink or grow and can group different or equal types of variables
    let tuple = (1, 1., true, "string");
    println!("{:?}", tuple);
    // to get a specific value you must provide the index position of value in tuple
    println!("{}", tuple.3);// get the element on position 3 ("string")
    // println!("{}", tuple.4); // error:  no field `4` on type `({integer}, {float}, bool, &str)`
    // you can break a single tuple em parts
    let (x, y, z, w) = tuple;
    println!("x:{} y:{} z:{} w:{}", x, y, z, w);
    // ===== Arrays
    // In rust arrays always have fixed length and it stores on same variable types
    let array: [i32; 4] = [1, 2, 3, 4];// the type of array is defined by [type; number of elements]
    println!("{:?}", array);
    let rarray = [1;50]; // it will create an array with 50 repeated values equals to 1
    println!("{:?}", rarray);
    // To get a specific value from array, you must use the [] syntax
    println!("value at position 2 is equal to {}", array[2]);
    // If you try to get a value from array using an invalid index, Rust will immediatly halt the
    // code execution
    //println!("{}", array[4]); // error: index out of bounds: the length is 4 but the index is 4
}
