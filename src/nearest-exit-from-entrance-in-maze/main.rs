use std::collections::VecDeque;

struct TestCase {
    maze: Maze,
    entrance:Position,
    expected: i32
}

impl TestCase {
    pub fn run(self) {
        println!("Running test case...");
        let e = vec![self.entrance[0], self.entrance[1]];
        let result = nearest_exit(self.maze, e);
        println!("EXPECTED: {}, {}", self.expected, result);
        assert_eq!(self.expected, result);
    }
}

type Position = Vec<i32>;
type Maze = Vec<Vec<char>>;
type Explored = Vec<Vec<bool>>;

#[derive(Eq,PartialEq)]
struct W<T>(T);

impl From<W<&Position>> for (i32,i32) {
    fn from(val: W<&Position>) -> Self {
        (*val.0.first().unwrap(), *val.0.get(1).unwrap())
    }
}

fn get_maze_dimensions(maze: &Maze) -> (usize, usize) {
    (maze.len(), maze.get(0).expect("Maze does not contain any cell").len())
}
fn in_maze(maze: &Maze, current: &(i32,i32)) -> bool {
    let dim = get_maze_dimensions(maze);
    let (r,c) = *current;
    0<=r && r<dim.0 as i32 && 0<=c && c<dim.1 as i32
}
fn is_border(maze: &Maze, current: &Position) -> bool {
    let dim = get_maze_dimensions(maze);
    let (cx,cy) = W(current).into();
    cx == 0 || cy == 0 || cx as usize == dim.0-1 || cy as usize == dim.1-1
}
fn is_road(maze: &Maze, current: &Position) -> bool {
    let (x,y) = W(current).into();
    maze[x as usize][y as usize] == '.'
}
fn is_exit(maze: &Maze, entrance: &Position, current: &Position) -> bool {
    is_road(maze, current) && is_border(maze, current) && entrance != current
}
fn next(maze: &Maze, explored: &Explored, current: &Position) -> Vec<Position> {
    let (r,c) = W(current).into();
    [(1,0),(0,1),(-1,0),(0,-1)]
        .into_iter()
        .map(|(dx, dy)| (i32::overflowing_add(dx,r), i32::overflowing_add(dy,c)))
        .filter_map(|(r,c)| if !r.1 && !c.1 && in_maze(maze, &(r.0,c.0) ) {
            Some(vec![r.0, c.0])
        } else {
            None
        })
        .filter(|p| is_road(maze, p) && !is_explored(explored, p))
        .collect::<Vec<Position>>()
}
fn explore(explored: &mut Explored, current: &Position) {
    let (cx,cy) = W(current).into();
    explored[cx as usize][cy as usize] = true;
}
fn is_explored(explored: &Explored, current: &Position) -> bool {
    let (cx,cy) = W(current).into();
    explored[cx as usize][cy as usize]
}
fn nearest_exit(maze: Maze, entrance: Vec<i32>) -> i32 {
    let dim = get_maze_dimensions(&maze);
    let mut explored = vec![ vec![false; dim.1]; dim.0];
    let mut queue = VecDeque::<W<(Position, u32)>>::new();
    let entrance = vec![entrance[0],entrance[1]];
    queue.push_back(W((entrance.clone(), 0u32)));

    while let Some(W((node, cost))) = queue.pop_front() {
        let next_nodes = next(&maze, &explored, &node);
        for candidate in next_nodes {
            if is_exit(&maze, &entrance, &candidate) {
                return cost as i32 + 1;
            }
            explore(&mut explored, &candidate);
            queue.push_back(W((candidate, cost+1)));
        }
    }
    -1
}

fn main() {
    let test_case = TestCase {
        maze: vec![ vec!['+','+','.','+'], 
                    vec!['.','.','.','+'], 
                    vec!['+','+','+','.']],
        entrance: vec![1,2],
        expected: 1
    };
    test_case.run();

    let test_case = TestCase{
        maze: vec![
            vec!['+','.','+','+','+','+','+'],
            vec!['+','.','+','.','.','.','+'],
            vec!['+','.','+','.','+','.','+'],
            vec!['+','.','.','.','+','.','+'],
            vec!['+','+','+','+','+','.','+']],
        entrance: vec![0,1],
        expected: 12
    };
    test_case.run();

    let test_case = TestCase{
        maze: vec![
        vec!['.','.','.','.','.','+','.','.','.'],
        vec!['.','+','.','.','.','.','.','.','.'],
        vec!['.','.','+','.','+','.','+','.','+'],
        vec!['.','.','.','.','+','.','.','.','.'],
        vec!['.','.','.','.','+','+','.','.','.'],
        vec!['+','.','.','.','.','.','.','.','.'],
        vec!['.','.','.','+','.','.','.','.','.'],
        vec!['.','.','.','+','.','.','.','.','+'],
        vec!['+','.','.','+','.','+','+','.','.']],
        entrance: vec![8,4],
        expected: 5,
    };
    test_case.run();
}
