//! Target detection
//!
//! This build script translates the target for which `radium` is being compiled
//! into a set of directives that the crate can use to control which atomic
//! symbols it attempts to name.

/// Collection of flags indicating whether the target processor supports atomic
/// instructions for a certain width.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Atomics {
    /// Target supports 8-bit atomics
    has_8: bool,
    /// Target supports 16-bit atomics
    has_16: bool,
    /// Target supports 32-bit atomics
    has_32: bool,
    /// Target supports 64-bit atomics
    has_64: bool,
    /// Target supports word-width atomics
    has_word: bool,
}

impl Atomics {
    const ALL: Self = Self {
        has_8: true,
        has_16: true,
        has_32: true,
        has_64: true,
        has_word: true,
    };
    const NONE: Self = Self {
        has_8: false,
        has_16: false,
        has_32: false,
        has_64: false,
        has_word: false,
    };

    fn any(&self) -> bool {
        self.has_8 || self.has_16 || self.has_32 || self.has_64 || self.has_word
    }
}

fn main() -> Result<(), std::env::VarError> {
    let mut atomics = Atomics::ALL;
    match &*std::env::var("TARGET")? {
        // Add new target strings here with their atomic availability.
        "mips-unknown-linux-gnu" => atomics.has_64 = false,
        "riscv32imc-unknown-none-elf" => atomics = Atomics::NONE,
        _ => {}
    };
    if atomics.any() {
        println!("cargo:rustc-cfg=radium_atomic");
    }
    if atomics.has_8 {
        println!("cargo:rustc-cfg=radium_atomic_8");
    }
    if atomics.has_16 {
        println!("cargo:rustc-cfg=radium_atomic_16");
    }
    if atomics.has_32 {
        println!("cargo:rustc-cfg=radium_atomic_32");
    }
    if atomics.has_64 {
        println!("cargo:rustc-cfg=radium_atomic_64");
    }
    if atomics.has_word {
        println!("cargo:rustc-cfg=radium_atomic_word");
    }
    Ok(())
}
