#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    fn area(&mut self) -> u32 {
        self.width = 12;
        self.width * self.height
    }
    fn reactangle(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let mut ret1 = Reactangle {
        width: 32,
        height: 20,
    };

    let ret2 = Reactangle::reactangle(12, 13);
    println!("{}", ret1.area());
}
