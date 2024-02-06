use std::io::Write;

use crate::error::*;

pub trait Emitter {
    fn vcl_keyword(&mut self) -> R;
    fn number(&mut self, num: &str) -> R;
    fn semicolon(&mut self) -> R;
    fn include_keyword(&mut self) -> R;
    fn string(&mut self, string: &str) -> R;
    fn import_keyword(&mut self) -> R;
    fn ident(&mut self, ident: &str) -> R;
    #[allow(clippy::wrong_self_convention)]
    fn from_keyword(&mut self) -> R;
    fn probe_keyword(&mut self) -> R;
    fn body_start(&mut self) -> R;
    fn body_end(&mut self) -> R;
    fn prefix_operator(&mut self, op: &str) -> R;
    fn infix_operator(&mut self, op: &str) -> R;
    fn backend_keyword(&mut self) -> R;
    fn none_keyword(&mut self) -> R;
    fn acl_keyword(&mut self) -> R;
    fn sub_keyword(&mut self) -> R;
    fn set_keyword(&mut self) -> R;
    fn call_keyword(&mut self) -> R;
    fn new_keyword(&mut self) -> R;
    fn l_paren(&mut self) -> R;
    fn r_paren(&mut self) -> R;
    fn comma(&mut self) -> R;
    fn unset_keyword(&mut self) -> R;
    fn if_keyword(&mut self) -> R;
    fn else_keyword(&mut self) -> R;
    fn return_keyword(&mut self) -> R;
    fn comment(&mut self, comment: &str) -> R;
    fn newlines(&mut self, how_many: usize) -> R;
    fn file_end(&mut self) -> R;
    fn hint_string_list_start(&mut self);
}

pub struct StandardEmitter<'a> {
    write: &'a mut dyn Write,
    indent_step: usize,
    needs_whitespace: bool,
    new_line: bool,
    in_string_list: bool,
    in_acl: bool,
    new_line_pending: bool,
    allow_line_break: bool,
    ident_before_lparen: bool,
    nest_level: usize,
    materialized_nest_levels: Vec<usize>,
}

impl<'a> StandardEmitter<'a> {
    pub fn new(write: &'a mut dyn Write, indent_step: usize) -> Self {
        Self {
            write,
            indent_step,
            needs_whitespace: false,
            new_line: true,
            in_string_list: false,
            in_acl: false,
            new_line_pending: false,
            allow_line_break: false,
            ident_before_lparen: false,
            nest_level: 0,
            materialized_nest_levels: vec![],
        }
    }

    fn flush_preceding_whitespace(&mut self) -> R {
        if self.new_line_pending {
            self.line()?;
        }

        if self.new_line {
            if self.nest_level > self.last_nest() {
                self.materialized_nest_levels.push(self.nest_level);
            }

            write!(
                self.write,
                "{:<i$}",
                "",
                i = self.indent_step * self.materialized_nest_levels.len()
            )?;
        } else if self.needs_whitespace {
            write!(self.write, " ")?;
        }
        self.new_line = false;
        self.needs_whitespace = false;
        self.new_line_pending = false;
        self.allow_line_break = false;

        Ok(())
    }

    fn line(&mut self) -> R {
        writeln!(self.write)?;
        self.new_line = true;
        self.new_line_pending = false;

        Ok(())
    }

    fn increase_nest(&mut self) {
        self.nest_level += 1;
    }

    fn decrease_nest(&mut self) {
        assert!(self.nest_level > 0);
        if self.last_nest() == self.nest_level {
            let _ = self.materialized_nest_levels.pop();
        }
        self.nest_level -= 1;
    }

    fn last_nest(&self) -> usize {
        if self.materialized_nest_levels.is_empty() {
            0
        } else {
            self.materialized_nest_levels[self.materialized_nest_levels.len() - 1]
        }
    }

    fn keyword(&mut self, kw: &str) -> R {
        self.flush_preceding_whitespace()?;
        write!(self.write, "{kw}")?;
        self.needs_whitespace = true;

        Ok(())
    }
}

impl<'a> Emitter for StandardEmitter<'a> {
    fn vcl_keyword(&mut self) -> R {
        self.keyword("vcl")?;
        Ok(())
    }

    fn number(&mut self, num: &str) -> R {
        self.flush_preceding_whitespace()?;
        write!(self.write, "{}", num)?;

        Ok(())
    }

    fn semicolon(&mut self) -> R {
        self.needs_whitespace = false;
        write!(self.write, ";")?;
        self.new_line_pending = true;

        if self.in_string_list {
            self.in_string_list = false;
            self.decrease_nest();
        }

        Ok(())
    }

    fn include_keyword(&mut self) -> R {
        self.keyword("include")?;
        Ok(())
    }

    fn string(&mut self, string: &str) -> R {
        if self.in_string_list && !self.new_line {
            self.new_line_pending = true;
        }
        self.flush_preceding_whitespace()?;
        write!(self.write, "{}", string)?;
        self.needs_whitespace = true;

        Ok(())
    }

    fn import_keyword(&mut self) -> R {
        self.keyword("import")?;
        Ok(())
    }

    fn ident(&mut self, ident: &str) -> R {
        self.flush_preceding_whitespace()?;
        write!(self.write, "{}", ident)?;
        self.needs_whitespace = true;
        self.ident_before_lparen = true;

        Ok(())
    }

    fn from_keyword(&mut self) -> R {
        self.keyword("from")?;
        Ok(())
    }

    fn probe_keyword(&mut self) -> R {
        self.keyword("probe")?;
        Ok(())
    }

    fn body_start(&mut self) -> R {
        self.needs_whitespace = false;
        write!(self.write, " {{")?;
        self.new_line_pending = true;
        self.increase_nest();

        Ok(())
    }

    fn body_end(&mut self) -> R {
        self.decrease_nest();
        self.flush_preceding_whitespace()?;
        write!(self.write, "}}")?;
        self.new_line_pending = true;
        self.in_acl = false;

        Ok(())
    }

    fn prefix_operator(&mut self, op: &str) -> R {
        self.flush_preceding_whitespace()?;
        write!(self.write, "{}", op)?;

        Ok(())
    }

    fn infix_operator(&mut self, op: &str) -> R {
        self.needs_whitespace = false;
        if op == "/" && self.in_acl {
            write!(self.write, "{op}")?;
        } else {
            write!(self.write, " {op}")?;
            self.needs_whitespace = true;
            self.allow_line_break = true;
            self.ident_before_lparen = false;
        }

        Ok(())
    }

    fn backend_keyword(&mut self) -> R {
        self.keyword("backend")?;
        Ok(())
    }

    fn none_keyword(&mut self) -> R {
        self.keyword("none")?;
        Ok(())
    }

    fn acl_keyword(&mut self) -> R {
        self.keyword("acl")?;
        self.in_acl = true;
        Ok(())
    }

    fn sub_keyword(&mut self) -> R {
        self.keyword("sub")?;
        Ok(())
    }

    fn set_keyword(&mut self) -> R {
        self.keyword("set")?;
        Ok(())
    }

    fn l_paren(&mut self) -> R {
        if self.ident_before_lparen {
            self.needs_whitespace = false;
            self.ident_before_lparen = false;
        }
        self.flush_preceding_whitespace()?;
        write!(self.write, "(")?;
        self.increase_nest();

        Ok(())
    }

    fn r_paren(&mut self) -> R {
        self.needs_whitespace = false;
        self.decrease_nest();
        write!(self.write, ")")?;

        Ok(())
    }

    fn comma(&mut self) -> R {
        self.needs_whitespace = false;
        write!(self.write, ",")?;
        self.needs_whitespace = true;
        self.allow_line_break = true;

        Ok(())
    }

    fn unset_keyword(&mut self) -> R {
        self.keyword("unset")?;
        Ok(())
    }

    fn if_keyword(&mut self) -> R {
        self.keyword("if")?;
        self.ident_before_lparen = false;

        Ok(())
    }

    fn else_keyword(&mut self) -> R {
        self.new_line_pending = false;
        self.needs_whitespace = true;
        self.keyword("else")?;

        Ok(())
    }

    fn return_keyword(&mut self) -> R {
        self.keyword("return")?;
        self.ident_before_lparen = false;

        Ok(())
    }

    fn comment(&mut self, comment: &str) -> R {
        self.new_line_pending = false;
        self.needs_whitespace = true;
        self.flush_preceding_whitespace()?;
        write!(self.write, "{}", comment)?;
        self.new_line_pending = true;

        Ok(())
    }

    fn newlines(&mut self, how_many: usize) -> R {
        assert!(how_many > 0);
        if self.new_line_pending {
            self.line()?;
            if how_many > 1 {
                self.line()?;
            }
        } else if self.allow_line_break {
            self.line()?;
        }

        self.new_line_pending = false;
        self.allow_line_break = false;

        Ok(())
    }

    fn file_end(&mut self) -> R {
        if self.new_line_pending {
            self.line()?;
        }

        Ok(())
    }

    fn call_keyword(&mut self) -> R {
        self.keyword("call")?;
        Ok(())
    }

    fn new_keyword(&mut self) -> R {
        self.keyword("new")?;
        Ok(())
    }

    fn hint_string_list_start(&mut self) {
        self.in_string_list = true;
        self.increase_nest();
    }
}
