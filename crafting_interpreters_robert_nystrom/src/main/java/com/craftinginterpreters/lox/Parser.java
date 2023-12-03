package com.craftinginterpreters.lox;

import java.util.List;

import static com.craftinginterpreters.lox.TokenType.*;

// TODO test out the loop around part because I don't see the conneciton yet.
// Parsing a == b == c == d == e. For each iteration, we create a new binary expression using the previous one as the left operand.
// Then we call comparison() again to parse the right-hand operand. We combine the operator and its two operands into a new Expr.Binary syntax tree node, and then loop around.

public class Parser {
    private final List<Token> tokens;
    private int current = 0;

    public Parser(List<Token> tokens) {
        this.tokens = tokens;
    }
    private Expr expression() {
        return equality();

    };

    // equality -> comparison( ( "!=" | "==" ) comparison ) *;
    // if the parser never encounters an equality operator, then it never enters the loop.
    // In that case, the equality() method effectively calls and returns comparison(). In that way, this method matches an equality operator or anything of higher precedence.
    private Expr equality() {
        Expr expr = comparison();

        // in while loop we know we have found != or == and we must be parsing an equality expression
        while (match(BANG_EQUAL,EQUAL_EQUAL)) {
            // previous because we advance and consume one in match
            Token operator = previous();
            Expr right = comparison();
            expr = new Expr.Binary(expr,operator,right);
        }
        return expr;
    }

    private boolean match(TokenType ... types ) {
        for (TokenType type: types) {
            if (check (type)) {
                advance();
                return true;
            }
        }
        return false;
    }


    // if current token is equal to BANG_EQULA or EQUAL_EQUAL
    private boolean check(TokenType type) {
        if (isAtEnd()) return false;
        return peek().getType() == type;
    }

    private Token advance() {
        if (!isAtEnd()) current++ ;
        return previous();
    }
    private boolean isAtEnd() {
        return peek().getType()  == EOF;
    }
    private Token peek() {
        return tokens.get(current);
    }
    private Token previous() {
        return tokens.get(current - 1);
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
   private Expr comparison() {
        Expr expr = term();

        while (match(GREATER, GREATER_EQUAL, LESS, LESS_EQUAL)) {
            Token operator = previous();
            Expr right = term();
            expr = new Expr.Binary(expr,operator,right);
        }
        return expr;
   }


//    term           → factor ( ( "-" | "+" ) factor )* ;
    private Expr term() {
        Expr expr = factor();
        while (match(MINUS,PLUS)) {
            Token operator = previous();
            Expr right = factor();
            expr = new Expr.Binary(expr,operator,right);
        }
        return expr;
    }

//    factor         → unary ( ( "/" | "*" ) unary )* ;
   private Expr factor() {
        Expr expr = unary();
        while (match(SLASH,STAR)) {
            Token operator = previous();
            Expr right = unary();
            expr = new Expr.Binary(expr,operator,right);
        }
        return expr;
   }
//    unary          → ( "!" | "-" ) unary | primary ;
    private Expr unary() {
        if (match(BANG,MINUS)) {
            Token operator = previous();
            Expr right = unary();
            return new Expr.Unary(operator,right);
        }
        return primary();
    }
//    primary        → NUMBER | STRING | "true" | "false" | "nil"  | "(" expression ")" ;
    private Expr primary() {
        if (match(FALSE)) return new Expr.Literal(false);
        if (match(TRUE)) return new Expr.Literal(true);
        if (match(NIL)) return new Expr.Literal(null);

        if (match(NUMBER,STRING)) {
            return new Expr.Literal(previous().getLiteral());
        }
        if (match(LEFT_PAREN)) {
            Expr expr = expression();
            consume(RIGHT_PAREN, "Expect ')' after expression");
            return new Expr.Grouping(expr);
        }

    }


};
