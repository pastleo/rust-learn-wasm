use std::mem;

fn main() {
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
}
