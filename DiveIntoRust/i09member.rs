trait Shape {
    fn area(self: &Self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    // Self 类型就是 Circle
    // self 的类型是 &Self,即 &Circle
    fn area(&self) -> f64 {
        // 访问成员变量,需要用 self.radius
        std::f64::consts::PI * self.radius * self.radius
    }
}
fn main() {
    let c = Circle { radius : 2f64};
    // 第一个参数名字是 self,可以使用小数点语法调用
    println!("The area is {}", c.area());
}
