use crate::error::*;
use crate::lexer::token::*;

macro_rules! define_ast {
    ($type:ident/$visitor:ident, $( $choice:ident/$struct:ident/$visit:ident {$ ($prop_name:ident: $prop_type:ty),* $(,)?})*) => {
        pub enum $type {
            $(
                $choice($struct),
            )*
        }

        impl $type {
            pub fn accept<T>(&self, visitor: &dyn $visitor<T>) -> Result<T, LoxError> {
                match self {
                    $(
                        $type::$choice(expr) => expr.accept(visitor),
                    )*
                }
            }
        }

        $(
            pub struct $struct {
                $(
                    pub $prop_name: $prop_type,
                )*
            }
        )*

        pub trait $visitor<T> {
            $(
                fn $visit(&self, expr: &$struct) -> Result<T, LoxError>;
            )*
        }

        $(
            impl $struct {
                pub fn accept<T>(&self, visitor: &dyn $visitor<T>) -> Result<T, LoxError> {
                    visitor.$visit(self)
                }
            }
        )*
    };
}

define_ast!(
    Expr/ExprVisitor,
    Binary/BinaryExpr/visit_binary_expr {left: Box<Expr>, operator: Token, right: Box<Expr>}
    Grouping/GroupingExpr/visit_grouping_expr {expression: Box<Expr>}
    Literal/LiteralExpr/visit_literal_expr {value: Option<Object>}
    Unary/UnaryExpr/visit_unary_expr {operator: Token, right: Box<Expr>}
);
