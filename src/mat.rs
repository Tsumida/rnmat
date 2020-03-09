use super::rnum::RNum;
use std::fmt::Display;

#[derive(Debug)]
pub enum RNMatError{
    RowDismatch,
    ColDismatch,
    InvalidIndex,
}

impl Display for RNMatError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RNMatError{}

type Result<T> = std::result::Result<T, RNMatError>;

#[derive(Debug)]
pub struct RNMat {
    mat: Vec<Vec<RNum>>,
}

impl RNMat {
    /// Create a empty RNMat.
    pub fn new() -> RNMat {
        RNMat { mat: Vec::new() }
    }

    pub fn row_num(&self) -> usize {
        self.mat.len()
    }

    pub fn col_num(&self) -> usize {
        if self.mat.len() == 0 {
            0
        } else {
            self.mat[0].len()
        }
    }

    pub fn push_row(&mut self, row: Vec<RNum>) -> Result<()>{
        if self.mat.len() > 0 &&
            self.mat[0].len() != row.len(){
            return Err(RNMatError::RowDismatch)
        }
        self.mat.push(row);
        Ok(())
    }

    pub fn push_col(&mut self, col: Vec<RNum>) -> Result<()>{
        let row_cnt = self.mat.len();
        if row_cnt == 0 {
            self.mat.extend(
                col.into_iter()
                    .map(|ele| vec![ele])
                    .collect::<Vec<Vec<RNum>>>(),
            );
        } else {
            if row_cnt != col.len(){
                return Err(RNMatError::ColDismatch);
            }
            for (i, ele) in col.into_iter().enumerate() {
                self.mat[i].push(ele);
            }
        }
        Ok(())

    }

    /// Swap row a and row b.
    pub fn swap_row(&mut self, rindex_a: usize, rindex_b: usize) -> Result<()>{
        let leng = self.mat.len();
        if leng <= rindex_a || leng <= rindex_b{
            return Err(RNMatError::InvalidIndex);
        }
        self.mat.swap(rindex_a, rindex_b);
        Ok(())
    }

    pub fn row_mul_scalar(&mut self, factor: RNum, index: usize) -> Result<()>{
        if self.mat.len() <= index{
            return Err(RNMatError::InvalidIndex);
        }
        self.mat[index]
            .iter_mut()
            .for_each(|ele| *ele = *ele * factor);
            Ok(())
    }

    /// Check before matrix multiplication.
    fn is_valid_dimension(&self, other: &RNMat) -> bool {
        (self.mat.len() + other.mat.len() == 0) || (self.col_num() == other.mat.len())
    }
}

impl From<Vec<Vec<(i32, i32)>>> for RNMat {
    fn from(vecs: Vec<Vec<(i32, i32)>>) -> RNMat {
        if vecs.len() == 0 {
            return RNMat { mat: Vec::new() };
        }
        let col_len = vecs[0].len();
        RNMat {
            mat: vecs
                .into_iter()
                .map(|row| {
                    assert_eq!(col_len, row.len());
                    row.into_iter()
                        .map(|tup| RNum::new(tup.0, tup.1))
                        .collect::<Vec<RNum>>()
                })
                .collect(),
        }
    }
}

impl PartialEq for RNMat {
    fn eq(&self, other: &Self) -> bool {
        // empty.
        let row_cnt = self.mat.len();
        if row_cnt != other.mat.len() {
            return false;
        }
        if row_cnt == 0 {
            // empty
            return true;
        }
        let col_cnt = self.mat[0].len();
        if col_cnt != other.mat[0].len() {
            return false;
        }
        for i in 0..row_cnt {
            for j in 0..col_cnt {
                if self.mat[i][j] != other.mat[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test_rnmat {
    use super::*;

    #[test]
    #[should_panic]
    fn test_from_panic() {
        let _ = RNMat::from(vec![vec![(1, 2), (3, 4)], vec![(5, 6)]]);
    }

    #[test]
    fn test_eq() {
        // row !=
        assert_ne!(RNMat::from(vec![vec![(1, 2), (3, 4)],]), RNMat::new(),);
        // row ==, col !=
        assert_ne!(
            RNMat::from(vec![vec![(1, 2), (3, 4)],]),
            RNMat::from(vec![vec![(1, 2)],]),
        );
        // row ==, col ==, ele !=
        assert_ne!(
            RNMat::from(vec![vec![(1, -2), (-3, 4)], vec![(5, 6), (7, -8)],]),
            RNMat::from(vec![vec![(1, -2), (-3, 4)], vec![(-5, 6), (7, -8)],]),
        );

        // empty
        assert_eq!(RNMat::from(vec![]), RNMat::from(vec![]),);

        // all ele ==
        assert_eq!(
            RNMat::from(vec![vec![(1, -2), (-3, 4)], vec![(-5, 6), (7, -8)],]),
            RNMat::from(vec![vec![(1, -2), (-3, 4)], vec![(-5, 6), (7, -8)],]),
        );
    }

    #[test]
    fn test_push_row() {
        let mut rnm = RNMat::new();

        assert_eq!(0, rnm.row_num());
        assert_eq!(0, rnm.col_num());

        rnm.push_row(vec![RNum::new(1, 2), RNum::new(3, 4)]).unwrap();
        assert_eq!(1, rnm.row_num());
        assert_eq!(2, rnm.col_num());
    }

    #[test]
    #[should_panic]
    fn test_panic_push_row() {
        let mut rnm = RNMat::from(vec![vec![(1, 2)]]);
        rnm.push_row(vec![RNum::new(1, 2), RNum::new(3, 4)]).unwrap();
    }

    #[test]
    fn test_push_col() {
        let mut rnm = RNMat::new();
        assert_eq!(0, rnm.row_num());
        assert_eq!(0, rnm.col_num());

        rnm.push_col(vec![RNum::new(1, 2), RNum::new(3, 4)]).unwrap();
        assert_eq!(rnm, RNMat::from(vec![vec![(1, 2)], vec![(3, 4)],]));
    }

    #[test]
    #[should_panic]
    fn test_panic_push_col() {
        let mut rnm = RNMat::from(vec![vec![(1, 2)]]);
        rnm.push_col(vec![RNum::new(1, 2), RNum::new(3, 4)]).unwrap();
    }

    #[test]
    fn test_is_valid_dimension() {
        assert_eq!(true, RNMat::new().is_valid_dimension(&RNMat::new()));
        assert_eq!(
            false,
            RNMat::new().is_valid_dimension(&RNMat::from(vec![vec![(1, 2)]]))
        );
        // 1x2 x 1x2
        assert_eq!(
            false,
            RNMat::from(vec![vec![(1, 2), (3, 4)]])
                .is_valid_dimension(&RNMat::from(vec![vec![(1, 2), (3, 4)]]))
        );
        // 2x1 x 2x2
        assert_eq!(
            false,
            RNMat::from(vec![vec![(1, 2)], vec![(3, 4)]]).is_valid_dimension(&RNMat::from(vec![
                vec![(1, 2), (3, 4)],
                vec![(5, 6), (7, 8)]
            ]))
        );
    }

    #[test]
    fn test_swap() {
        let mut mat1 = RNMat::from(vec![vec![(1, 2), (3, 4)], vec![(5, 6), (7, 8)]]);
        mat1.swap_row(0, 1).unwrap();
        assert_eq!(
            mat1,
            RNMat::from(vec![vec![(5, 6), (7, 8)], vec![(1, 2), (3, 4)]])
        )
    }
    #[test]
    #[should_panic]
    fn test_panic_swap() {
        let mut mat1 = RNMat::from(vec![vec![(1, 2), (3, 4)], vec![(5, 6), (7, 8)]]);
        mat1.swap_row(2, 1).unwrap();
        assert_eq!(
            mat1,
            RNMat::from(vec![vec![(5, 6), (7, 8)], vec![(1, 2), (3, 4)]])
        )
    }

    #[test]
    fn test_row_mul_scalar() {
        let mut mat = RNMat::from(vec![vec![(1, 2), (3, 4)]]);
        mat.row_mul_scalar(RNum::new(1, 2), 0usize).unwrap();
        assert_eq!(mat, RNMat::from(vec![vec![(1, 4), (3, 8)]]));

        mat.row_mul_scalar(RNum::new(0, 1), 0).unwrap();
        assert_eq!(mat, RNMat::from(vec![vec![(0, 1), (0, 2)]]));
    }

    #[test]
    #[should_panic]
    fn test_panic_row_mul_scalar() {
        let mut mat = RNMat::new();
        mat.row_mul_scalar(RNum::new(1, 2), 0).unwrap();
    }
}
