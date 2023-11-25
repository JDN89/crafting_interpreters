use std::fs::File;
use std::io::Write;

fn main() {
    if let Err(e) = generate_ast(
        "src",
        "Expr",
        &[
            "Binary: Box<Expr> left, Token operator, Box<Expr> right",
            "Grouping: Box<Expr> expression",
            "Literal: Literal literal",
            "Unary: Token operator, Box<Expr> right",
        ],
    ) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    fn generate_ast(output_dir: &str, base_name: &str, types: &[&str]) -> std::io::Result<()> {
        let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
        let mut file = File::create(&path)?;

        writeln!(file, "use crate::token::Token;")?;
        writeln!(file, "use crate::literal::Literal;")?;
        writeln!(file, "\n#[derive(Debug)]")?;
        writeln!(file, "pub enum {} {{", base_name)?;

        for type_def in types {
            let type_def_parts: Vec<&str> = type_def.split(':').map(|s| s.trim()).collect();
            let class_name = type_def_parts[0].trim();
            let fields = type_def_parts[1].trim();

            writeln!(file, "    {}({}{}),", class_name, class_name, base_name)?;
        }

        writeln!(file, "}}\n")?;

        for type_def in types {
            let type_def_parts: Vec<&str> = type_def.split(':').map(|s| s.trim()).collect();
            let class_name = type_def_parts[0].trim();
            let fields = type_def_parts[1].trim();

            writeln!(file, "pub struct {}{} {{", class_name, base_name)?;

            // Constructor
            for field in fields.split(", ") {
                let mut iter = field.split_whitespace();
                let field_type = iter.nth(0).unwrap();
                let field_name = iter.last().unwrap();
                // let field_name = field.split_whitespace().last().unwrap();
                // let field_type = field.split_whitespace()
                writeln!(file, "        {}: {},", field_name, field_type)?;
            }
            writeln!(file, "    }}\n")?;

            // put in impl of expr
            // Visitor pattern
            // writeln!(
            //     file,
            //     "    pub fn accept<R>(&self, visitor: &dyn {}Visitor<R>) -> Result<R, LoxError> {{",
            //     base_name
            // )?;
            // writeln!(
            //     file,
            //     "        visitor.visit_{}(self)",
            //     class_name.to_lowercase()
            // )?;
            // writeln!(file, "    }}\n")?;

            // Fields
        }

        // Visitor trait
        writeln!(file, "pub trait {}Visitor<R> {{", base_name)?;
        for type_def in types {
            let type_name = type_def.split(':').next().unwrap().trim();
            writeln!(
                file,
                "    fn visit_{}(&self, {}expr: &{}) -> Result<R, LoxError>;",
                type_name.to_lowercase(),
                base_name.to_lowercase(),
                type_name
            )?;
        }
        writeln!(file, "}}")?;

        for type_def in types {
            let type_def_parts: Vec<&str> = type_def.split(':').map(|s| s.trim()).collect();
            let class_name = type_def_parts[0].trim();

            writeln!(file, "impl {}{} {{", class_name, base_name)?;
            writeln!(
                file,
                "    pub fn accept<R>(&self, visitor: &dyn {}Visitor<R>) -> Result<R, LoxError> {{",
                base_name
            )?;
            writeln!(
                file,
                "        visitor.visit_{}(self)",
                class_name.to_lowercase()
            )?;
            writeln!(file, "    }}\n")?;
            writeln!(file, "    }}\n")?;
        }

        Ok(())
    }
}
