//lucas_number lev:2
pub fn lucas_number(n: u32) -> u32 {
    let mut n0:u32=2;
    let mut n1:u32=1;
    let mut res:u32= 0;
    if n ==0{
        return n0;
    }else if n == 1{
        return n1;
    }
    let mut step = 2;
    while step<=n{
        step+=1;
        res=n1+n0;
        n0=n1;
        n1=res;
    }
    res
}
 #[test]
    fn lucas_number_test() {
        assert_eq!(lucas_number(2), 3);
        assert_eq!(lucas_number(5), 11);
        assert_eq!(lucas_number(10), 123);
        assert_eq!(lucas_number(13), 521);
        assert_eq!(lucas_number(25), 167761);
    }