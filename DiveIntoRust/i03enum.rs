enum Number {
    Int(i32),
    Float(f32),
}

fn read_num(num: &Number) {
    match num {
        // 如果匹配到了 Number::Int 这个成员,那么value的类型就是 i32
        &Number::Int(value) => println!("Integer {}", value),
        // 如果匹配到了 Number::Float 这个成员,那么value的类型就是 f32
        &Number::Float(value) => println!("Float {}", value),
    }
}

fn main() {
    let n: Number = Number::Int(10);
    read_num(&n);
}
