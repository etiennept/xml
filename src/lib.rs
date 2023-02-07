#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub xmlparser);


pub fn add(left: usize, right: usize) -> usize {
    left + right
}
struct TagName {


}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 3);

    }
}
