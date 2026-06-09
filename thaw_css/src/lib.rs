//! Build-time CSS bundle for Thaw UI components.
//!
//! This crate provides a pre-bundled CSS file containing all Thaw component styles.
//! It's designed to be used as a build dependency, allowing you to include Thaw's
//! CSS without pulling in the full runtime dependencies.
//!
//! # Usage
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [build-dependencies]
//! thaw_css = "0.1"
//! ```
//!
//! Then in your `build.rs`:
//!
//! ```ignore
//! fn main() {
//!     // Write CSS to your public assets directory
//!     std::fs::write("public/thaw.css", thaw_css::THAW_CSS).unwrap();
//! }
//! ```
//!
//! Or inline it directly in your HTML template:
//!
//! ```ignore
//! use thaw_css::THAW_CSS;
//!
//! let html = format!(r#"
//!     <!DOCTYPE html>
//!     <html>
//!         <head>
//!             <style>{}</style>
//!         </head>
//!         <body>...</body>
//!     </html>
//! "#, THAW_CSS);
//! ```

include!(concat!(env!("OUT_DIR"), "/generated_css.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_is_not_empty() {
        assert!(!THAW_CSS.is_empty());
        assert!(THAW_CSS_LEN > 0);
    }

    #[test]
    fn css_len_matches() {
        assert_eq!(THAW_CSS.len(), THAW_CSS_LEN);
    }
}
