pub fn foo () {
    println!("_|_|_");
    println!("_|_|_");
    println!(" | | ");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
