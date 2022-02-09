pub use atext::AText;
pub use changeset::Changeset;

mod apool;
mod changeset;
mod digit;
mod head;
mod body;
mod atext;
mod write_to;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
