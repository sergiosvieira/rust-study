fn main() {
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
    println!("m:{}", _m);
    // 10. constants are always immutables
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
}
