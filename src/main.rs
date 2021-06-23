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
    //2. let i; <-- error!!!, consider giving `i` a type
    let k: i32; //3. warning (unused variable)
    let _w: i32; //4. if you put _ (underscore) before the variable name, it means,
                 //5. ok, I will not annoying you with this warning anymore
    let i = 20;
    // i = 30; <--6. error!!! variable are immutable!
    // println!(i); <--7. error!!!
    println!("i:{}", i);
    //8. if you want change the values of a variable you must use the keyword mut
    //let mut j; <--8. error!!! consider giving `j` a type
    let mut j: i32; //9. warning (unused variable)
    let mut _m: i32;
    // println!("m:{}", _m); <--10. error!!! use of possibly-uninitialized `_m`
    _m = 10;
    println!("m:{}", _m);

}
