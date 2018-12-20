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
}