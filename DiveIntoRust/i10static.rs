struct T(i32);
impl T {
    // 这是一个静态方法
    fn func(this: &Self) {
        println!{"value {}", this.0};
    }
}


trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> i32 { *self * 2 }
}


fn main() {
    let x = T(42);
    // x.func(); 小数点方式调用是不合法的
    T::func(&x);

    // 可以像成员方法一样调用
    let x1 : i32 = 10.double();
    println!("{}", x1);

}
