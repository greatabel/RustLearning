struct T(usize);
impl T {
    fn get1(&self) -> usize {self.0}
    fn get2(&self) -> usize {self.0}
}
fn get3(t: &T) -> usize { t.0 }
fn check_type( _ : fn(&T)->usize ) {}

fn main() {
    check_type(T::get1);
    check_type(T::get2);
    check_type(get3);
}
