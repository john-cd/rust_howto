
// Configuration struct
// When deserializing, any missing fields are filled in from the struct's
// implementation of Default.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    // Check internal URLs (and external URLs if check_external_urls = true)
    // (default = true)
    pub check_urls: bool,
    // Check whether external URLs (e.g. to http://... or https://...) are valid; default: false.
    // Has no effect if check_urls = false
    // Beware of the performance penalty when true.
    pub check_external_urls: bool,
    // If true (default), detect e.g. [ref]: http://example.com without a corresponding [some_text][ref]
    pub detect_unused_reference_definitions: bool,
    // Remove unused ref defs from the markdown, if true (default)
    pub delete_unused_reference_definitions: bool,
}

impl Default for PreprocConfig {
    fn default() -> Self {
        Self {
            check_urls: true,
            check_external_urls: false,
            detect_unused_reference_definitions: true,
            delete_unused_reference_definitions: true,
        }
    }
}
