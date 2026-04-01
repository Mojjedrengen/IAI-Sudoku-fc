use crate::sudoku_solver_trait::AbstractSudokuSolver;
use array2d::{Array2D, Error};
use std::intrinsics::breakpoint;
use std::vec;

pub struct SudokuSolver {
    puzzle: Array2D<usize>,
    size: usize,
    d: Vec<Vec<usize>>,
}

impl AbstractSudokuSolver for SudokuSolver {
    fn get_puzzle(&self) -> &Array2D<usize> {
        &self.puzzle
    }
    fn set_value(&mut self, col: usize, row: usize, value: usize) {
        self.puzzle
            .set(row, col, value)
            .expect("Error in setting value");
    }
    fn setup(&mut self, size: usize) {
        self.size = size;
        self.puzzle = Array2D::filled_with(0, size * size, size * size);
        self.d = Vec::new();
        for _ in 0..(size * size * size * size) {
            self.d.push(Vec::new());
        }
    }
    fn read_in_puzzle(&mut self, p: Array2D<usize>) {
        self.puzzle = p;
    }

    fn solve(&self) -> bool {
        // INITIAL_FC
        // FC

        true
    }
}

impl SudokuSolver {
    pub fn fc(&mut self, asn: &[usize]) -> Option<Vec<usize>> {
        let mut asn = asn.to_vec().to_owned();
        if !asn.contains(&0) {
            return Some(asn);
        }

        let x: usize = asn.iter().find(|i| **i == 0).unwrap().to_owned();
        //let d_old: Vec<Vec<usize>> = self.d.clone();
        let mut d_old: Vec<Vec<usize>> = Vec::new();

        {
            for lst in &self.d {
                let i = lst.clone();
                d_old.push(i);
            }
        }
        let mut ret = None?;
        d_old[x].iter().for_each(|v| {
            if self.ac_fc(&x, v) {
                asn.insert(x, *v);
                //let r = self.fc(asn);
                match self.fc(&asn) {
                    Some(r) => ret = Some(r),
                    None => {
                        asn.insert(x, 0);
                        self.d = d_old.clone();
                    }
                }
            } else {
                self.d = d_old.clone();
            }
        });

        ret
    }

    fn ac_fc(&mut self, x: &usize, v: &usize) -> bool {
        self.d[*x].clear();
        self.d[*x].push(*v);

        let mut q = Vec::new();
        let row = x / (self.size * self.size);
        let col = x - ((x / (self.size * self.size)) * self.size * self.size);
        let cell_x = row / self.size;
        let cell_y = col / self.size;

        for i in 0..self.size * self.size {
            if self.get_variable(&i, &col) > *x {
                q.push(self.get_variable(&i, &col));
            }
        }

        for j in 0..self.size * self.size {
            if self.get_variable(&j, &row) > *x {
                q.push(self.get_variable(&row, &j));
            }
        }

        for i in cell_x * self.size..cell_x * self.size + 2 + 1 {
            for j in cell_y * self.size..cell_y * self.size + 2 + 1 {
                if self.get_variable(&i, &j) > *x {
                    q.push(self.get_variable(&i, &j));
                }
            }
        }

        let mut consistant = true;
        while !q.is_empty() && consistant {
            let y = q.pop().unwrap();
            if self.revise(&y, x) {
                consistant = !self.d.get(y).expect("Error in getting AC-FC").is_empty();
            }
        }
        consistant
    }

    // Would be good if it could not use clone.
    // Probably uses a lot of javaisime
    fn revise(&mut self, xi: &usize, xj: &usize) -> bool {
        let mut deleted = false;

        for vi in self.d.get_mut(*xi).unwrap().clone() {
            let mut xi_eq_val: Vec<usize> = (0..self.size * self.size * self.size * self.size)
                .map(|_| 0)
                .collect();
            xi_eq_val.insert(*xi, vi);
            let mut has_support = false;
            for vj in self.d.get(*xj).unwrap().clone() {
                if self.consistant(&xi_eq_val, xj, &vj) {
                    has_support = true;
                    break;
                }
            }

            if !has_support {
                self.d.get_mut(*xj).unwrap().remove(vi);
                deleted = true;
            }
        }
        deleted
    }

    fn consistant(&self, asn: &[usize], variable: &usize, val: &usize) -> bool {
        true
    }

    fn get_variable(&self, i: &usize, j: &usize) -> usize {
        i * self.size * self.size * j
    }
}
