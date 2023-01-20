use crate::ast::types::Range;
use crate::ast::whitespace::LinesWithTrailingNewline;
use crate::checkers::ast::Checker;
use crate::docstrings::definition::Docstring;
use crate::fix::Fix;
use crate::registry::{Diagnostic, Rule};
use crate::rules::pydocstyle::helpers;
use crate::violations;

/// D200
pub fn one_liner(checker: &mut Checker, docstring: &Docstring) {
    let mut line_count = 0;
    let mut non_empty_line_count = 0;
    for line in LinesWithTrailingNewline::from(docstring.body) {
        line_count += 1;
        if !line.trim().is_empty() {
            non_empty_line_count += 1;
        }
        if non_empty_line_count > 1 {
            return;
        }
    }

    if non_empty_line_count == 1 && line_count > 1 {
        let mut diagnostic = Diagnostic::new(
            violations::FitsOnOneLine,
            Range::from_located(docstring.expr),
        );
        if checker.patch(&Rule::FitsOnOneLine) {
            if let (Some(leading), Some(trailing)) = (
                helpers::leading_quote(docstring.contents),
                helpers::trailing_quote(docstring.contents),
            ) {
                diagnostic.amend(Fix::replacement(
                    format!("{leading}{}{trailing}", docstring.body.trim()),
                    docstring.expr.location,
                    docstring.expr.end_location.unwrap(),
                ));
            }
        }
        checker.diagnostics.push(diagnostic);
    }
}