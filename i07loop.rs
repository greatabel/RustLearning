fn main() {
    let mut count = 0u32;
    println!("Let's count until infinity!");
    // 无限循环
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // 不再继续执行后面的代码,跳转到loop开头继续循环
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            // 跳出循环
            break;
        }
    }

        // A counter variable
    let mut n = 1;
    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        // Increment counter
        n += 1;
    }

}