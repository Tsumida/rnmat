#[inline]
pub fn gcd(mut a:u32, mut b:u32) -> u32{
    if a < b{
        let tmp = b;
        b = a;
        a = tmp;
    } 
    if b == 1{
        1
    }else if a == b || b == 0{
        a
    }else{
        // gcd(a, b) = gcd(b, a % b)
        let mut r = a % b;
        while r > 0{
            a = b;
            b = r;
            r = a % b;
        }
        b
    }
}

pub fn get_reduced_pair(a:u32, b:u32) -> (u32, u32){
    let gcd_num = gcd(a, b);
    (a / gcd_num, b / gcd_num)
}


#[cfg(test)]
mod utils_test{
    use super::*;

    #[test]
    fn test_gcd() {
        // corner case
        assert_eq!(1, gcd(1, 1));
        assert_eq!(1, gcd(2, 1));
        assert_eq!(2, gcd(2, 0));
        // normal
        assert_eq!(2, gcd(10, 2));
        assert_eq!(1, gcd(17, 23));
        // a < b
        assert_eq!(gcd(9, 3), gcd(3, 9));
        /*
        let res = gcd(16, 4);
        assert_eq!(res, 4);
        assert_eq!(res, gcd(-16, 4));
        assert_eq!(res, gcd(-16, -4));
        assert_eq!(res, gcd(16, -4));
        */
    }
}

