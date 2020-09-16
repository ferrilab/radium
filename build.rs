//! Target detection
//!
//! Prefer putting target information in the `src/has_atomic.rs:has_atomic!`
//! macro instead. Some targets, such as `riscv32imc`, are not uniquely
//! selectable through the available compiler `cfg` discovery APIs, and so must
//! be supported here. Where your target *is* precisely selectable through the
//! existing `cfg` infrastructure, it belongs in the macro.
//!
//! This build script translates the target for which `radium` is being compiled
//! into a set of directives that the crate can use to control which atomic
//! symbols it attempts to name.
//!
//! The compiler maintains its store of target information here:
//! <https://github.com/rust-lang/rust/tree/be28b6235e64e0f662b96b710bf3af9de169215c/compiler/rustc_target/src/spec>
//!
//! That module is not easily extracted into something that can be loaded here,
//! so we are replicating it through string matching on the target name until
//! the `cfg(target_has_atomic)` flag stabilizes.
//!
//! Use `rustc --print target-list` to enumerate the full list of targets
//! available.

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
    has_ptr: bool,
}

impl Atomics {
    const ALL: Self = Self {
        has_8: true,
        has_16: true,
        has_32: true,
        has_64: true,
        has_ptr: true,
    };
    const NONE: Self = Self {
        has_8: false,
        has_16: false,
        has_32: false,
        has_64: false,
        has_ptr: false,
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut atomics = Atomics::ALL;

    let target = std::env::var("TARGET")?;
    // Add new target strings here with their atomic availability.
    #[allow(clippy::match_single_binding, clippy::single_match)]
    match &*target {
        _ => {}
    }

    let arch = target.split('-').next().ok_or("Invalid target triple")?;
    // Add new architecture sections here with their atomic availability.
    #[allow(clippy::match_single_binding, clippy::single_match)]
    match arch {
        // "riscv32imc-unknown-none-elf" and "riscv32imac-unknown-none-elf" are
        // both `target_arch = "riscv32", and have no `cfg`-discoverable
        // distinction. As such, the non-atomic RISC-V target must be discovered
        // here, rather than in the macro.
        "riscv32i" | "riscv32imc" => atomics = Atomics::NONE,
        "riscv32imac" => atomics.has_64 = false,
        _ => {}
    }

    // Target detection prints out flags indicating that the target does **NOT**
    // have an atomic instruction for the specified width. This flag is picked
    // up by the `has_atomic!` macro, which looks for markers that an atomic is
    // absent. The macro presumes that presence is the default state.
    if !atomics.has_8 {
        println!("cargo:rustc-cfg=radium_missing_8");
    }
    if !atomics.has_16 {
        println!("cargo:rustc-cfg=radium_missing_16");
    }
    if !atomics.has_32 {
        println!("cargo:rustc-cfg=radium_missing_32");
    }
    if !atomics.has_64 {
        println!("cargo:rustc-cfg=radium_missing_64");
    }
    if !atomics.has_ptr {
        println!("cargo:rustc-cfg=radium_missing_ptr");
    }

    Ok(())
}
