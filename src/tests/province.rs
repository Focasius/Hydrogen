#[allow(unused_imports)]
use super::super::core::province::Province;
#[test]
fn test() {
    let p = Province::new("example".to_string(), 123);
    println!("{:#?}", p);
}
