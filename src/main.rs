fn main() {}

#[tracing::instrument]
fn foobar() {
	// ↑↑ Hover or trigger docs on `foobar`

	// See how it shows:
	// `let __tracing_attr_fake_return: ()`
	// instead of function signature
	// `fn foobar()`
}
