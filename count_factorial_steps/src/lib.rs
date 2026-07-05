//count_factorial_steps lev:2
pub fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial <=1{
        return 0;
    }
    let mut res = 1;
    let mut step = 1;
    while res<factorial{
        step+=1;
        res*=step;
    }
    if res ==factorial{
        step
    }else {
        0
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_factorial_steps_edge_cases() {
        assert_eq!(0, count_factorial_steps(0));
        assert_eq!(0, count_factorial_steps(1));
        assert_eq!(0, count_factorial_steps(123));
    }
    #[test]
    fn count_factorial_steps_normal_cases() {
        assert_eq!(6, count_factorial_steps(720));
        assert_eq!(10, count_factorial_steps(3628800));
    }
}