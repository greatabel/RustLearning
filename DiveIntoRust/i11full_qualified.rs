trait Cook {
    fn start(&self);
}

trait Wash {
    fn start(&self);
}

struct Chef;
impl Cook for Chef {
    fn start(&self) { println!("Cook::start");}
}
impl Wash for Chef {
    fn start(&self) { println!("Wash::start");}
}
// fn main() {
//     let me = Chef;
//     me.start();
// }
fn main() {
    let me = Chef;
// 函数名字使用更完整的path来指定,同时,self参数需要显式传递
    <Cook>::start(&me);
    <Chef as Wash>::start(&me);
}
