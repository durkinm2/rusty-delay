pub const PLUGIN_NAME: &str = "Tape Delay";
pub const PLUGIN_VENDOR: &str = "durk-dev";
pub const PLUGIN_URL: &str = "";
pub const PLUGIN_EMAIL: &str = "";
pub const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const PLUGIN_ID: &str = "com.durkdev.tape-delay";
pub const PLUGIN_DESCRIPTION: &str = "A simple tape-style delay effect";

// VST3 specific
pub const VST3_CLASS_ID: [u8; 16] = *b"TapeDelay\0\0\0\0\0\0\0";