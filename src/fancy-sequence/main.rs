
struct TestCase<'a> {
    sequence: &'a[&'a str],
    values: &'a [i32],
    expected: &'a str,
}

impl TestCase<'_> {
    fn run(mut self) {
        let mut fancy = Fancy::new();

        for (cmd, val) in self.sequence.into_iter().skip(1).zip(self.values.into_iter().skip(1)) {
            print!("{cmd}: ");
            match cmd {
                &"fancy" => print!(""),
                &"append" => fancy.append(*val),
                &"addAll" => fancy.add_all(*val),
                &"multAll" => fancy.mult_all(*val),
                &"getIndex" => print!("{}", fancy.get_index(*val)),
                v => panic!("{v}")
            }
            println!();
        }
    }
}

#[derive(Default)]
struct Fancy {
    modulus: i32,
    data: Vec<LazyCalc>
}

#[derive(Default)]
struct LazyCalc {
    v: i32,
    calc: Vec<Box<dyn Fn(i32) -> i32>>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {

    pub fn new() -> Self {
        Self {
            modulus: i32::pow(10, 9) + 7,
            ..Self::default()
        }
    }
    
    pub fn append(&mut self, val: i32) {
        self.data.push(LazyCalc { v: val, calc: Vec::default() })
    }
    
    pub fn add_all(&mut self, inc: i32) {
        let modulus = self.modulus;
        self.data.iter_mut().for_each(|v| v.calc.push(Box::new(move |v| i32::rem_euclid(i32::rem_euclid(v, modulus) * i32::rem_euclid(inc, modulus), modulus))));
    }
    
    pub fn mult_all(&mut self, m: i32) {    
        let modulus = self.modulus;
        self.data.iter_mut().for_each(|v| v.calc.push(Box::new(move |v| i32::rem_euclid(i32::rem_euclid(v, modulus) * i32::rem_euclid(m, modulus), modulus))));
    }
    
    pub fn get_index(&mut self, idx: i32) -> i32 {
        let idx = idx as usize;

        if idx >= self.data.len() { -1 } else { 
            let lazy = &mut self.data[idx];
            for func in &lazy.calc {
                lazy.v = func(lazy.v);
            }
            lazy.calc.clear();
            lazy.v.rem_euclid(i32::pow(10, 9) + 7)
        }
    }
}

fn main() {
    println!("{}", i32::rem_euclid(-1, 5));
    let tc = TestCase{
        sequence: &["Fancy","append","append","getIndex","append","getIndex","addAll","append","getIndex","getIndex","append","append","getIndex","append","getIndex","append","getIndex","append","getIndex","multAll","addAll","getIndex","append","addAll","getIndex","multAll","getIndex","multAll","addAll","addAll","append","multAll","append","append","append","multAll","getIndex","multAll","multAll","multAll","getIndex","addAll","append","multAll","addAll","addAll","multAll","addAll","addAll","append","append","getIndex"],
        values: &[12,8,1,12,0,12,8,2,2,4,13,4,12,6,11,1,10,2,3,1,6,14,5,6,12,3,12,15,6,7,8,13,15,15,10,9,12,12,9,9,9,9,4,8,11,15,9,1,4,10,9],
        expected: "",
    };
    tc.run();
}