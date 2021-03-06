//! The `FluentValue` enum represents values which can be formatted to a String.
//!
//! The [`ResolveValue`][] trait from the [`resolve`][] module evaluates AST nodes into
//! `FluentValues` which can then be formatted to Strings using the i18n formatters stored by the
//! `MessageContext` instance if required.
//!
//! The arguments `HashMap` passed to [`MessageContext::format`][] should also use `FluentValues`
//! as values of arguments.
//!
//! [`ResolveValue`]: ../resolve/trait.ResolveValue.html
//! [`resolve`]: ../resolve
//! [`MessageContext::format`]: ../context/struct.MessageContext.html#method.format

use std::f32;
use std::boxed::FnBox;

use super::context::MessageContext;

/// Value types which can be formatted to a String.
#[derive(Clone, Debug, PartialEq)]
pub enum FluentValue {
    /// Fluent String type.
    String(String),
    /// Fluent Number type.
    Number(f32),
}

// XXX Replace this with a proper plural rule
fn get_plural_rule(ctx: &MessageContext) -> Box<FnBox(f32) -> &'static str> {
    match ctx.locales[0] {
        "x-testing" => Box::new(|num| if num == 1.0 { "one" } else { "other" }),
        _ => Box::new(|_| "other"),
    }
}

impl FluentValue {
    pub fn format(&self, _ctx: &MessageContext) -> String {
        match *self {
            FluentValue::String(ref s) => s.clone(),
            FluentValue::Number(ref n) => format!("{}", n),
        }
    }

    pub fn matches(&self, ctx: &MessageContext, other: &FluentValue) -> bool {
        match (self, other) {
            (&FluentValue::String(ref a), &FluentValue::String(ref b)) => a == b,
            (&FluentValue::Number(ref a), &FluentValue::Number(ref b)) => {
                (a - b).abs() < f32::EPSILON
            }
            (&FluentValue::String(ref a), &FluentValue::Number(ref b)) => {
                let plural_rule = get_plural_rule(ctx);
                plural_rule(*b) == a
            }
            (&FluentValue::Number(..), &FluentValue::String(..)) => false,
        }
    }
}

impl From<String> for FluentValue {
    fn from(s: String) -> Self {
        FluentValue::String(s)
    }
}

impl<'a> From<&'a str> for FluentValue {
    fn from(s: &'a str) -> Self {
        FluentValue::String(String::from(s))
    }
}

impl From<f32> for FluentValue {
    fn from(n: f32) -> Self {
        FluentValue::Number(n)
    }
}

impl From<i8> for FluentValue {
    fn from(n: i8) -> Self {
        FluentValue::Number(f32::from(n))
    }
}
