struct T(i32);
impl T {
    // 这是一个静态方法
    fn func(this: &Self) {
        println!{"value {}", this.0};
    }
}
fn main() {
    let x = T(42);
    // x.func(); 小数点方式调用是不合法的
    T::func(&x);
}
