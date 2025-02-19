use rustpython_parser::ast::Stmt;

use ruff_macros::{derive_message_formats, violation};
use ruff_python_stdlib::str;

use crate::ast::helpers::identifier_range;
use crate::registry::Diagnostic;
use crate::rules::pep8_naming::helpers;
use crate::source_code::Locator;
use crate::violation::Violation;

/// ## What it does
/// Checks for `CamelCase` imports that are aliased to lowercase names.
///
/// ## Why is this bad?
/// [PEP 8] recommends naming conventions for classes, functions,
/// constants, and more. The use of inconsistent naming styles between
/// import and alias names may lead readers to expect an import to be of
/// another type (e.g., confuse a Python class with a constant).
///
/// Import aliases should thus follow the same naming style as the member
/// being imported.
///
/// ## Example
/// ```python
/// from example import MyClassName as myclassname
/// ```
///
/// Use instead:
/// ```python
/// from example import MyClassName
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/
#[violation]
pub struct CamelcaseImportedAsLowercase {
    pub name: String,
    pub asname: String,
}

impl Violation for CamelcaseImportedAsLowercase {
    #[derive_message_formats]
    fn message(&self) -> String {
        let CamelcaseImportedAsLowercase { name, asname } = self;
        format!("Camelcase `{name}` imported as lowercase `{asname}`")
    }
}

/// N813
pub fn camelcase_imported_as_lowercase(
    import_from: &Stmt,
    name: &str,
    asname: &str,
    locator: &Locator,
) -> Option<Diagnostic> {
    if helpers::is_camelcase(name) && str::is_lower(asname) {
        return Some(Diagnostic::new(
            CamelcaseImportedAsLowercase {
                name: name.to_string(),
                asname: asname.to_string(),
            },
            identifier_range(import_from, locator),
        ));
    }
    None
}
