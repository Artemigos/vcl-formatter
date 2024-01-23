use std::io::Write;

use crate::ast::*;

#[derive(Debug)]
pub enum E {
    IO(std::io::Error),
    UnexpectedStringList,
    ExpectedStringList,
}

impl From<std::io::Error> for E {
    fn from(value: std::io::Error) -> Self {
        E::IO(value)
    }
}

type R = Result<(), E>;

pub struct Emitter<'a> {
    w: &'a mut dyn Write,
    ind: usize,
    ci: usize,
}

impl<'a> Emitter<'a> {
    pub fn new(writer: &'a mut dyn Write, indent: usize) -> Self {
        Self {
            w: writer,
            ind: indent,
            ci: 0,
        }
    }

    pub fn emit(&mut self, sf: &SourceFile) -> R {
        for td in sf {
            self.emit_toplevel_declaration(td)?;
        }
        Ok(())
    }

    fn emit_indent(&mut self) -> R {
        write!(self.w, "{:<i$}", "", i = self.ci * self.ind)?;
        Ok(())
    }

    fn emit_toplevel_declaration(&mut self, td: &TopLevelDeclaration) -> R {
        match td {
            TopLevelDeclaration::VclVersion(v) => self.emit_vcl_version(v)?,
            TopLevelDeclaration::Import { name, from } => self.emit_import(name, *from)?,
            TopLevelDeclaration::Include(i) => self.emit_include(i)?,
            TopLevelDeclaration::Acl { name, entries } => self.emit_acl(name, entries)?,
            TopLevelDeclaration::Backend { name, properties } => {
                self.emit_backend(name, properties.as_ref())?
            }
            TopLevelDeclaration::Probe { name, properties } => self.emit_probe(name, properties)?,
            TopLevelDeclaration::Sub { name, statements } => todo!(),
        };

        Ok(())
    }

    fn emit_vcl_version(&mut self, v: &str) -> R {
        writeln!(self.w, "vcl {v};")?;
        Ok(())
    }

    fn emit_import(&mut self, name: &str, from: Option<&str>) -> R {
        match from {
            Some(f) => writeln!(self.w, "import {name} from {f};")?,
            None => writeln!(self.w, "import {name};")?,
        };
        Ok(())
    }

    fn emit_include(&mut self, inc: &str) -> R {
        writeln!(self.w, "include {inc};")?;
        Ok(())
    }

    fn emit_acl(&mut self, name: &str, entries: &Vec<AclEntry>) -> R {
        writeln!(self.w, "acl {name} {{")?;
        self.ci += 1;
        for entry in entries {
            self.emit_acl_entry(entry)?;
        }
        self.ci -= 1;
        writeln!(self.w, "}}")?;
        Ok(())
    }

    fn emit_acl_entry(&mut self, e: &AclEntry) -> R {
        let v = e.value;
        self.emit_indent()?;
        match e.mask {
            Some(m) => writeln!(self.w, "{v}/{m};")?,
            None => writeln!(self.w, "{v};")?,
        };
        Ok(())
    }

    fn emit_probe(&mut self, name: &str, properties: &Vec<BackendProperty>) -> R {
        writeln!(self.w, "probe {name} {{")?;
        self.ci += 1;
        for prop in properties {
            self.emit_backend_property(prop.name, &prop.value)?;
        }
        self.ci -= 1;
        writeln!(self.w, "}}")?;
        Ok(())
    }

    fn emit_backend_property(&mut self, name: &str, value: &BackendValue) -> R {
        self.emit_indent()?;
        write!(self.w, "{name} =")?;
        match &value {
            BackendValue::Expression(e) => {
                write!(self.w, " ")?;
                self.emit_expression(e)?;
                writeln!(self.w, ";")?;
            }
            BackendValue::Composite(props) => {
                writeln!(self.w, " {{")?;
                self.ci += 1;
                for prop in props {
                    self.emit_backend_property(prop.name, &prop.value)?;
                }
                self.ci -= 1;
                self.emit_indent();
                writeln!(self.w, "}}")?;
            }
            BackendValue::StringList(l) => {
                self.ci += 1;
                for val in l {
                    writeln!(self.w)?;
                    self.emit_indent();
                    write!(self.w, "{val}")?;
                }
                self.ci -= 1;
                writeln!(self.w, ";")?;
            }
        };
        Ok(())
    }

    fn emit_expression(&mut self, expr: &Expression) -> R {
        match expr {
            Expression::Ident(i) => write!(self.w, "{i}")?,
            Expression::Literal(l) => write!(self.w, "{l}")?,
            Expression::Neg(e) => {
                write!(self.w, "!")?;
                self.emit_expression(e)?;
            }
            Expression::Binary { left, op, right } => {
                self.emit_expression(left)?;
                write!(self.w, " {op} ")?;
                self.emit_expression(right)?;
            }
            Expression::IdentCall(e) => {
                self.emit_ident_call(e)?;
            }
        };
        Ok(())
    }

    fn emit_ident_call(&mut self, e: &IdentCallExpression) -> R {
        let n = e.name;
        write!(self.w, "{n}(")?;
        let mut first = true;
        for arg in &e.args {
            if first {
                first = false;
            } else {
                write!(self.w, ", ")?;
            };
            match arg {
                FunctionCallArg::Named { name, value } => {
                    write!(self.w, "{name}=")?;
                    self.emit_expression(value)?;
                }
                FunctionCallArg::Positional(p) => self.emit_expression(p)?,
            };
        }
        write!(self.w, ")")?;
        Ok(())
    }

    fn emit_backend(&mut self, name: &str, properties: Option<&Vec<BackendProperty>>) -> R {
        match properties {
            Some(p) => {
                writeln!(self.w, "backend {name} {{")?;
                self.ci += 1;
                for prop in p {
                    self.emit_backend_property(prop.name, &prop.value)?;
                }
                self.ci -= 1;
                writeln!(self.w, "}}")?;
            }
            None => writeln!(self.w, "backend {name} none;")?,
        };
        Ok(())
    }
}
