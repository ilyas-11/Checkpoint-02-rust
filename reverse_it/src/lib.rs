//reverse_it lev:1
pub fn reverse_it(v: i32) -> String {
  let s = v.abs().to_string();
  let rev:String= s.chars().rev().collect();
  if v<0{
    return format!("-{}{}",rev,s);
  }
  format!("{}{}",rev,s)
}
#[test]
fn reverse_it_test() {
    assert_eq!("321123", &reverse_it(123));
    assert_eq!("987654321123456789", &reverse_it(123456789));
    assert_eq!("00", &reverse_it(0));
    assert_eq!("-321123", &reverse_it(-123));
    assert_eq!("11", &reverse_it(1));
}