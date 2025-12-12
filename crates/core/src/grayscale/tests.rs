
#[cfg(test)]
mod tests {
    use crate::grayscale::ColorPalette;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_ron_serialization_deserialization() {
        let palette = ColorPalette::fake(5);
        let file = NamedTempFile::new().unwrap();
        let path = file.path();

        // Save as RON
        palette.save_to_file(path).unwrap();

        // Load back
        let loaded_palette = ColorPalette::load_from_file(path).unwrap();
        assert_eq!(palette, loaded_palette);
    }

    #[test]
    fn test_toml_backward_compatibility() {
        let palette = ColorPalette::fake(3);
        let file = NamedTempFile::new().unwrap();
        let path = file.path();

        // Manually save as TOML
        let toml_string = toml::to_string(&palette).unwrap();
        fs::write(path, toml_string).unwrap();

        // Load back using the new load_from_file (should handle TOML)
        let loaded_palette = ColorPalette::load_from_file(path).unwrap();
        assert_eq!(palette, loaded_palette);
    }
}
