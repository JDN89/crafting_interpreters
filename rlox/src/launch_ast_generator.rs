use std::fs::File;
use std::io::Write;

fn main() {
    if let Err(e) = generate_ast(
        "src",
        "expr",
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

            writeln!(file, "    {}({}),", class_name, fields)?;
        }

        writeln!(file, "}}\n")?;

        for type_def in types {
            let type_def_parts: Vec<&str> = type_def.split(':').map(|s| s.trim()).collect();
            let class_name = type_def_parts[0].trim();
            let fields = type_def_parts[1].trim();

            writeln!(file, "pub struct {} {{", class_name)?;

            // Constructor
            writeln!(file, "    pub fn new({}) -> Self {{", fields)?;
            for field in fields.split(", ") {
                let field_name = field.split_whitespace().last().unwrap();
                writeln!(file, "        {}: {},", field_name, field)?;
            }
            writeln!(file, "        {} {{ {} }}", class_name, fields)?;
            writeln!(file, "    }}\n")?;

            // Visitor pattern
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

            // Fields
            for field in fields.split(", ") {
                writeln!(file, "    pub {},", field)?;
            }

            writeln!(file, "}}\n")?;
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

        Ok(())
    }
}