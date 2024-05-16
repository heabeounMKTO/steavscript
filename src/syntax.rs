use crate::token::Token;
use std::fmt;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub enum LiteralValue {
    Boolean(bool),
    Null,
    Number(f64),
    String(String),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Null => write!(f, "null"),
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "{}", s),
        }
    }
}

pub trait Visitor<R> {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_grouping_expr(&self, expression: &Expr) -> R;
    fn visit_literal_expr(&self, value: String) -> R;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value.to_string()),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> String {
        self.parenthesize("group".to_string(), vec![expression])
    }

    fn visit_literal_expr(&self, value: String) -> String {
        value
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.clone(), vec![right])
    }
}

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: String, exprs: Vec<&Expr>) -> String {
        let mut r = String::new();
        r.push_str("(");
        r.push_str(&name);
        for e in &exprs {
            r.push_str(" ");
            r.push_str(&e.accept(self));
        }
        r.push_str(")");
        r
    }
}
