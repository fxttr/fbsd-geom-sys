#[cfg(test)]
mod tests {
    use std::env::var;
    use std::path::PathBuf;

    #[test]
    fn bindings() {
        assert!(PathBuf::from(var("OUT_DIR").unwrap())
            .join("bindings.rs")
            .exists())
    }
}
