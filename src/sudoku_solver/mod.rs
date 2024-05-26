use rand::*;
use std::collections::{HashMap, HashSet};
type Grid = [[i32; 9]; 9];
#[derive(PartialEq)]
pub enum GAME_STATE {
    InProgress,
    Completed,
    Failed,
}
pub struct SudokuSolver {
    pub grid: Grid,
    pub completed_cell_coord: HashSet<i8>,
    pub visited: HashMap<i8, HashSet<i32>>,
    pub current_grid_index: (i32, i32),
    pub is_backtrack: bool,
    pub current_state: GAME_STATE,
}

impl SudokuSolver {
    pub fn new(g: &Grid) -> Self {
        let mut solver = SudokuSolver {
            completed_cell_coord: HashSet::new(),
            visited: HashMap::new(),
            current_grid_index: (0, 0),
            grid: g.clone(),
            is_backtrack: false,
            current_state: GAME_STATE::InProgress,
        };
        for i in 0..9 {
            for j in 0..9 {
                let h = SudokuSolver::hash_coord(&i, &j);
                solver.visited.insert(h.clone(), HashSet::new());
                if solver.grid[i as usize][j as usize] != 0 {
                    solver.completed_cell_coord.insert(h);
                }
            }
        }
        solver
    }

    pub fn is_completed_coord(&self, i: i32, j: i32) -> bool {
        return self
            .completed_cell_coord
            .contains(&SudokuSolver::hash_coord(&i, &j));
    }

    pub fn iterate(&mut self) -> &mut Self {
        if self.current_state != GAME_STATE::InProgress {
            return self;
        }
        let c_i = self.current_grid_index.0;
        let c_j = self.current_grid_index.1;
        let coord_hash = SudokuSolver::hash_coord(&c_i, &c_j);
        if !self.is_backtrack {
            if c_i > 8 {
                self.current_state = GAME_STATE::Completed;
                return self;
            }
            if self.completed_cell_coord.contains(&coord_hash) {
                self.current_grid_index = SudokuSolver::get_next_index(c_i, c_j);
                return self;
            }
            let current_coord_set: &mut HashSet<i32> = self.visited.get_mut(&coord_hash).unwrap();
            match SudokuSolver::get_unvisited_coord(current_coord_set, c_i, c_j, &self.grid) {
                None => {
                    self.is_backtrack = true;
                }
                Some(coordinate) => {
                    current_coord_set.insert(coordinate);
                    self.grid[c_i as usize][c_j as usize] = coordinate;
                    self.current_grid_index = SudokuSolver::get_next_index(c_i, c_j);
                }
            }
            return self;
        }
        self.visited.insert(coord_hash, HashSet::new());
        self.grid[c_i as usize][c_j as usize] = 0;
        if c_i == 0 && c_j == 0 {
            self.current_state = GAME_STATE::Failed;
            return self;
        }
        let mut tmp_curr = SudokuSolver::get_previous_index(c_i, c_j);
        loop {
            let h = SudokuSolver::hash_coord(&tmp_curr.0, &tmp_curr.1);
            if !self.completed_cell_coord.contains(&h) {
                break;
            }
            tmp_curr = SudokuSolver::get_previous_index(tmp_curr.0, tmp_curr.1);
        }
        self.current_grid_index = tmp_curr;
        self.is_backtrack = false;
        return self;
    }

    fn hash_coord(i: &i32, j: &i32) -> i8 {
        return (i * 10 + j) as i8;
    }

    fn get_unvisited_coord(set: &HashSet<i32>, i: i32, j: i32, grid: &Grid) -> Option<i32> {
        //
        let mut current_visisted: HashSet<i32> = HashSet::new();
        while current_visisted.len() + set.len() < 9 {
            let mut rng = rand::thread_rng();
            let n: i32 = rng.gen_range(1..=9);
            // for n in 1..=9 {
            if !set.contains(&n) && !current_visisted.contains(&n) {
                let mut g = grid.clone();
                g[i as usize][j as usize] = n;
                if SudokuSolver::is_valid_cell(i, j, &g) {
                    return Some(n);
                }
                current_visisted.insert(n);
            }
            // }
        }
        return None;
    }

    fn get_previous_index(i: i32, j: i32) -> (i32, i32) {
        if j == 0 {
            (i - 1, 8)
        } else {
            (i, j - 1)
        }
    }

    fn get_next_index(i: i32, j: i32) -> (i32, i32) {
        if (j + 1) % 9 == 0 {
            (i + 1, 0)
        } else {
            (i, (j + 1) % 9)
        }
    }

    fn is_board_valid(grid: &Grid) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if grid[i][j] == 0 || !SudokuSolver::is_valid_cell(i as i32, j as i32, grid) {
                    return false;
                }
            }
        }
        return true;
    }

    fn is_valid_cell(i: i32, j: i32, grid: &Grid) -> bool {
        let mut x_sum: HashSet<i32> = HashSet::new();
        let mut y_sum: HashSet<i32> = HashSet::new();
        for x in 0..9 {
            let a = grid[x][j as usize];
            let b = grid[i as usize][x];
            if a != 0 {
                if x_sum.contains(&a) {
                    return false;
                }
                x_sum.insert(a);
            }
            if b != 0 {
                if y_sum.contains(&b) {
                    return false;
                }
                y_sum.insert(b);
            }
        }
        let ([xi, xj], [yi, yj]) =
            SudokuSolver::get_segment_coordinates(SudokuSolver::get_segment(i, j));
        let mut visited: HashSet<i32> = HashSet::new();
        for i in yi..=yj {
            for j in xi..=xj {
                let c = grid[i as usize][j as usize];
                if visited.contains(&c) && c > 0 {
                    return false;
                }
                visited.insert(c);
            }
        }
        return true;
    }

    fn get_segment_coordinates(index: i32) -> ([i32; 2], [i32; 2]) {
        match index {
            0 => ([0, 2], [0, 2]),
            1 => ([0, 2], [3, 5]),
            2 => ([0, 2], [6, 8]),
            3 => ([3, 5], [0, 2]),
            4 => ([3, 5], [3, 5]),
            5 => ([3, 5], [6, 8]),
            6 => ([6, 8], [0, 2]),
            7 => ([6, 8], [3, 5]),
            8 => ([6, 8], [6, 8]),
            _ => panic!("get_segment_coordinates - Invalid index"),
        }
    }

    fn get_segment(i: i32, j: i32) -> i32 {
        // Top
        if j < 3 {
            if i < 3 {
                return 0;
            } else if i < 6 {
                return 1;
            } else {
                return 2;
            }
        }
        // Mid
        else if j < 6 {
            if i < 3 {
                return 3;
            } else if i < 6 {
                return 4;
            } else {
                return 5;
            }
        }
        // Bottom
        else {
            if i < 3 {
                return 6;
            } else if i < 6 {
                return 7;
            } else {
                return 8;
            }
        }
    }
}
