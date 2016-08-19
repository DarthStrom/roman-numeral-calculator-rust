pub fn add(augend: &str, addend: &str) -> String {
    augend.to_string() + addend
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn i_plus_i_equals_ii() {
        assert_eq!(add("I", "I"), "II");
    }
}
