// hello world
/* 
hello world
*/
fn main() {
    let s = "Hello world!";
    println!("{}", s);

    println!("{}", 1);            // 默认用法,打印Display
    println!("{:o}", 9);          // 八进制
    println!("{:x}", 255);        // 十六进制 小写
    println!("{:X}", 255);        // 十六进制 大写
    println!("{:p}", &0);         // 指针
    println!("{:b}", 15);         // 二进制
    println!("{:e}", 10000f32);   // 科学计数(小写)
    println!("{:E}", 10000f32);   // 科学计数(大写)

    println!("{:?}", "test");     // 打印Debug
    println!("{:#?}", ("test1", "test2"));        // 带换行和缩进的Debug打印

    println!("{a} {b} {b}", a = "x", b = "y");   // 命名参数

    //变量声明
    let variable: i32 = 100;
    let mut x = 5;
    x = 10;
    let (mut a, mut b) = (1, 2);

    let x = "hello";
    println!("x is {}", x);
    let x = 5;
    println!("x is {}", x);
}