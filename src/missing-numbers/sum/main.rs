struct TestCase {
    numbers: Vec<i32>,
    missing: i32
}

impl TestCase {
    pub fn run(self) {
        assert_eq!(missing_numbers(self.numbers),self.missing);
    }
}

fn missing_numbers(numbers: Vec<i32>) -> i32 {
    let size = numbers.len();
    let expected = (size * (size + 1) / 2) as i32;
    let sum : i32 = numbers.into_iter().sum();
    expected-sum
}

fn main() {
    TestCase{numbers: vec![0,1,2,4,5,6], missing: 3 }.run()
}