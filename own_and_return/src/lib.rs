//own_and_return lev:2
pub struct Film {
    pub name: String,
}
pub fn read_film_name(f : &Film) -> String {
    f.name.clone()
    
}
pub fn take_film_name(f : Film) -> String {
    f.name
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume() {
        assert_eq!(
            take_film_name(Film {
                name: "Matrix".to_string()
            }),
            "Matrix".to_string()
        );
    }
    #[test]
    fn test_only_print() {
        assert_eq!(
            read_film_name(&Film {
                name: "Matrix".to_string()
            }),
            "Matrix".to_string()
        )
    }
}