#[macro_use(array)]
extern crate ndarray;
use ndarray::Array2;

fn spiral(matrix: &Array2<i32>) {
    let dim = matrix.raw_dim();
    let (mut row_ub, mut col_ub, mut row_lb, mut col_lb) = (dim[0], dim[1], -1, -1);
    let total = dim[0] * dim[1];
    let (mut i, mut j) = (0i32, 0i32);
    let mut count = 1;
    let direction: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut d = 0;
    while count <= total {
        print!("{} ", matrix[(i as usize, j as usize)]);
        if d == 0 && i + 1 >= col_ub as i32 {
            row_lb += 1;
            d = (d + 1) % 4;
        }
        if d == 1 && j + 1 >= row_ub as i32 {
            col_ub -= 1;
            d = (d + 1) % 4;
        }

        if d == 2 && i - 1 <= col_lb as i32 {
            row_ub -= 1;
            d = (d + 1) % 4;
        }

        if d == 3 && j - 1 <= row_lb as i32 {
            col_lb += 1;
            d = (d + 1) % 4;
        }
        i += direction[d].0;
        j += direction[d].1;
        count += 1;
    }
}

fn main() {
    let matrix = array![
        [1, 16, 15, 14, 13],
        [2, 17, 24, 23, 12],
        [3, 18, 25, 22, 11],
        [4, 19, 20, 21, 10],
        [5, 6, 7, 8, 9],
    ];
    spiral(&matrix);
}
