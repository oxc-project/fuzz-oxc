#![no_main]

use oxc::allocator::Allocator;
use oxc_regular_expression::{LiteralParser, Options};

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if s.chars().all(|s| !s.is_control()) {
            let allocator = Allocator::default();
            let _ = LiteralParser::new(&allocator, s, Some("v"), Options::default()).parse();
        }
    }
});
