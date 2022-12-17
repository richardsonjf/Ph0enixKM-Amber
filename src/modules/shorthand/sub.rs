use heraclitus_compiler::prelude::*;
use crate::modules::expression::{expr::Expr, binop::expression_arms_of_type};
use crate::modules::variable::{variable_name_extensions, handle_variable_reference};
use crate::utils::{ParserMetadata, TranslateMetadata};
use crate::translate::{module::TranslateModule, compute::{ArithOp, translate_computation}};
use crate::modules::types::{Type, Typed};

#[derive(Debug, Clone)]
pub struct ShorthandSub {
    var: String,
    expr: Box<Expr>,
    kind: Type,
    global_id: Option<usize>,
    is_ref: bool
}

impl SyntaxModule<ParserMetadata> for ShorthandSub {
    syntax_name!("Shorthand Sub");

    fn new() -> Self {
        Self {
            var: String::new(),
            expr: Box::new(Expr::new()),
            kind: Type::Null,
            global_id: None,
            is_ref: false
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        let var_tok = meta.get_current_token();
        self.var = variable(meta, variable_name_extensions())?;
        let tok = meta.get_current_token();
        token(meta, "-=")?;
        let variable = handle_variable_reference(meta, var_tok, &self.var)?;
        self.kind = variable.kind;
        self.global_id = variable.global_id;
        self.is_ref = variable.is_ref;
        self.expr.parse(meta)?;
        let message = "Substract operation can only substract numbers";
        expression_arms_of_type(meta, &self.kind, &self.expr.get_type(), &[Type::Num, Type::Text], tok, message)?;
        Ok(())
    }
}

impl TranslateModule for ShorthandSub {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        let expr = self.expr.translate(meta);
        let name = match self.global_id {
            Some(id) => format!("__{id}_{}", self.var),
            None => if self.is_ref { format!("eval ${{{}}}", self.var) } else { self.var.clone() }
        };
        let var = format!("${{{name}}}");
        format!("{}={}", name, translate_computation(meta, ArithOp::Sub, Some(var), Some(expr)))
    }
}