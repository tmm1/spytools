use regex::Regex;

use crate::process::ProcessType;

/// Dummy type for providing a Node implementation of the trait
pub struct NodeProcessType {}

impl ProcessType for NodeProcessType {
    fn library_regex() -> Regex {
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        return Regex::new(r"/libnode\.so").unwrap();

        #[cfg(target_os = "macos")]
        return Regex::new(r"/libnode\.(dylib|so)$").unwrap();

        #[cfg(windows)]
        return Regex::new(r"\\node\.dll$").unwrap();
    }

    #[cfg(target_os = "macos")]
    fn is_framework(_path: &std::path::Path) -> bool {
        false
    }
}