type Age = u32;

fn grow(age: Age, year: u32) -> Age {
    age + year
}

fn main() {
    let x: Age = 20;

    println!("20 years later: {:?}", grow(x, 20));

    //局部变量声明,可以留待后面初始化,只要保证使用前已经初始化即可
    let x;
    let y = 1_i32;
    x = 2_i32;
    println!("{} {}", x, y);

//全局变量必须声明的时候初始化,因为全局变量可以写到函数外面,被任意一个函数使用
    static G1 : i32 = 3;
    println!("{}", G1);

//可变全局变量无论读写都必须用 unsafe修饰
    static mut G2 : i32 = 4;
    unsafe {
        G2 = 5;
        println!("{}", G2);
    }

//全局变量的内存不是分配在当前函数栈上,函数退出的时候,并不会销毁全局变量占用的内存空间,程序退出才会回收”
    let x = true;
    let y: bool = !x;      // 取反运算

    let z = x && y;        // 逻辑与,带短路功能
    println!("{}", z);

    let z = x || y;        // 逻辑或,带短路功能
    println!("{}", z);

    let z = x & y;         // 按位与,不带短路功能
    println!("{}", z);

    let z = x | y;         // 按位或,不带短路功能
    println!("{}", z);

    let z = x ^ y;         // 按位异或,不带短路功能
    println!("{}", z);



}