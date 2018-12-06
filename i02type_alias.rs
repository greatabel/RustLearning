type Age = u32;

fn grow(age: Age, year: u32) -> Age {
    age + year
}

fn main() {
    let x: Age = 20;

    println!("20 years later: {:?}", grow(x, 20));


}