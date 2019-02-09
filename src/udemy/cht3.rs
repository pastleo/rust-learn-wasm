pub fn sec_3_12_if() {
    println!(">> udemy::3-12: if");
    let score:u16 = rand::random::<u16>() % 100;
    let result =
        if score > 60 { "pass" }
        else if score == 60 { "just pass" }
        else { "failed" };
    println!("score: {}, {}!", score, result);
}

pub fn sec_3_13_while_loop() {
    println!(">> udemy::3-13: while and loop");

    let mut divider:u16;
    let mut number:u16 = rand::random::<u16>() % 100;
    let target:u16 = rand::random::<u16>() % 100 + number;

    print!("from {} to {}, prime numbers: ", number, target);
    
    while number <= target {
        divider = 2;
        loop {
            if number % divider == 0 {
                break; // also, there is continue
            } else if divider * divider > number {
                print!("{} ", number);
                break;
            }
            divider += 1;
        }

        number += 1;
    }

    println!("");
}
