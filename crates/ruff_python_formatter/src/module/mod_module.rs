use crate::builders::PyFormatterExtensions;
use crate::context::NodeLevel;
use crate::{FormatNodeRule, PyFormatter};
use ruff_formatter::prelude::hard_line_break;
use ruff_formatter::{Format, FormatResult};
use rustpython_parser::ast::ModModule;

#[derive(Default)]
pub struct FormatModModule;

impl FormatNodeRule<ModModule> for FormatModModule {
    fn fmt_fields(&self, item: &ModModule, f: &mut PyFormatter) -> FormatResult<()> {
        f.join_nodes(NodeLevel::TopLevel)
            .nodes(&item.body)
            .finish()?;
        hard_line_break().fmt(f)?;
        Ok(())
    }
}
