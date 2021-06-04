pub mod entry;
pub mod string;
pub mod util;

mod serde_utils;

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
