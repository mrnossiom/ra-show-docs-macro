#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {}
fn foobar() {
	{}
	#[allow(clippy::suspicious_else_formatting)]
	{
		let __tracing_attr_span;
		let __tracing_attr_guard;
		if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
			&& tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
			|| { false }
		{
			__tracing_attr_span = {
				use ::tracing::__macro_support::Callsite as _;
				static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
					static META: ::tracing::Metadata<'static> = {
						::tracing_core::metadata::Metadata::new(
							"foobar",
							"ra_show_docs_macro",
							tracing::Level::INFO,
							::core::option::Option::Some("src/main.rs"),
							::core::option::Option::Some(3u32),
							::core::option::Option::Some("ra_show_docs_macro"),
							::tracing_core::field::FieldSet::new(
								&[],
								::tracing_core::callsite::Identifier(&__CALLSITE),
							),
							::tracing::metadata::Kind::SPAN,
						)
					};
					::tracing::callsite::DefaultCallsite::new(&META)
				};
				let mut interest = ::tracing::subscriber::Interest::never();
				if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
					&& tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
					&& {
						interest = __CALLSITE.interest();
						!interest.is_never()
					} && ::tracing::__macro_support::__is_enabled(__CALLSITE.metadata(), interest)
				{
					let meta = __CALLSITE.metadata();
					::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
				} else {
					let span = ::tracing::__macro_support::__disabled_span(__CALLSITE.metadata());
					{};
					span
				}
			};
			__tracing_attr_guard = __tracing_attr_span.enter();
		}
		#[warn(clippy::suspicious_else_formatting)]
		{
			#[allow(
				unknown_lints,
				unreachable_code,
				clippy::diverging_sub_expression,
				clippy::let_unit_value,
				clippy::unreachable,
				clippy::let_with_type_underscore,
				clippy::empty_loop
			)]
			if false {
				let __tracing_attr_fake_return: () = loop {};
				return __tracing_attr_fake_return;
			}
			{}
		}
	}
}
