# ! [no_std]

mod admin;
mod allowance;
mod balance;
mod contract;
mod metadata;
mod storage_types;

pubuse crate::contract::TokenCIient;



/* 
pub fn add(left: u64, right: u64) -> u64 {
    left + right
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
 */