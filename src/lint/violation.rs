use crate::lint::diagnostic::Diagnostic;

pub trait ComplianceViolation {
    fn to_diagnostic(&self) -> Diagnostic;
}
