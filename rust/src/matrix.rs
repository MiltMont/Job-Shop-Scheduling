use std::fmt::Debug;

pub struct Matrix<T: Default + Debug + Clone> {
    mat: Vec<Vec<T>>,
}

impl<T: Default + Debug + Clone> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix").field("", &self.mat).finish()
    }
}

impl<T: Default + Debug + Clone> Default for Matrix<T> {
    fn default() -> Self {
        Self {
            mat: Default::default(),
        }
    }
}

impl<T: Debug + Default + Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Matrix<T> {
        let mat: Vec<Vec<T>> = vec![vec![T::default(); cols]; rows];
        Matrix { mat }
    }

    pub fn at(&self, i: usize, j: usize) -> Option<&T> {
        self.mat.get(i)?.get(j)
    }

    pub fn set_at(&mut self, item: T, i: usize, j: usize) {
        self.mat[i][j] = item;
    }
}
