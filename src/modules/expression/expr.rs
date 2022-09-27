use heraclitus_compiler::prelude::*;
use crate::modules::types::{Typed, Type};
use crate::translate::module::TranslateModule;
use crate::utils::{ParserMetadata, TranslateMetadata};
use super::literal::{
    bool::Bool,
    number::Number,
    text::Text
};
use super::binop::{
    add::Add,
    sub::Sub,
    mul::Mul,
    div::Div,
    modulo::Modulo,
    and::And,
    or::Or,
    gt::Gt,
    ge::Ge,
    lt::Lt,
    le::Le,
    eq::Eq,
    neq::Neq
};
use super::unop::{
    not::Not
};
use super::parenthesis::Parenthesis;
use crate::modules::variable::get::VariableGet;
use crate::modules::command::expr::CommandExpr;
use crate::modules::condition::ternary::Ternary;
use crate::modules::function::invocation::FunctionInvocation;
use crate::handle_types;

#[derive(Debug, Clone)]
pub enum ExprType {
    Bool(Bool),
    Number(Number),
    Text(Text),
    CommandExpr(CommandExpr),
    Parenthesis(Parenthesis),
    VariableGet(VariableGet),
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Modulo(Modulo),
    And(And),
    Or(Or),
    Gt(Gt),
    Ge(Ge),
    Lt(Lt),
    Le(Le),
    Eq(Eq),
    Neq(Neq),
    Not(Not),
    Ternary(Ternary),
    FunctionInvocation(FunctionInvocation)
}

#[derive(Debug, Clone)]
pub struct Expr {
    value: Option<ExprType>,
    kind: Type
}

impl Typed for Expr {
    fn get_type(&self) -> Type {
        self.kind.clone()
    }
}

impl Expr {
    handle_types!(ExprType, [
        // Ternary conditional
        Ternary,
        // Logical operators
        And, Or, Not,
        // Comparison operators
        Gt, Ge, Lt, Le, Eq, Neq,
        // Arithmetic operators
        Add, Sub, Mul, Div, Modulo,
        // Literals
        Parenthesis, CommandExpr, Bool, Number, Text,
        // Function invocation
        FunctionInvocation,
        // Variable access
        VariableGet
    ]);

    // Get result out of the provided module and save it in the internal state
    fn get<S>(&mut self, meta: &mut ParserMetadata, mut module: S, cb: impl Fn(S) -> ExprType) -> SyntaxResult
    where
        S: SyntaxModule<ParserMetadata> + Typed
    {
        // Match syntax
        match syntax(meta, &mut module) {
            Ok(()) => {
                self.kind = module.get_type();
                self.value = Some(cb(module));
                Ok(())
            }
            Err(details) => Err(details)
        }
    }
}

impl SyntaxModule<ParserMetadata> for Expr {
    syntax_name!("Expr");

    fn new() -> Self {
        Expr {
            value: None,
            kind: Type::Null
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        let statements = self.get_modules();
        for statement in statements {
            // Handle comments
            if let Some(token) = meta.get_current_token() {
                if token.word.starts_with('#') {
                    meta.increment_index();
                    continue
                }
            }
            match self.parse_match(meta, statement) {
                Ok(()) => return Ok(()),
                Err(failure) => {
                    if let Failure::Loud(err) = failure {
                        return Err(Failure::Loud(err))
                    }
                }
            }
        }
        // TODO: Handle this `can_fail` atribute
        error!(meta, meta.get_current_token(), "Expected expression")
    }
}

impl TranslateModule for Expr {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        self.translate_match(meta, self.value.as_ref().unwrap())
    }
}