fn main() {
    // Deu ruim! Rust para programadores
    // Brain compiler! Rust for programmers
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
    // by default, "primitive" data types are stored in the stack.
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
    // in Rust, all "primitive" data type have related methods, so, they don't 
    // are really primitive like other languages, like c++ or c.
    println!("{}", _m.is_positive());// numbers have methods!!!
    // ***** CONSTANTS *****
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
    // you can use the turbofish sintax ::<>
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
    // you can break a single tuple in parts
    let (x, y, z, w) = tuple;
    println!("x:{} y:{} z:{} w:{}", x, y, z, w);
    // ===== Arrays
    // In rust arrays always have fixed length and it stores always same variable types
    let array: [i32; 4] = [1, 2, 3, 4];// the type of array is defined by [type; number of elements]
    println!("{:?}", array);
    let rarray = [1;50]; // it will create an array with 50 repeated values equals to 1
    println!("{:?}", rarray);
    // To get a specific value from array, you must use the [] syntax
    println!("value at position 2 is equal to {}", array[2]);
    // If you try to get a value from array using an invalid index, Rust will immediatly halt the
    // code execution
    //println!("{}", array[4]); // error: index out of bounds: the length is 4 but the index is 4
    // ***** FUNCTIONS *****
    // you must put the keyword "fn" before the name of your function
    /*
    fn fooBar() { // help: convert the identifier to snake case:
        "foobar"; // the keyword return is optional
    }
    */
    /*
    // if you don't use the function, Rust will generate a warning message
    fn foo_bar() { // warning: function is never used: `foo_bar`
        "foorbar";
    }
    */
    fn foo_bar() {
        "foobar";
    }
    //println!(foo_bar());// error: format argument must be a string literal
    //println!("{}", foo_bar()); // error:  `()` doesn't implement `std::fmt::Display`
    // foo_bar() don't return the string, but it returns a empty tuple ()
    println!("{:?}", foo_bar());
    // you can specify the return of the function using ->
    fn foo() -> u8 {
        42
    }
    println!("{}", foo());
    /*
    fn spam() -> u8 {
        325 // error:  literal out of range for `u8`
    }
    */
    // ok, but if you want create a function that returns a string
    /*
    fn foo_bar_() -> &str{ // error: missing lifetime specifier
        "foobar"
    }
    */
    // let's try again
    /*
    fn foo_bar_() ->&'static str { // error: mismatched types, expected `&str`, found `()`
        "foobar";// if you put semicolon the function will return a empty tuple ()
    }
    */
    // Finally!!!
    fn foo_bar_() ->&'static str {
        "foobar"
    }
    println!("{}", foo_bar_());
    // You can do that using the keyword return, with or without semicolon
    fn bar() ->&'static str {
        return "bar" // works!!
    }
    println!("{}", bar());
    // ===== Function Parameters
    // A function can have none or many function parameters
    // the function parameter syntax is var_name1: var_type1, var_name2: var_type2, ...
    /*
    fn sum(x, y) {// error: expected one of `:`, `@`, or `|`, found `)`

    }
    */
    /*
    fn sum(x:i32, y:i16) {
        x + y // error: mismatched types, y <- expected `i32`, found `i16`
    }
    */
    /*
    fn sum(x:i32, y:i32) {
        x + y// error:  mismatched types, help: try adding a return type: `-> i32`
    }
    */
    /*
    fn sum(x:i32, y:i32) {
        return x + y // error:  mismatched types, help: try adding a return type: `-> i32`
    }
    */
    // Finally, the correct syntax is
    fn sum(x:i32, y:i32) -> i32 {
        x + y
    }
    println!("2 + 2 = {}", sum(2, 2));
    // But if you need to sum i32 with i16
    fn sum_(x:i32, y:i16) -> i32{
        x + i32::from(y)
    }
    println!("4 + 4 = {}", sum_(4, 4));
    // ===== Function Bodies Contain Statements and Expressions
    // Statements performs some instructions and don't return a value
    // Expression return a value
    let k = 42; // it is a statement
    //let k = (let k = 52); // error: expected expression, found statement (`let`)
    // In Rust, 1 + 1, calling a function, calling a macro, {} are expressions
    let k = {// this return an expression
        let k = 1;
        k + 10 // remember, if I put the semicolon here, it will returns an empty tuple ()
    };
    // Expression don't include ending semicolon symbol
    // If you do that, you will transform the expression in a statement
    println!("{}", k);
    // ***** CONTROL FLOW *****
    // if expression
    // With "if expression" you can choose which instruction will be used considering
    // a logical expression
    let number = 5;
    /*
    if (number < 0) { // warning: unnecessary parentheses around `if` condition
    }
    */
    //if (number < 0) println!("heelooo"); // error: expected `{`, found `println`
    if number < 0 {
        println!("{} is lower than 0", number);
    } else if number > 0 {
        println!("{} is higher than 0", number);
    } else {
        println!("{} is equal to 0", number);
    }
    // like other programming languages one can use if else, else if expressions
    // You can use if to initialize an immutable and mutable variables; and constants
    let _positive = if number < 0 { false } else { true };
    let mut _negative = if number < 0 { true } else { false };
    // but it's allowed use only constants in if expression
    //const ZERO:bool = if number == 0 { true } else { false };// error: attempt to use a non-constant value in a constant
    const _NUMBER:i8 = 5;
    const _ZERO:bool = if _NUMBER == 0 { true } else { false };
    //if number { true }// expected `bool`, found integer
    //if bool::from(number) { true } // error:  mismatched types, expected `bool`, found integer
    //let k = if number < 0 { false } else { "string" }; // error: `if` and `else` have incompatible types
    // ===== Repetition with Loops
    // In Rust, it have three kinds of loop: loop, while and for
    let mut k = 0;
    loop {
        println!("{}", k);
        if k > 10 { break; } // like other programming languages rust have the keywords break and continue
        k = k + 1;

    }
    //let expr = loop { "string" };// error: mismatched types, expected `()`, found `&str`
    //let expr = loop { "string"; };// error: warning: unreachable statement, any code following
    // this expression is unreachable 
    let expr = loop { break "string" };// yes, you can do that
    // loop is a expression, when you use the break keyword it's look like a return keyword
    println!("{}", expr);
    let mut aux = 2;
    let expr = loop {aux *= 2; if aux > 1024 { break aux }};// hehehheheheheheheh
    println!("{}", expr);
    // ===== While
    // It works like a loop, but you must check a logical expression before execute the loop
    let mut counter = 0;
    while counter < 4096 {
        //println!("{}", counter += 1); // error: `()` doesn't implement `std::fmt::Display`
        // counter += 1 is a statement
        println!("{}", { counter + 1 });
        counter += 1;
    }
    // ===== For
    /*
    for (let mut i = 0; i < 10; i += i) { // Sorry, you can't do that :)
    }
    */
    for i in 1..10 { // 1..10 - i start with 1 and finish with 9
        print!("i:{} ", i);
    }
    println!();
    // but if you want iterate until the last value use the sintax 1..=10
    for j in 1..=10 {
        print!("j:{} ", j);
    }
    println!();
    // ===== Looping Throught a Collection with for
    let ar = [1; 10];
    for i in ar { // "in ar": in previous versions of Rust you can't do this,
                      // you had to do "in ar.iter()"
        print!("i->{} ", i);
    }
    println!();
    // there is a method to support you create reverse lists
    //for i in ar.reverse() { // remember! you can't reverse a immutable variable!
    //let mut rev = ar.reverse();// this return a statement (empty tuple) not a expression    
    //for i in rev {// error: `()` is not an iterator 
    let mut rev = [2, 4, 8, 16, 32];
    rev.reverse();
    for i in rev {
        print!("{} ", i);
    }
    // output: 32 16 8 4 2
    // and about a range?
    let r = 1..10;
    //for i in r.reverse() { // error: no method named `reverse` found for struct `std::ops::Range<{integer}>` in the current scope
    for i in r.rev() {// there are different methos to create reverse iterators
        print!("{} ", i);
    }
    // output: 9 8 7 6 5 4 3 2 1
    // ***** UNDERSTANDING OWNERSHIP *****
    // The way that memory is managed in Rust is through 
    // a system of ownership with a set of rules that the compiler checks at compile 
    // time.
    // The rules of ownership are:
    // 1. Each value in Rust has its owner (variable).
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    // ===== What is variable scope? =====
    // {} <- this is a Rust variable scope, and a variable only survive inside a scope
    /*
    {
        let s = "string";
    }
    println!("{}", s); // error: cannot find value `s` in this scope
    */
    {
        //println!("{}", s); // error: cannot find value `s` in this scope
        let s = "string";// ok, now the variable exists in this scope
        println!("{}", s);
    }
    // output: string
    // ===== The String Type
    // unlike "primitive" data type, the String type (capital S) is stored in heap, so, Rust have
    // to concern about memory releasing of this String.
    // literal strings (lower s) are immutable, so, in Rust, strings aren't suitable for all
    // situations.
    // For example, if you need to get a value from default input, you must use
    // a String
    //let str = String::from("Hello Rust programmers!");
    //println!("{}", str);
    // output: Hello Rust programmers!
    //str.push_str("!!");
    let mut str = "Hello Rust programmers from the stack!";// warning: variable does not need to be mutable
    // str = str + "hello";// there is no way to concat literal strings and return a literal string without use the type String
    // there is no method to change the content of this variable
    // so there is no sense to make a string (lower s) as mutable
    println!("{}", str);
    // output: Hello Rust programmers from the stack!
    let str = String::from(str);
    let str = str.replace("stack","heap");
    println!("{}", str);
    // output: Hello Rust programmers from the stack!
    let str2 = str; // the ownsership is transfered from str to str2
    //println!("{}", str);// error: borrow of moved value: `str`
    // str doesn't point to valid memory address anymore, it is invalidated!!!
    println!("{}", str2);
    // if you want copy the the content of str2 from stack and heap you can use
    // the method clone
    let mut str3 = str2.clone();
    println!("{}", str3);
    // In Rust some types implements by default a techinique called copy traits
    // We will talk more about traits later, but when you have a type with values
    // stored only in the stack, the Rust language implements automatically
    // the copy trait, so, there is no problem when you do something like this:
    // let x = 1; let y = x;
    // The types that implements by default the copy traits are: Integers (i8, i16 ..),
    // Floats, Booleans, char and tuple of types that implements the copy traits too.
    // ===== Ownership and Functions
    // The rules are the same to assignment when you pass a variable to a function
    move_(str3);
    //println!("{}", str3);// error: borrow of moved value: `str3`
    //move_(str3); error: expected `&String`, found struct `String`
    copy_(x);// x stores in the stack, x implements copy traits
    println!("{}", x);// no problem!!
    //==== Return Values and Scope
    // When your function returns a value, it can move the ownership of the variable
    let mut str4 = String::from("Hello!!!");
    str4 = transfer(str4);
    println!("{}", str4);
    // It can be very tedious if you pass many variables
    // Rust deals this problem with references
    //===== References and Borrowing
    // if you can't move the ownership of a variable to the function parameter
    // you must use references &
    reference(&str4);
    println!("{}", str4);// now, str4 remain with the ownership of the memory address
/*
you can't chage the value of a variable passed as reference
fn change_reference_value(str: &String) {
    str.push_str("crash");// error[E0596]: cannot borrow `*str` as mutable, as it is behind a `&` reference
}
*/
// But you can change the parameter using mutable references :)
    mutable_reference(&mut str4);// the 
    reference(&str4);
// and about primitive types?
/*
    let k = 1;
    increment(&mut k);// cannot borrow `k` as mutable, as it is not declared as mutable
*/
    let mut k = 1;
    increment(&mut k);
    println!("increment:{}", k);
// but to prevents memory leak, you can't share mutable references :)
    let y = &mut k;// this is same as let y:&mut i32 = &mut k;
    //k = 3;// error: cannot assign to `k` because it is borrowed
    //hahahahha, you can't change the value of k after create a mutable reference to k
    //let z = &mut k;// error:: cannot borrow `k` as mutable more than once at a time
    println!("k->y:{}", y);
    let a:&mut i32 = &mut 42;// yes.. this is valid!!!
    println!("a:{}", a);
    // but if you want multiples references, you can create a new scope
    {
        let z = &mut k;// now, this is valid!!
        println!("z:{}", z);
    }
    // and about immutable references, can you create more than one?
    let k = 42;
    let y = &k;// no problem
    let z = &k;// no problem
    //let x = &mut k; // error: cannot borrow `k` as mutable, as it is not declared as mutable

}

fn increment(x: &mut i32) {
    *x = *x + 1;
}

fn mutable_reference(str: &mut String) {
    str.push_str("references");
}

fn reference(str: &String) {
    println!("reference:{}", str);
}

fn transfer(str: String) -> String {
    println!("transfer:{}", str);
    str
}

fn move_(str: String) {
    println!("{}", str);
}

fn copy_(x: i32) {
    println!("{}", x);
}