#[path = "rnum.rs"]
mod rnum;
use rnum::RNum;

#[derive(Debug)]
pub struct RNMat{
    mat: Vec<Vec<RNum>>,
}

impl RNMat{
    pub fn new() -> RNMat{
        RNMat{
            mat: Vec::new(),
        }
    }

    pub fn get_row_cnt(&self) -> usize{
        self.mat.len()
    }

    pub fn get_col_cnt(&self) -> usize{
        if self.mat.len() == 0{
            0
        }else{
            self.mat[0].len()
        }
    }

    pub fn push_row(&mut self, row: Vec<RNum>){
        if self.mat.len() > 0{
            assert!(self.mat[0].len() == row.len());
        }
        self.mat.push(row);
    }

    pub fn push_col(&mut self, col: Vec<RNum>){
        if self.mat.len() > 0{
            assert!(self.mat.len() == col.len());
        }
        for (i, ele) in col.into_iter().enumerate(){
            self.mat[i].push(ele);
        }
    }

    
}

impl From<Vec<Vec<(i32, i32)>>> for RNMat{
    fn from(vecs: Vec<Vec<(i32, i32)>>) -> RNMat{
        RNMat{
            mat: vecs.into_iter()
                    .map(|row| row.into_iter()
                                    .map(|tup| RNum::new(tup.0, tup.1))
                                    .collect::<Vec<RNum>>())
                    .collect()
        }
    }
}

impl PartialEq for RNMat{
    fn eq(&self, other:&Self) -> bool{
        // empty.
        let row_cnt = self.mat.len();
        if row_cnt != other.mat.len(){
            return false;
        }
        if row_cnt == 0{ // empty
            return true;
        }
        let col_cnt = self.mat[0].len(); 
        if col_cnt != other.mat[0].len(){
            return false;
        }
        for i in 0..row_cnt{
            for j in 0..col_cnt{
                if self.mat[i][j] != other.mat[i][j]{
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test_rnmat{
    use super::*;

    #[test]
    fn test_eq() {
        // row != 
        assert_ne!(
            RNMat::from(
                vec![vec![(1, 2), (3,4)],]
            ),
            RNMat::new(),
        );
        // row ==, col !=
        assert_ne!(
            RNMat::from(
                vec![vec![(1, 2), (3,4)],]
            ),
            RNMat::from(
                vec![vec![(1, 2)],]
            ),
        );
        // row ==, col ==, ele !=
        assert_ne!(
            RNMat::from(vec![
                vec![(1, -2), (-3, 4)],
                vec![(5, 6), (7, -8)],
            ]),
            RNMat::from(vec![
                vec![(1, -2), (-3, 4)],
                vec![(-5, 6), (7, -8)],
            ]),
        );

        // empty
        assert_eq!(
            RNMat::from(vec![]),
            RNMat::from(vec![]),
        );

        // all ele ==
        assert_eq!(
            RNMat::from(vec![
                vec![(1, -2), (-3, 4)],
                vec![(-5, 6), (7, -8)],
            ]),
            RNMat::from(vec![
                vec![(1, -2), (-3, 4)],
                vec![(-5, 6), (7, -8)],
            ]),
        );
    }

    #[test]
    fn test_push_row() {
        let mut rnm = RNMat::new();

        assert_eq!(0, rnm.get_row_cnt());
        assert_eq!(0, rnm.get_col_cnt());

        rnm.push_row(vec![RNum::new(1, 2), RNum::new(3, 4)]);
        assert_eq!(1, rnm.get_row_cnt());
        assert_eq!(2, rnm.get_col_cnt());
    }

    #[test]
    #[should_panic]
    fn test_push_row_panic() {
        let mut rnm = RNMat::from(vec![vec![(1, 2)]]);
        rnm.push_row(vec![RNum::new(1, 2), RNum::new(3, 4)]);
    }


    #[test]
    fn test_push_col() {
        let mut rnm = RNMat::new();

        assert_eq!(0, rnm.get_row_cnt());
        assert_eq!(0, rnm.get_col_cnt());

        rnm.push_col(vec![RNum::new(1, 2), RNum::new(3, 4)]);
        assert_eq!(2, rnm.get_row_cnt());
        assert_eq!(1, rnm.get_col_cnt());
    }

    #[test]
    #[should_panic]
    fn test_push_col_panic() {
        let mut rnm = RNMat::from(vec![vec![(1, 2)]]);
        rnm.push_col(vec![RNum::new(1, 2), RNum::new(3, 4)]);
    }

}