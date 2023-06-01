use crate::AsFormat;
use crate::{FormatNodeRule, PyFormatter};
use ruff_formatter::formatter::Formatter;
use ruff_formatter::prelude::{empty_line, hard_line_break};
use ruff_formatter::{write, Buffer, Format, FormatContext, FormatResult};
use ruff_python_ast::prelude::Stmt;
use rustpython_parser::ast::ModModule;

#[derive(Default)]
pub struct FormatModModule;

impl FormatNodeRule<ModModule> for FormatModModule {
    fn fmt_fields(&self, item: &ModModule, f: &mut PyFormatter) -> FormatResult<()> {
        for stmt in &item.body {
            match stmt {
                Stmt::FunctionDef(function_def) => write!(f, [TwoNewlinesAfter(function_def)])?,
                Stmt::AsyncFunctionDef(async_function_def) => {
                    write!(f, [TwoNewlinesAfter(async_function_def)])?
                }
                Stmt::ClassDef(class_def) => write!(f, [TwoNewlinesAfter(class_def)])?,
                Stmt::Assign(assign) => write!(f, [assign.format(), hard_line_break()])?,
                Stmt::AugAssign(assign) => write!(f, [assign.format(), hard_line_break()])?,
                Stmt::AnnAssign(assign) => write!(f, [assign.format(), hard_line_break()])?,
                _ => write!(f, [stmt.format(), empty_line()])?,
            }
        }
        Ok(())
    }
}

pub(crate) struct TwoNewlinesAfter<T>(T);

impl<C: FormatContext, T: AsFormat<C>> Format<C> for TwoNewlinesAfter<T> {
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        write!(f, [self.0.format(), empty_line(), empty_line()])
    }
}
