use std::mem;

pub fn sec_2_7_core_data_types() {
    println!(">> udemy::2-7: core data types");
    let mut a:u8 = 123;
    println!("a:u8 = {}, size = {}", a, mem::size_of_val(&a));
    a = 234;
    println!("a:u8 = {}", a);

    let mut b = 1234;
    println!("b = {}, size = {}", b, mem::size_of_val(&b));
    b = -1;
    println!("b = {}", b);

    let c:isize = 1234;
    println!("c:isize = {}, size = {}", c, mem::size_of_val(&c));

    let d = 'p';
    println!("d = '{}', size = {}", d, mem::size_of_val(&d));

    let e = 3.14;
    println!("e = {}, size = {}", e, mem::size_of_val(&e));

    let f = true;
    println!("f = {}, size = {}", f, mem::size_of_val(&f));

    println!("");
}

pub fn sec_2_8_operators() {
    println!(">> udemy::2-8: operators");

    let mut a = 2 + 3 * 4;
    println!("a = 2 + 3 * 4 = {}", a);

    a += 2;
    println!("a += 2, a = {}", a);

    println!("a % 3 = {}", a % 3);

    println!("a^3 (X), i32::pow(a, 3) = {}", i32::pow(a, 3));

    println!("f64::powi(2.5, 3) = {}", f64::powi(2.5, 3));
    println!("f64::powf(2.5, std::f64::consts::PI) = {}", f64::powf(2.5, std::f64::consts::PI));

    let b:i64 = 1;
    println!("b:i64 = {}", b);
    println!("bitwise, OR: b | 2 = {}, AND: b & 2 = {}, XOR: b ^ 2 = {}, NOT: !b = {}", b | 2, b & 2, b ^ 2, !b);
    println!("bitwise, shitf: b << 5 = {}", b << 5);

    println!("3 > 2 = {}", 3 > 2);
    println!("true || false = {}", true || false);

    println!("");
}

pub fn sec_2_9_scope_shadowing() {
    println!(">> udemy::2-9: scope and shadowing");

    let a = 123;
    println!("a = {}", a);

    let a = 1234;
    println!("redeclared a = {}", a);

    {
        let b = 123;
        println!("inside b = {}", b);

        let a = 12345;
        println!("inside and redeclared a = {}", a);
    }

    // println!("inside b = {}", b); // cannot
    println!("outside a = {}", a);

    println!("");
}

const GLOBAL_CONST:u8 = 123;
static GLOBAL_STATIC:i32 = 321;
static mut GLOBAL_MUT_STATIC:i32 = 111;

pub fn sec_2_10_const() {
    println!(">> udemy::2-10: const");

    println!("GLOBAL_CONST = {}", GLOBAL_CONST);
    println!("GLOBAL_STATIC = {}", GLOBAL_STATIC);

    unsafe {
        println!("inside unsafe block");
        println!("GLOBAL_MUT_STATIC = {}", GLOBAL_MUT_STATIC);
        GLOBAL_MUT_STATIC = 777;
        println!("GLOBAL_MUT_STATIC = {}", GLOBAL_MUT_STATIC);
    }
    println!("");
}

pub fn sec_2_11_stack_heap() {
    println!(">> udemy::2-11: stack heap");

    let x:u8 = 16;
    println!("x:u8 = {}, size = {}", x, mem::size_of_val(&x));

    let y:Box<u8> = Box::new(15); // kind of a pointer, the `15` is store in the heap
    println!("y:Box<u8> = {}, size = {}", *y, mem::size_of_val(&y));

    println!("");
}
