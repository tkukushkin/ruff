use std::fmt;

use rustpython_parser::ast;
use rustpython_parser::ast::Cmpop;

use ruff_python_semantic::analyze::function_type;
use ruff_python_semantic::{ScopeKind, SemanticModel};

use crate::settings::Settings;

pub(super) fn in_dunder_init(semantic: &SemanticModel, settings: &Settings) -> bool {
    let scope = semantic.scope();
    let (
        ScopeKind::Function(ast::StmtFunctionDef {
            name,
            decorator_list,
        ..
        }) |
        ScopeKind::AsyncFunction(ast::StmtAsyncFunctionDef {
            name,
            decorator_list,
            ..
        })
    ) = scope.kind else {
        return false;
    };
    if name != "__init__" {
        return false;
    }
    let Some(parent) = scope.parent.map(|scope_id| &semantic.scopes[scope_id]) else {
        return false;
    };

    if !matches!(
        function_type::classify(
            name,
            decorator_list,
            parent,
            semantic,
            &settings.pep8_naming.classmethod_decorators,
            &settings.pep8_naming.staticmethod_decorators,
        ),
        function_type::FunctionType::Method
    ) {
        return false;
    }
    true
}

/// A wrapper around [`Cmpop`] that implements `Display`.
#[derive(Debug)]
pub(super) struct CmpopExt(Cmpop);

impl From<&Cmpop> for CmpopExt {
    fn from(cmpop: &Cmpop) -> Self {
        CmpopExt(*cmpop)
    }
}

impl fmt::Display for CmpopExt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let representation = match self.0 {
            Cmpop::Eq => "==",
            Cmpop::NotEq => "!=",
            Cmpop::Lt => "<",
            Cmpop::LtE => "<=",
            Cmpop::Gt => ">",
            Cmpop::GtE => ">=",
            Cmpop::Is => "is",
            Cmpop::IsNot => "is not",
            Cmpop::In => "in",
            Cmpop::NotIn => "not in",
        };
        write!(f, "{representation}")
    }
}
