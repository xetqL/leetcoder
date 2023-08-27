use std::collections::{HashMap, VecDeque};

struct TestCase {
    heights: Vec<i32>,
    expected: Vec<i32>
}

impl TestCase {
    pub fn run(self) {
        assert_eq!(can_see_persons_count(self.heights),self.expected);
    }
}
#[derive(Eq,PartialEq)]
struct W(usize,i32);
impl Ord for W {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.1 < other.1 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}
impl PartialOrd for W {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

fn can_see_persons_count(mut heights: Vec<i32>) -> Vec<i32> {
    let mut sight : Vec<i32> = (0i32..heights.len() as i32).rev().collect();
    let mut stack: Vec<i32> = vec![heights.pop().unwrap()];
    sight[heights.len()] = 0;
    for (lh_idx, lh_height) in heights.iter().enumerate().rev() {
        let size = stack.len();        
        let idx = stack.iter().rposition(|x| lh_height < x).map(|v| v as i32).unwrap_or(-1) + 1;
        stack.truncate(idx as usize);
        let mut c = size - stack.len();
        if !stack.is_empty() {
            c+=1;
        }
        sight[lh_idx] = c as i32;
        stack.push(*lh_height);
    }
    sight
}

fn main() {
    TestCase{ heights: [10,6,8,5,11,9].into(), expected: [3,1,2,1,1,0].into() }.run();
}