struct TestCase {
    numbers: Vec<i32>,
    missing: i32
}

impl TestCase {
    pub fn run(self) {
        assert_eq!(missing_numbers(self.numbers),self.missing);
    }
}

fn missing_numbers(nums: Vec<i32>) -> i32 {
    let size = nums.len() + 1;
    if size == 1 {
        return 0;
    }
    let mut found = vec![false; size];
    nums.into_iter().for_each(|v| found[v as usize] = true);
    let v : Vec<usize> = found.into_iter().enumerate().filter_map(|(i, v)| if !v { Some(i)} else {None}).collect();
    (*v.first().unwrap()) as i32
}

fn main() {
    TestCase{numbers: vec![0,1,2,4,5,6], missing: 3 }.run()
}