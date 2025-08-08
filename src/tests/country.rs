#[allow(unused_imports)]
use super::super::core::country::{Country, Ideology};

#[test]
fn test() {
    let c = Country::new("example".to_string(), 123);
    println!("{:#?}", c)
}
