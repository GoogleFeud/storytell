pub mod input;

pub fn test_fn() -> i32 {
    32
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_test() {
        assert_eq!(test_fn(), 32);
    }
}