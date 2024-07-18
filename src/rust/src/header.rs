use extendr_api::prelude::*;
use std::io::Write;

struct ContainerBuilder<'a> {
    data: OptMatrix<&'a str>,
    rendered: OptMatrix<bool>,
}

#[derive(Debug)]
struct OptMatrix<T> {
    data: Vec<Option<T>>,
    nrow: usize,
    ncol: usize,
}

impl<T: Clone + Copy> OptMatrix<T> {
    // Constructor for the OptMatrix
    pub fn new(nrow: usize, ncol: usize) -> Self {
        let mut data = Vec::new();
        data.resize(nrow * ncol, None);
        OptMatrix { data, nrow, ncol }
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        if row > self.nrow || col > self.ncol {
            panic!("Row or column index out of bounds");
        }
        let index = row * self.ncol + col;
        self.data[index] = Some(val);
    }

    fn fill(&mut self, val: T) {
        self.data.fill(Some(val));
    }

    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        if row > self.nrow || col > self.ncol {
            panic!("Row or column index out of bounds");
        }
        let index = row * self.ncol + col;
        self.data[index]
    }

    pub fn fill_downup(&mut self) {
        let mut last_val;
        let mut index;
        for col in 0..self.ncol {
            // Fill down the columns
            last_val = None;
            for row in 0..self.nrow {
                index = row * self.ncol + col;
                if self.data[index].is_some() {
                    last_val = self.data[index];
                } else if last_val.is_some() {
                    self.data[index] = last_val;
                }
            }

            // Fill up the columns
            last_val = None;
            for row in (0..self.nrow).rev() {
                index = row * self.ncol + col;
                if self.data[index].is_some() {
                    last_val = self.data[index];
                } else if last_val.is_some() {
                    self.data[index] = last_val;
                }
            }
        }
    }

    pub fn transpose(&mut self) {
        let mut new_data = vec![None; self.data.len()]; // Pre-allocate new data with clones of an arbitrary element
        for i in 0..self.nrow {
            for j in 0..self.ncol {
                let old_index = i * self.ncol + j;
                let new_index = j * self.nrow + i;
                new_data[new_index] = self.data[old_index];
            }
        }

        *self = OptMatrix {
            data: new_data,
            nrow: self.ncol, // Swap the number of rows and columns
            ncol: self.nrow,
        };
    }
}

impl<'a> ContainerBuilder<'a> {
    pub fn new(var_names: &'a Strings) -> Self {
        let cnames_split: Vec<Vec<&str>> = var_names
            .iter()
            .map(|name| name.split('.').collect())
            .collect();

        let matrix_rows = cnames_split
            .iter()
            .map(Vec::len)
            .max()
            .expect("Must have a max value");

        let matrix_cols = var_names.len();

        let mut var_names_data = OptMatrix::new(matrix_cols, matrix_rows);

        cnames_split.iter().enumerate().for_each(|(c, vals)| {
            vals.iter()
                .enumerate()
                .for_each(|(r, val)| var_names_data.set(c, r, *val))
        });

        var_names_data.transpose();
        var_names_data.fill_downup();

        let mut rendered_var_names = OptMatrix::new(matrix_rows, matrix_cols);
        rendered_var_names.fill(false);

        Self {
            data: var_names_data,
            rendered: rendered_var_names,
        }
    }
    fn get_colspan(&self, row: usize, col: usize) -> usize {
        if self.data.ncol == 1 {
            return 1;
        }

        let mut colspan = 0;

        let mut current_col = col;

        loop {
            if current_col >= self.data.ncol {
                break;
            }

            if row > 0 && self.get_colspan(row - 1, col) <= colspan {
                break;
            }

            if self.data.get(row, col) != self.data.get(row, current_col) {
                break;
            }

            current_col += 1;
            colspan += 1;
        }

        colspan
    }
    fn get_rowspan(&self, row: usize, col: usize) -> usize {
        if self.data.nrow == 1 {
            return 1;
        }

        let mut rowspan = 0;
        let mut current_row = row;

        loop {
            if current_row >= self.data.nrow {
                break;
            }

            if self.data.get(row, col) != self.data.get(current_row, col) {
                break;
            }

            current_row += 1;
            rowspan += 1;
        }

        rowspan
    }
    fn is_rendered(&self, row: usize, col: usize) -> bool {
        unsafe { self.rendered.get(row, col).unwrap_unchecked() }
    }
    fn render_th(&mut self, row: usize, col: usize) -> Option<(usize, usize)> {
        let rowspan = self.get_rowspan(row, col);
        let colspan = self.get_colspan(row, col);
        if self.is_rendered(row, col) {
            return None;
        }
        for row in row..rowspan {
            for col in col..col + colspan {
                self.rendered.set(row, col, true);
            }
        }
        Some((rowspan, colspan))
    }
    fn build(mut self, buffer: &mut Vec<u8>) {
        let _ = write!(buffer, "<thead>");

        for row in 0..self.data.nrow {
            let _ = write!(buffer, "<tr>");
            for col in 0..self.data.ncol {
                if let Some((rowspan, colspan)) = self.render_th(row, col) {
                    let _ = write!(
                        buffer,
                        r#"
                        <th colspan={colspan} rowspan={rowspan} class="border text-center align-middle">{}</th>
                        "#,
                        self.data.get(row, col).unwrap()
                    );
                }
            }
            let _ = write!(buffer, "</tr>");
        }

        let _ = write!(buffer, "</thead>");
    }
}

pub fn spyc_header_create(names: Strings, buffer: &mut Vec<u8>) {
    ContainerBuilder::new(&names).build(buffer);
}
