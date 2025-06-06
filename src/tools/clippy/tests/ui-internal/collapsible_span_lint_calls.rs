#![deny(clippy::collapsible_span_lint_calls)]
#![allow(clippy::missing_clippy_version_attribute)]
#![feature(rustc_private)]

extern crate clippy_utils;
extern crate rustc_ast;
extern crate rustc_errors;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

use clippy_utils::diagnostics::{span_lint_and_help, span_lint_and_note, span_lint_and_sugg, span_lint_and_then};
use rustc_ast::ast::Expr;
use rustc_errors::Applicability;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_tool_lint! {
    pub clippy::TEST_LINT,
    Warn,
    "",
    report_in_external_macro: true
}

declare_lint_pass!(Pass => [TEST_LINT]);

impl EarlyLintPass for Pass {
    fn check_expr(&mut self, cx: &EarlyContext, expr: &Expr) {
        let lint_msg = "lint message";
        let help_msg = "help message";
        let note_msg = "note message";
        let sugg = "new_call()";
        let predicate = true;

        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.span_suggestion(expr.span, help_msg, sugg.to_string(), Applicability::MachineApplicable);
        });
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.span_help(expr.span, help_msg);
        });
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.help(help_msg);
        });
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.span_note(expr.span, note_msg);
        });
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.note(note_msg);
        });

        // This expr shouldn't trigger this lint.
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            db.note(note_msg);
            if predicate {
                db.note(note_msg);
            }
        });

        // Issue #8798
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            db.help(help_msg).help(help_msg);
        });
    }
}

fn main() {}
