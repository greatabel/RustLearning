fn main() {
    let x = 100;
    let y = 10;
    println!("{} {} {} {} {}", x + y, x - y, x * y, x / y, x % y);

    let num1 : u8 = 0b_1010_1010;
    let num2 : u8 = 0b_1111_0000;
    println!("{:08b}", !num1);
    println!("{:08b}", num1 & num2);
    println!("{:08b}", num1 | num2);
    println!("{:08b}", num1 ^ num2);
    println!("{:08b}", num1 << 4);
    println!("{:08b}", num1 >> 4);

}
