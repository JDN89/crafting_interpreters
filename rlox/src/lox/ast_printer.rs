use crate::{expr::*, lox_error::LoxError};

pub struct AstPrinter {}

impl AstPrinter {
    // pas the whole compolete expression to the print function
    pub fn print(&self, expression: &Expr) -> Result<String, LoxError> {
        expression.accept(self)
    }

    pub fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = String::new();
        builder.push_str("(");
        builder.push_str(&name);
        for ex in exprs {
            builder.push_str(" ");
            builder.push_str(&ex.accept(self).unwrap());
        }
        builder.push_str(")");
        builder
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&self, expr: &BinaryExpr) -> Result<String, LoxError> {
        // we first get the operator lexeme
        // then we pass left and right that can be a literal/ unary/ binary or grouping
        Ok(self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right]))
    }

    fn visit_grouping(&self, expr: &GroupingExpr) -> Result<String, crate::LoxError> {
        // we pass expr in our example grouping had a literal value that we passed to parenthesize
        Ok(self.parenthesize("group", &[&expr.expression]))
    }

    fn visit_literal(&self, expr: &LiteralExpr) -> Result<String, crate::LoxError> {
        Ok(expr.value.to_string())
    }

    fn visit_unary(&self, expr: &UnaryExpr) -> Result<String, crate::LoxError> {
        Ok(self.parenthesize(&expr.operator.lexeme, &[&expr.right]))
    }
}

// test ast printer result :
// (* (- 123) (group 45.67))
// pub fn test_ast_printer() {
//     let expression = Expr::Binary(BinaryExpr {
//         left: Box::new(Expr::Unary(UnaryExpr {
//             operator: Token {
//                 token_type: TokenType::Minus,
//                 lexeme: "-".to_string(),
//                 literal: crate::token::Literal::Integer(123.0),
//                 line: 1,
//             },
//             right: Box::new(Expr::Literal(LiteralExpr {
//                 value: token::Literal::Integer(123.0),
//             })),
//         })),
//         operator: Token {
//             token_type: TokenType::Star,
//             lexeme: "*".to_string(),
//             literal: token::Literal::Integer(0.0),
//             line: 1,
//         },
//         right: Box::new(Expr::Grouping(GroupingExpr {
//             expression: Box::new(Expr::Literal(LiteralExpr {
//                 value: token::Literal::Integer(45.67),
//             })),
//         })),
//     });
//
//     println!(
//         "{}",
//         crate::ast_printer::AstPrinter {}
//             .print(&expression)
//             .unwrap()
//     );
// }
