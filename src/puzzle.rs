use crate::config::SOLVE_OUTPUT_ENABLED;
use ahash::AHashSet;
use cached::proc_macro::cached;
use core::fmt;
use counter::Counter;

pub const SIZE: usize = 9;
pub const BOX_SIZE: usize = 3;

type NumSet = AHashSet<u8>;
type IndexSet = AHashSet<(usize, usize)>;
type Grid = [[u8; SIZE]; SIZE];
type Candidates = [[NumSet; SIZE]; SIZE];

pub struct Puzzle {
    grid: Grid,
    pub candidates: Candidates,
}

#[cached]
fn get_box_base_index(x: usize, y: usize) -> (usize, usize) {
    (x - x % BOX_SIZE, y - y % BOX_SIZE)
}

#[cached]
fn get_rcb_indices(x: usize, y: usize) -> IndexSet {
    let row_indices = get_row_indices(y);
    let column_indices = get_column_indices(x);
    let box_indices = get_box_indices(x, y);

    let rowcol = &row_indices | &column_indices;

    &rowcol | &box_indices
}

#[cached]
fn get_row_indices(y: usize) -> IndexSet {
    (0..SIZE).map(|x| (x, y)).collect()
}

#[cached]
fn get_column_indices(x: usize) -> IndexSet {
    (0..SIZE).map(|y| (x, y)).collect()
}

#[cached]
fn get_box_indices(x: usize, y: usize) -> IndexSet {
    let (box_x, box_y) = get_box_base_index(x, y);

    (box_x..box_x + BOX_SIZE)
        .flat_map(|i| (box_y..box_y + BOX_SIZE).map(move |j| (i, j)))
        .collect()
}

#[cached]
pub fn get_all_group_indices() -> Vec<IndexSet> {
    let all_row_indices = get_all_row_indices();
    let all_column_indices = get_all_column_indices();
    let all_box_indices = get_all_box_indices();

    [all_row_indices, all_column_indices, all_box_indices].concat()
}

#[cached]
fn get_all_row_indices() -> Vec<IndexSet> {
    let mut result: Vec<IndexSet> = Vec::new();
    for y in 0..SIZE {
        result.push(get_row_indices(y));
    }

    result
}

#[cached]
fn get_all_column_indices() -> Vec<IndexSet> {
    let mut result: Vec<IndexSet> = Vec::new();
    for x in 0..SIZE {
        result.push(get_column_indices(x));
    }

    result
}

#[cached]
fn get_all_box_indices() -> Vec<IndexSet> {
    let mut result: Vec<IndexSet> = Vec::new();
    for y in (0..SIZE).step_by(BOX_SIZE) {
        for x in (0..SIZE).step_by(BOX_SIZE) {
            result.push(get_box_indices(x, y));
        }
    }

    result
}

impl Puzzle {
    fn new(grid: Grid) -> Self {
        let mut candidates: Candidates = [(); SIZE].map(|_| [(); SIZE].map(|_| AHashSet::new()));
        for y in 0..SIZE {
            for x in 0..SIZE {
                if grid[y][x] == 0 {
                    candidates[y][x] = Puzzle::get_candidates_for_cell(grid, x, y);
                }
            }
        }

        Self { grid, candidates }
    }

    pub fn from_string(puzzle_string: &String) -> Self {
        assert_eq!(puzzle_string.len(), SIZE * SIZE);
        let mut grid: Grid = [[0; SIZE]; SIZE];
        for (i, value) in puzzle_string.chars().enumerate() {
            let x = i % SIZE;
            let y = i / SIZE;
            grid[y][x] = value.to_digit(10).expect("Not a digit!") as u8;
        }

        Puzzle::new(grid)
    }

    pub fn check_if_solved(&self) -> bool {
        for row in self.grid {
            if row.iter().any(|&x| x == 0) {
                return false;
            }
        }

        true
    }

    fn get_candidates_for_cell(grid: Grid, x: usize, y: usize) -> NumSet {
        let all_values: NumSet = (0..(SIZE + 1) as u8).into_iter().collect();
        let rcb: NumSet = Puzzle::get_rcb(grid, x, y);

        &all_values - &rcb
    }

    fn get_rcb(grid: Grid, x: usize, y: usize) -> NumSet {
        let mut rcb_values = NumSet::new();
        for (i, j) in get_rcb_indices(x, y) {
            rcb_values.insert(grid[j][i]);
        }

        rcb_values
    }

    pub fn assign_value_to_cell(&mut self, value: u8, x: usize, y: usize) {
        if SOLVE_OUTPUT_ENABLED {
            println!("  found {} at position {}, {}", value, x, y);
        }

        self.grid[y][x] = value;
        self.remove_candidate_from_rcb(value, x, y);
        self.candidates[y][x] = NumSet::new();
    }

    fn remove_candidate_from_rcb(&mut self, value: u8, x: usize, y: usize) {
        for (i, j) in get_rcb_indices(x, y) {
            self.candidates[j][i].remove(&value);
        }
    }

    pub fn get_candidates_counter(&self, group: &IndexSet) -> Counter<u8> {
        let mut counter = Counter::new();
        for (x, y) in group {
            counter.update(self.candidates[*y][*x].clone());
        }

        counter
    }

    pub fn get_candidates_indices_by_value(&self, value: u8, group: &IndexSet) -> IndexSet {
        let mut result = IndexSet::new();
        for (x, y) in group {
            if self.candidates[*y][*x].contains(&value) {
                result.insert((*x, *y));
            }
        }

        result
    }

    fn get_values_by_group_indices(&self, group_indices: Vec<IndexSet>) -> Vec<NumSet> {
        let mut result: Vec<NumSet> = Vec::new();
        for group in group_indices {
            let mut numset = NumSet::new();
            for (x, y) in group {
                numset.insert(self.grid[y][x]);
            }
            result.push(numset);
        }

        result
    }

    fn get_all_rows(&self) -> Vec<NumSet> {
        self.get_values_by_group_indices(get_all_row_indices())
    }

    fn get_all_columns(&self) -> Vec<NumSet> {
        self.get_values_by_group_indices(get_all_column_indices())
    }

    fn get_all_boxes(&self) -> Vec<NumSet> {
        self.get_values_by_group_indices(get_all_box_indices())
    }

    pub fn validate_solution(&self) -> bool {
        let all_groups: Vec<NumSet> = [
            self.get_all_rows(),
            self.get_all_columns(),
            self.get_all_boxes(),
        ]
        .concat();

        all_groups.iter().all(|x| x.len() == SIZE)
    }
}

impl fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n").expect("Couldn't write newline!");
        for y in 0..SIZE {
            for x in 0..SIZE {
                write!(f, "{} ", self.grid[y][x]).expect("Couldn't write a cell");
            }
            write!(f, "\n").expect("Couldn't write newline!");
        }

        write!(f, "")
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Hello, you are running the puzzle module!");
    let pstring: String =
        "030072001000030090518000003050203100000705306000640205200060014007000630000008900"
            .to_string();

    let my_puzzle = Puzzle::from_string(&pstring);

    dbg!(my_puzzle.candidates);
}
