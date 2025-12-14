use zed_extension_api::{self as zed};

struct Syscript {}

impl zed::Extension for Syscript {
    fn new() -> Self {
        Self {}
    }
}

zed::register_extension!(Syscript);
