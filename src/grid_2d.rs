pub struct Grid2D {
    columns: usize,
    rows: usize,
    grid: Vec<f64>,
}

impl Grid2D {
    pub fn new(columns: usize, rows: usize) -> Grid2D {
        if columns < 1 || rows < 1 {
            panic!("Grid size of rows and columns must be greater than zero.");
        }

        let grid = vec![0.0; columns * rows];

        Grid2D {
            columns,
            rows,
            grid,
        }
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn get_value_by_index(&self, index: usize) -> f64 {
        self.grid[index]
    }

    pub fn get_value(&self, column: usize, row: usize) -> f64 {
        let index = self.get_index(column, row);

        self.grid[index]
    }

    pub fn set_value_by_index(&mut self, index: usize, value: f64) {
        //let got = std::mem::replace(&mut self.grid[index], value);
        self.grid[index] = value;
    }

    pub fn set_value(&mut self, column: usize, row: usize, value: f64) {
        let index = self.get_index(column, row);
        self.grid[index] = value;
    }

    pub fn get_smallest_number(&self) -> f64 {
        //*self.grid.iter().min().unwrap()
        self.grid.iter().cloned().fold(0.0 / 0.0, f64::min)
    }

    pub fn get_largest_number(&self) -> f64 {
        //*self.grid.iter().max().unwrap()
        self.grid.iter().cloned().fold(0.0 / 0.0, f64::max)
    }

    fn get_index(&self, column: usize, row: usize) -> usize {
        row * self.columns + column
    }

    // fn get_column_row(&self, index: usize) -> (usize, usize) {
    //     let column = index % self.width as usize;
    //     let row = index / self.width as usize;
    //
    //     (column, row)
    // }
}

#[cfg(test)]
mod tests {
    use crate::grid_2d::Grid2D;

    #[test]
    fn create_grid() {
        let grid = Grid2D::new(4, 3);

        assert_eq!(grid.columns(), 4);
        assert_eq!(grid.rows(), 3);
    }

    #[test]
    fn fetch_from_grid() {
        let grid = Grid2D::new(1, 1);

        assert_eq!(grid.get_value(0, 0), 0.0);
    }

    #[test]
    fn update_value_in_grid() {
        let mut grid = Grid2D::new(2, 2);
        grid.set_value_by_index(0, 1.0);
        grid.set_value_by_index(1, 2.0);
        grid.set_value_by_index(2, 3.0);
        grid.set_value(1, 1, 4.0);

        assert_eq!(grid.get_value_by_index(0), 1.0);
        assert_eq!(grid.get_value(0, 0), 1.0);
        assert_eq!(grid.get_value_by_index(1), 2.0);
        assert_eq!(grid.get_value(1, 0), 2.0);
        assert_eq!(grid.get_value_by_index(2), 3.0);
        assert_eq!(grid.get_value(0, 1), 3.0);
        assert_eq!(grid.get_value_by_index(3), 4.0);
        assert_eq!(grid.get_value(1, 1), 4.0);
    }
}
