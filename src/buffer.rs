use std::{
    marker::PhantomPinned,
    ops::{Deref, DerefMut, Index, IndexMut},
    pin::Pin,
    slice::SliceIndex,
};

use bc_utils::other::{roll_slice1, transpose, transpose_set};
use bc_utils_lg::structs::source::src::Src;

#[derive(Clone, Debug, Default)]
#[pin_project::pin_project]
pub struct Buffer {
    // row data
    #[pin]
    pub d: Vec<Vec<f64>>,
    pub current_src: *const Vec<f64>,
    pub window: usize,
    pub is_transposed: bool,
    #[pin]
    pub _pin: PhantomPinned,
}

impl Buffer {
    pub fn from_row(src: Vec<Vec<f64>>, window: usize) -> Pin<Box<Self>> {
        let mut df = Box::pin(Self::default());
        let mut project = df.as_mut().project();
        *project.d.as_mut().get_mut() = src[src.len() - window..].to_vec();
        *project.window = window;
        *project.is_transposed = false;
        *project.current_src = project.d.last().unwrap() as *const _;
        df
    }

    pub fn from_column(src: Vec<Vec<f64>>, window: usize) -> Pin<Box<Self>> {
        Self::from_row(transpose(src), window)
    }
}

impl Buffer {
    pub fn update(self: &mut Pin<Box<Self>>, src: Vec<f64>) {
        let mut pt = self.as_mut().project();
        roll_slice1(pt.d.as_mut_slice(), -1);
        let mut project = self.as_mut().project();
        project.d[*project.window - 1] = src;
        *project.current_src = project.d.last().unwrap() as *const _;
    }

    pub fn update_from_src(self: &mut Pin<Box<Self>>, src: Src) {
        self.update(src.to_vec());
    }

    pub fn update_extend(self: &mut Pin<Box<Self>>, src: &[Vec<f64>]) {
        let mut pt = self.as_mut().project();
        roll_slice1(pt.d.as_mut_slice(), -(src.len() as i32));
        for _ in 0..src.len() {
            pt.d.pop();
        }
        pt.d.extend_from_slice(src);
        *pt.current_src = pt.d.last().unwrap() as *const _;
    }
}

impl Buffer {
    pub fn iter(&self) -> impl Iterator<Item = &Vec<f64>> {
        self.d.iter()
    }
    pub fn first(&self) -> Option<&Vec<f64>> {
        self.d.first()
    }
    pub fn last(&self) -> Option<&Vec<f64>> {
        self.d.last()
    }
    pub fn last1(&self) -> Option<&Vec<f64>> {
        self.d.get(self.len() - 2)
    }
    pub fn len(&self) -> usize {
        self.d.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() != 0
    }
    pub fn as_slice(&self) -> &[Vec<f64>] {
        self.d.as_slice()
    }
    pub fn transpose_set(self: &mut Pin<Box<Self>>) {
        let mut pt = self.as_mut().project();
        transpose_set(pt.d.as_mut().get_mut());
        *pt.window = pt.d.len();
        *pt.is_transposed = !*pt.is_transposed;
    }
    pub fn transpose(mut self: Pin<Box<Self>>) -> Pin<Box<Self>> {
        self.transpose_set();
        self
    }
}

impl<T: SliceIndex<[Vec<f64>]>> Index<T> for Buffer {
    type Output = T::Output;
    fn index(&self, index: T) -> &Self::Output {
        &self.d[index]
    }
}

impl IndexMut<usize> for Buffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.d[index]
    }
}

impl Extend<Vec<f64>> for Buffer {
    fn extend<T: IntoIterator<Item = Vec<f64>>>(&mut self, iter: T) {
        self.d.extend(iter);
    }
}

impl Deref for Buffer {
    type Target = [Vec<f64>];
    fn deref(&self) -> &Self::Target {
        self.d.as_slice()
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.d.as_mut_slice()
    }
}

pub trait ToBuff {
    fn to_buff(&self) -> Pin<Box<Buffer>>;
}

impl ToBuff for [Vec<f64>] {
    fn to_buff(&self) -> Pin<Box<Buffer>> {
        let mut df = Box::pin(Buffer::default());
        let mut pt = df.as_mut().project();
        *pt.d = self.to_vec();
        *pt.current_src = pt.d.last().unwrap() as *const _;
        *pt.window = pt.d.len();
        df
    }
}

impl PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.d == other.d
            && self.window == other.window
            && self.is_transposed == other.is_transposed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bc_test_kit::prelude::*;

    static BF: LazyLock<fn() -> Pin<Box<Buffer>>> = LazyLock::new(|| || SRC.to_buff());

    #[test]
    fn from_row_res_1() {
        let bf = Buffer::from_row(SRC.clone(), 50);
        assert_eq!(bf.len(), 50);
    }

    #[test]
    fn from_column_res_1() {
        let bf = Buffer::from_column(SRC_TRANSPOSE.clone(), 50);
        assert_eq!(bf.len(), 50);
    }

    #[test]
    fn transpose_res_1() {
        let mut bf = Buffer::from_column(SRC_TRANSPOSE.clone(), 50);
        bf.transpose_set();
        assert!(bf.is_transposed);
        bf.transpose_set();
        assert!(!bf.is_transposed);
    }

    #[test]
    fn update_res_1() {
        let mut bf = BF();
        let mut res = BF();
        bf.update(SRC_EL.to_vec());
        roll_slice1(&mut res.as_mut().project().d, -1);
        let l = res.d.len() - 1;
        res.as_mut().project().d[l] = SRC_EL.to_vec();
        assert_eq_pr!(bf, res);
    }

    #[test]
    fn update_extend_res_1() {
        let mut bf = BF();
        let mut res = BF();
        let mut res_pt = res.as_mut().project();
        bf.update_extend(&SRC);
        roll_slice1(res_pt.d.as_mut_slice(), -(SRC.len() as i32));
        for _ in 0..SRC.len() {
            res_pt.d.pop();
        }
        res_pt.d.extend_from_slice(&SRC);
        assert_eq_pr!(bf, res);
    }
}
