/// Detects whether a target has support for N-bit atomic instructions.
///
/// This macro is invoked with the width of an atomic instruction (`8`, `16`,
/// `32`, `64`, or `ptr`) and then any number of items that are conditional upon
/// the existence of the selected atomic width. It expands to a `cfg` guard that
/// removes the items from the compile graph when the atomics they depend on do
/// not exist.
///
/// This list is by necessity incomplete; the compiler maintains its information
/// about target atomicity [here][targets], and has an unstable `cfg` guard to
/// accomplish this task, tracked [here][target_has_atomic]. The compiler team
/// is also working on a separate `cfg` item, [`available(SYMBOL_PATH)`][avail],
/// which would allow crates to guard on the existence of a specific symbol.
///
/// To use this macro, wrap your conditional items in it:
///
/// ```rust
/// use radium::has_atomic;
///
/// has_atomic!(8
///   fn this_function_uses_byte_atomics() {}
///   fn so_does_this() {}
/// );
/// ```
///
/// If this macro is incorrect for your target, please submit a PR adding your
/// target to its expansion list.
///
/// [avail]: https://github.com/rust-lang/rust/issues/64797
/// [targets]: https://github.com/rust-lang/rust/tree/be28b6235e64e0f662b96b710bf3af9de169215c/compiler/rustc_target/src/spec
/// [target_has_atomic]: https://github.com/rust-lang/rust/issues/32976
#[macro_export]
macro_rules! has_atomic {
	(8 $($i:item)*) => { $(
		#[cfg(not(any(
			target_arch = "riscv32imc",
		)))]
		$i
	)* };
	(16 $($i:item)*) => { $(
		#[cfg(not(any(
			target_arch = "riscv32imc",
		)))]
		$i
	)* };
	(32 $($i:item)*) => { $(
		#[cfg(not(any(
			target_arch = "riscv32imc",
		)))]
		$i
	)* };
	(64 $($i:item)*) => { $(
		#[cfg(not(any(
			target_arch = "mips",
			target_arch = "mipsel",
			target_arch = "powerpc",
			target_arch = "riscv32imc",
		)))]
		$i
	)* };
	(ptr $($i:item)*) => { $(
		#[cfg(not(any(
			target_arch = "riscv32imc",
		)))]
		$i
	)* };
}
