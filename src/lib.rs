pub use deco::entries::deco_entries;
pub use deco::matrix::deco_matrix;
pub use deco::vector::deco_vector;

mod deco;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
