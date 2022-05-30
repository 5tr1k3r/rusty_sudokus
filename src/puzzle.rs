use crate::config::SOLVE_OUTPUT_ENABLED;
use ahash::AHashSet;
use cached::proc_macro::cached;
use core::fmt;
use counter::Counter;
use fixedbitset::FixedBitSet;

pub const SIZE: usize = 9;
pub const BOX_SIZE: usize = 3;

type IndexSet = AHashSet<(usize, usize)>;
type Grid = [[usize; SIZE]; SIZE];
type Candidates = [[FixedBitSet; SIZE]; SIZE];

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
        let mut candidates: Candidates =
            [(); SIZE].map(|_| [(); SIZE].map(|_| FixedBitSet::with_capacity(SIZE + 1)));
        for y in 0..SIZE {
            for x in 0..SIZE {
                if grid[y][x] == 0 {
                    candidates[y][x] = Puzzle::get_candidates_for_cell(grid, x, y);
                }
            }
        }

        Self { grid, candidates }
    }

    pub fn copy(&self) -> Self {
        let mut new_grid: Grid = [[0; SIZE]; SIZE];
        let mut new_candidates: Candidates = 
            [(); SIZE].map(|_| [(); SIZE].map(|_| FixedBitSet::with_capacity(SIZE + 1)));
        
        for y in 0..SIZE {
            for x in 0..SIZE {
                new_grid[y][x] = self.grid[y][x];
                new_candidates[y][x] = self.candidates[y][x].clone();
            }
        }

        Self { grid: new_grid, candidates: new_candidates }
    }

    pub fn from_string(puzzle_string: &str) -> Self {
        assert_eq!(puzzle_string.len(), SIZE * SIZE);
        let mut grid: Grid = [[0; SIZE]; SIZE];
        for (i, value) in puzzle_string.chars().enumerate() {
            let x = i % SIZE;
            let y = i / SIZE;
            grid[y][x] = value.to_digit(10).expect("Not a digit!") as usize;
        }

        Puzzle::new(grid)
    }

    pub fn is_solved(&self) -> bool {
        !self.grid.iter().flatten().any(|&x| x == 0)
    }

    fn get_candidates_for_cell(grid: Grid, x: usize, y: usize) -> FixedBitSet {
        // for SIZE 9: this produces `0111111111` which is a bitset of 1..=9
        let mut all_values: FixedBitSet =
            FixedBitSet::with_capacity_and_blocks(SIZE + 1, [(1 << (SIZE + 1)) - 2]);
        let rcb: FixedBitSet = Puzzle::get_rcb(grid, x, y);
        all_values.difference_with(&rcb);

        all_values
    }

    fn get_rcb(grid: Grid, x: usize, y: usize) -> FixedBitSet {
        let mut rcb_values = FixedBitSet::with_capacity(SIZE + 1);
        for (i, j) in get_rcb_indices(x, y) {
            rcb_values.insert(grid[j][i]);
        }

        rcb_values
    }

    pub fn assign_value_to_cell(&mut self, value: usize, x: usize, y: usize) {
        if SOLVE_OUTPUT_ENABLED {
            println!("  found {} at position {}, {}", value, x, y);
        }

        self.grid[y][x] = value;
        self.remove_candidate_from_rcb(value, x, y);
        self.candidates[y][x].clear();
    }

    fn remove_candidate_from_rcb(&mut self, value: usize, x: usize, y: usize) {
        for (i, j) in get_rcb_indices(x, y) {
            self.candidates[j][i].set(value, false);
        }
    }

    pub fn get_candidates_counter(&self, group: &IndexSet) -> Counter<usize> {
        let mut counter = Counter::new();
        for (x, y) in group {
            counter.update(self.candidates[*y][*x].ones());
        }

        counter
    }

    pub fn get_candidates_indices_by_value(&self, value: usize, group: &IndexSet) -> IndexSet {
        let mut result = IndexSet::new();
        for (x, y) in group {
            if self.candidates[*y][*x].contains(value) {
                result.insert((*x, *y));
            }
        }

        result
    }

    fn get_values_by_group_indices(&self, group_indices: Vec<IndexSet>) -> Vec<FixedBitSet> {
        let mut result: Vec<FixedBitSet> = Vec::new();
        for group in group_indices {
            let mut numset = FixedBitSet::with_capacity(SIZE + 1);
            for (x, y) in group {
                numset.insert(self.grid[y][x]);
            }
            result.push(numset);
        }

        result
    }

    fn get_all_rows(&self) -> Vec<FixedBitSet> {
        self.get_values_by_group_indices(get_all_row_indices())
    }

    fn get_all_columns(&self) -> Vec<FixedBitSet> {
        self.get_values_by_group_indices(get_all_column_indices())
    }

    fn get_all_boxes(&self) -> Vec<FixedBitSet> {
        self.get_values_by_group_indices(get_all_box_indices())
    }

    pub fn validate_solution(&self) -> bool {
        let all_groups: Vec<FixedBitSet> = [
            self.get_all_rows(),
            self.get_all_columns(),
            self.get_all_boxes(),
        ]
        .concat();

        all_groups.iter().all(|x| x.count_ones(..) == SIZE)
    }

    pub fn is_impossible(&self) -> bool {
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.grid[y][x] == 0 && self.candidates[y][x].count_ones(..) == 0 {
                    return true
                }
            }
        }

        false
    }

    pub fn find_cell_with_fewest_candidates(&self) -> (usize, usize) {
        let mut min_cands = SIZE;
        let mut min_x = 0;
        let mut min_y = 0;

        for (y, row) in self.candidates.iter().enumerate() {
            for (x, cands) in row.iter().enumerate() {
                let length = cands.count_ones(..);

                if length != 0 {
                    if length == 2 {
                        return (x, y)
                    }

                    if length < min_cands {
                        min_cands = length;
                        min_x = x;
                        min_y = y;
                    }
                }
            }
        }

        (min_x, min_y)
    }
}

impl fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f).expect("Couldn't write newline!");
        for y in 0..SIZE {
            for x in 0..SIZE {
                write!(f, "{} ", self.grid[y][x]).expect("Couldn't write a cell");
            }
            writeln!(f).expect("Couldn't write newline!");
        }

        write!(f, "")
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Hello, you are running the puzzle module!");
    let pstring: &str =
        "030072001000030090518000003050203100000705306000640205200060014007000630000008900";

    let my_puzzle = Puzzle::from_string(pstring);

    dbg!(my_puzzle.candidates);
}
