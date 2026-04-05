use crate::sudoku_solver_trait::AbstractSudokuSolver;
use array2d::{Array2D, Error};
use std::cmp::Ordering;
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

    // Created as a copy of the java version
    fn consistant(&self, asn: &[usize], variable: &usize, val: &usize) -> bool {
        //let v1;
        //let v2;
        let mut asn = asn.to_vec().to_owned();
        asn.insert(*variable, *val);

        // allDiff(col[i])
        for i in 0..self.size * self.size {
            for j in 0..self.size * self.size {
                for k in 0..self.size * self.size {
                    if k != j {
                        let v1 = asn.get(self.get_variable(&i, &j)).unwrap();
                        let v2 = asn.get(self.get_variable(&i, &k)).unwrap();
                        if *v1 != 0 && *v2 != 0 && v1.cmp(v2) == Ordering::Equal {
                            asn.insert(*variable, 0);
                            return false;
                        }
                    }
                }
            }
        }

        // alldiff(row[j])
        for j in 0..self.size * self.size {
            for i in 0..self.size * self.size {
                for k in 0..self.size * self.size {
                    if k != i {
                        let v1 = asn.get(self.get_variable(&i, &j)).unwrap();
                        let v2 = asn.get(self.get_variable(&k, &j)).unwrap();
                        if *v1 != 0 && *v2 != 0 && v1.cmp(v2) == Ordering::Equal {
                            asn.insert(*variable, 0);
                            return false;
                        }
                    }
                }
            }
        }

        // alldiff(block[size*i, size*j])
        for i in 0..self.size {
            for j in 0..self.size {
                for i1 in 0..self.size {
                    for j1 in 0..self.size {
                        let var1 = self.get_variable(&(self.size * i + i1), &(self.size * j + j1));
                        for i2 in 0..self.size {
                            for j2 in 0..self.size {
                                let var2 =
                                    self.get_variable(&(self.size * i + i2), &(self.size * j + j2));
                                if var1 != var2 {
                                    let v1 = asn.get(var1).unwrap();
                                    let v2 = asn.get(var2).unwrap();
                                    if *v1 != 0 && *v2 != 0 && v1.cmp(v2) == Ordering::Equal {
                                        asn.insert(*variable, 0);
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        asn.insert(*variable, 0);
        true
    }

    fn initial_fc(&mut self, an_assignment: &[usize]) -> bool {
        for i in 0..an_assignment.len() {
            let v = an_assignment.get(i).unwrap();
            if *v != 0 {
                let mut q = self.get_relevant_variables(&i);
                let mut consistant = true;
                while !q.is_empty() && consistant {
                    let y = q.pop().unwrap();
                    if self.revise(&y, &i) {
                        consistant = !self.d.get(y).unwrap().is_empty();
                    }
                }
                if !consistant {
                    return false;
                }
            }
        }
        true
    }

    fn get_relevant_variables(&self, x: &usize) -> Vec<usize> {
        let mut q = Vec::new();
        let col = self.get_column(x);
        let row = self.get_row(x);
        let cell_x = row / self.size;
        let cell_y = col / self.size;

        for i in 0..self.size * self.size {
            if self.get_variable(&i, &col) != *x {
                q.push(self.get_variable(&i, &col));
            }
        }
        for j in 0..self.size * self.size {
            if self.get_variable(&row, &j) != *x {
                q.push(self.get_variable(&row, &j));
            }
        }
        for i in cell_x * self.size..cell_x * self.size + 3 {
            for j in cell_y * self.size..cell_y * self.size + 3 {
                if self.get_variable(&i, &j) != *x {
                    q.push(self.get_variable(&i, &j));
                }
            }
        }

        q
    }

    pub fn get_assignment(&mut self, p: &Array2D<usize>) -> Vec<usize> {
        let mut asn = Vec::new();
        for i in 0..self.size * self.size {
            for j in 0..self.size * self.size {
                asn.insert(self.get_variable(&i, &j), *p.get(i, j).unwrap());
                if p.get(i, j).unwrap() != &0 {
                    self.d.get_mut(i).unwrap().clear();
                    self.d.get_mut(i).unwrap().push(*p.get(i, j).unwrap());
                }
            }
        }
        asn
    }

    pub fn get_puzzle(&self, asn: &[usize]) -> Array2D<usize> {
        let mut p = Array2D::filled_with(0, self.size * self.size, self.size * self.size);
        for i in 0..self.size * self.size {
            for j in 0..self.size * self.size {
                let val = asn.get(self.get_variable(&i, &j)).unwrap();
                let _ = p.set(i, j, *val);
            }
        }
        p
    }

    fn get_variable(&self, i: &usize, j: &usize) -> usize {
        i * self.size * self.size * j
    }

    fn get_row(&self, x: &usize) -> usize {
        x / (self.size * self.size)
    }
    fn get_column(&self, x: &usize) -> usize {
        x - ((x / (self.size * self.size)) * self.size * self.size)
    }
}
