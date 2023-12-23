use std::fs::File;
use std::io::Write;

fn main() {
    if let Err(e) = generate_ast(
        "src/lox",
        "Stmt",
        &[
            "Expression: Expr expression",
            "Print: Expr expression",
            "Var : Token name, Option<Expr> initializer"
        ],
    ) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = generate_ast(
        "src/lox",
        "Expr",
        &[
            "Binary: Box<Expr> left, Token operator, Box<Expr> right",
            "Grouping: Box<Expr> expression",
            "Literal: Literal value",
            "Unary: Token operator, Box<Expr> right",
            "Variable: Token name"
        ],
    ) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    fn generate_ast(output_dir: &str, base_name: &str, types: &[&str]) -> std::io::Result<()> {
        let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
        let mut file = File::create(&path)?;

        writeln!(file, "use crate::token::{{Token, Literal}};")?;
        writeln!(file, "use crate::lox_error::LoxError;")?;
        writeln!(file, "\n#[derive(Debug)]")?;

        // Declare the types of expressions in an enum value
        writeln!(file, "pub enum {} {{", base_name)?;

        for type_def in types {
            let type_def_parts: Vec<&str> = type_def.split(':').map(|s| s.trim()).collect();
            let class_name = type_def_parts[0].trim();

            writeln!(file, "    {}({}{}),", class_name, class_name, base_name)?;
        }

        writeln!(file, "}}\n")?;

        // impl accept method for enum members
        writeln!(file, "impl {} {{", base_name)?;

        writeln!(
            file,
            "    pub fn accept<R>(&self, visitor: &dyn {}Visitor<R>) -> Result<R, LoxError> {{",
            base_name
        )?;

        writeln!(file, "match self {{")?;

        for type_def in types {
            let type_def_parts: Vec<&str> = type_def.split(':').map(|s| s.trim()).collect();
            let class_name = type_def_parts[0].trim();

            writeln!(
                file,
                "    {}::{} ({}) => visitor.visit_{}(&{}),",
                base_name,
                class_name,
                base_name.to_lowercase(),
                class_name.to_lowercase(),
                base_name.to_lowercase()
            )?;
        }
        writeln!(file, "    }}\n")?;
        writeln!(file, "    }}\n")?;
        writeln!(file, "    }}\n")?;

        // Declare the productions under Expression (each kind of expression)
        for type_def in types {
            let type_def_parts: Vec<&str> = type_def.split(':').map(|s| s.trim()).collect();
            let class_name = type_def_parts[0].trim();
            let fields = type_def_parts[1].trim();

            writeln!(file, "\n#[derive(Debug)]")?;
            writeln!(file, "pub struct {}{} {{", class_name, base_name)?;

            // Constructor
            for field in fields.split(", ") {
                let mut iter = field.split_whitespace();
                let field_type = iter.nth(0).unwrap();
                let field_name = iter.last().unwrap();
                // let field_name = field.split_whitespace().last().unwrap();
                // let field_type = field.split_whitespace()
                writeln!(file, "pub {}: {},", field_name, field_type)?;
            }
            writeln!(file, "    }}\n")?;
        }

        // Visitor trait
        writeln!(file, "pub trait {}Visitor<R> {{", base_name)?;
        for type_def in types {
            let type_name = type_def.split(':').next().unwrap().trim();
            writeln!(
                file,
                "    fn visit_{}(&self, {}: &{}{}) -> Result<R, LoxError>;",
                type_name.to_lowercase(),
                base_name.to_lowercase(),
                type_name,
                base_name
            )?;
        }
        writeln!(file, "}}")?;

        Ok(())
    }
}
