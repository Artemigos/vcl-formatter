use std::io::Write;

// TODO: control indent after optional line breaks

pub trait Emitter {
    fn vcl_keyword(&mut self);
    fn number(&mut self, num: &str);
    fn semicolon(&mut self);
    fn include_keyword(&mut self);
    fn string(&mut self, string: &str);
    fn import_keyword(&mut self);
    fn ident(&mut self, ident: &str);
    fn from_keyword(&mut self);
    fn probe_keyword(&mut self);
    fn body_start(&mut self);
    fn body_end(&mut self);
    fn prefix_operator(&mut self, op: &str);
    fn string_list_entry(&mut self, entry: &str);
    fn expression(&mut self, expr: &str);
    fn infix_operator(&mut self, op: &str);
    fn backend_keyword(&mut self);
    fn none_keyword(&mut self);
    fn acl_keyword(&mut self);
    fn acl_mask_op(&mut self);
    fn sub_keyword(&mut self);
    fn set_keyword(&mut self);
    fn call_keyword(&mut self);
    fn call_ident(&mut self, ident: &str);
    fn l_paren(&mut self);
    fn r_paren(&mut self);
    fn comma(&mut self);
    fn unset_keyword(&mut self);
    fn if_keyword(&mut self);
    fn else_keyword(&mut self);
    fn return_keyword(&mut self);
    fn varnish_step_keyword(&mut self, step: &str);
    fn comment(&mut self, comment: &str);
    fn newlines(&mut self, how_many: usize);
    fn file_end(&mut self);
}

pub struct StandardEmitter<'a> {
    write: &'a mut dyn Write,
    indent_step: usize,
    needs_whitespace: bool,
    new_line: bool,
    in_string_list: bool,
    new_line_pending: bool,
    allow_line_break: bool,
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
            new_line_pending: false,
            allow_line_break: false,
            nest_level: 0,
            materialized_nest_levels: vec![],
        }
    }

    fn flush_preceding_whitespace(&mut self) {
        if self.new_line_pending {
            self.line();
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
            )
            .unwrap();
        } else if self.needs_whitespace {
            write!(self.write, " ").unwrap();
        }
        self.new_line = false;
        self.needs_whitespace = false;
        self.new_line_pending = false;
        self.allow_line_break = false;
    }

    fn line(&mut self) {
        writeln!(self.write).unwrap();
        self.new_line = true;
        self.new_line_pending = false;
    }

    fn increase_nest(&mut self) {
        self.nest_level += 1;
    }

    fn decrease_nest(&mut self) {
        assert!(self.nest_level > 0);
        if self.last_nest() == self.nest_level {
            self.materialized_nest_levels.pop();
        }
        self.nest_level -= 1;
    }

    fn last_nest(&self) -> usize {
        if self.materialized_nest_levels.len() == 0 {
            0
        } else {
            self.materialized_nest_levels[self.materialized_nest_levels.len() - 1]
        }
    }
}

impl<'a> Emitter for StandardEmitter<'a> {
    fn vcl_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "vcl").unwrap();
        self.needs_whitespace = true;
    }

    fn number(&mut self, num: &str) {
        self.flush_preceding_whitespace();
        write!(self.write, "{}", num).unwrap();
    }

    fn semicolon(&mut self) {
        self.needs_whitespace = false;
        write!(self.write, ";").unwrap();
        self.new_line_pending = true;

        if self.in_string_list {
            self.in_string_list = false;
            self.decrease_nest();
        }
    }

    fn include_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "include").unwrap();
        self.needs_whitespace = true;
    }

    fn string(&mut self, string: &str) {
        self.flush_preceding_whitespace();
        write!(self.write, "{}", string).unwrap();
        self.needs_whitespace = true;
    }

    fn import_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "import").unwrap();
        self.needs_whitespace = true;
    }

    fn ident(&mut self, ident: &str) {
        self.flush_preceding_whitespace();
        write!(self.write, "{}", ident).unwrap();
        self.needs_whitespace = true;
    }

    fn from_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "from").unwrap();
        self.needs_whitespace = true;
    }

    fn probe_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "probe").unwrap();
        self.needs_whitespace = true;
    }

    fn body_start(&mut self) {
        self.needs_whitespace = false;
        write!(self.write, " {{").unwrap();
        self.new_line_pending = true;
        self.increase_nest();
    }

    fn body_end(&mut self) {
        self.decrease_nest();
        self.flush_preceding_whitespace();
        write!(self.write, "}}").unwrap();
        self.new_line_pending = true;
    }

    fn prefix_operator(&mut self, op: &str) {
        self.flush_preceding_whitespace();
        write!(self.write, "{}", op).unwrap();
    }

    fn string_list_entry(&mut self, entry: &str) {
        if !self.in_string_list {
            self.in_string_list = true;
            self.increase_nest();
        } else {
            self.line();
        }

        self.flush_preceding_whitespace();
        write!(self.write, "{}", entry).unwrap();
    }

    fn expression(&mut self, expr: &str) {
        self.flush_preceding_whitespace();
        write!(self.write, "{}", expr).unwrap();
        self.needs_whitespace = true;
    }

    fn infix_operator(&mut self, op: &str) {
        self.needs_whitespace = false;
        write!(self.write, " {}", op).unwrap();
        self.needs_whitespace = true;
        self.allow_line_break = true;
    }

    fn backend_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "backend").unwrap();
        self.needs_whitespace = true;
    }

    fn none_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "none").unwrap();
        self.needs_whitespace = true;
    }

    fn acl_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "acl").unwrap();
        self.needs_whitespace = true;
    }

    fn acl_mask_op(&mut self) {
        self.needs_whitespace = false;
        write!(self.write, "/").unwrap();
    }

    fn sub_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "sub").unwrap();
        self.needs_whitespace = true;
    }

    fn set_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "set").unwrap();
        self.needs_whitespace = true;
    }

    fn call_ident(&mut self, ident: &str) {
        self.flush_preceding_whitespace();
        write!(self.write, "{}", ident).unwrap();
    }

    fn l_paren(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "(").unwrap();
        self.increase_nest();
    }

    fn r_paren(&mut self) {
        self.needs_whitespace = false;
        self.decrease_nest();
        write!(self.write, ")").unwrap();
    }

    fn comma(&mut self) {
        self.needs_whitespace = false;
        write!(self.write, ",").unwrap();
        self.needs_whitespace = true;
        self.allow_line_break = true;
    }

    fn unset_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "unset").unwrap();
        self.needs_whitespace = true;
    }

    fn if_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "if").unwrap();
        self.needs_whitespace = true;
    }

    fn else_keyword(&mut self) {
        self.new_line_pending = false;
        self.needs_whitespace = true;
        self.flush_preceding_whitespace();
        write!(self.write, "else").unwrap();
        self.needs_whitespace = true;
    }

    fn return_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "return").unwrap();
        self.needs_whitespace = true;
    }

    fn varnish_step_keyword(&mut self, step: &str) {
        self.flush_preceding_whitespace();
        write!(self.write, "{}", step).unwrap();
    }

    fn comment(&mut self, comment: &str) {
        self.new_line_pending = false;
        self.needs_whitespace = true;
        self.flush_preceding_whitespace();
        write!(self.write, "{}", comment).unwrap();
        self.new_line_pending = true;
    }

    fn newlines(&mut self, how_many: usize) {
        assert!(how_many > 0);
        if self.new_line_pending {
            self.line();
            if how_many > 1 {
                self.line();
            }
        } else if self.allow_line_break {
            self.line();
        }

        self.new_line_pending = false;
        self.allow_line_break = false;
    }

    fn file_end(&mut self) {
        if self.new_line_pending {
            self.line();
        }
    }

    fn call_keyword(&mut self) {
        self.flush_preceding_whitespace();
        write!(self.write, "call").unwrap();
        self.needs_whitespace = true;
    }
}

#[cfg(test)]
mod test {
    use crate::emitter::Emitter;

    #[test]
    fn emit_example_vcl() {
        let buf: Vec<u8> = Vec::new();
        let mut writer = std::io::BufWriter::new(buf);
        let mut e = crate::emitter::StandardEmitter::new(&mut writer, 4);

        // vcl version declaration
        e.vcl_keyword();
        e.number("4.1");
        e.semicolon();
        e.newlines(2);

        // include
        e.include_keyword();
        e.string("\"vha6/whatever\"");
        e.semicolon();
        e.newlines(1);

        // import 1
        e.import_keyword();
        e.ident("std");
        e.semicolon();
        e.newlines(1);

        // import 2
        e.import_keyword();
        e.ident("not_std");
        e.from_keyword();
        e.string("\"not_std.vcl\"");
        e.semicolon();
        e.newlines(2);

        // probe
        e.probe_keyword();
        e.ident("my_probe");
        e.body_start();
        e.newlines(1);

        // .request
        e.prefix_operator(".");
        e.ident("request");
        e.infix_operator("=");
        e.newlines(1);
        e.string_list_entry("\"HEAD / HTTP/1.1\"");
        e.newlines(1);
        e.string_list_entry("\"Host: localhost\"");
        e.newlines(1);
        e.string_list_entry("\"Connection: close\"");
        e.newlines(1);
        e.string_list_entry("\"User-Agent: Varnish Health Probe\"");
        e.semicolon();
        e.newlines(1);

        // .interval
        e.prefix_operator(".");
        e.ident("interval");
        e.infix_operator("=");
        e.expression("10s");
        e.semicolon();
        e.newlines(1);

        // .timeout
        e.prefix_operator(".");
        e.ident("timeout");
        e.infix_operator("=");
        e.expression("5s");
        e.semicolon();
        e.newlines(1);

        // .window
        e.prefix_operator(".");
        e.ident("window");
        e.infix_operator("=");
        e.expression("5");
        e.semicolon();
        e.newlines(1);

        // .threshold
        e.prefix_operator(".");
        e.ident("threshold");
        e.infix_operator("=");
        e.expression("3");
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // default backend
        e.backend_keyword();
        e.ident("default");
        e.none_keyword();
        e.semicolon();
        e.newlines(2);

        // server1 backend
        e.backend_keyword();
        e.ident("server1");
        e.body_start();
        e.newlines(1);

        // .host
        e.prefix_operator(".");
        e.ident("host");
        e.infix_operator("=");
        e.expression("\"127.0.0.1\"");
        e.semicolon();
        e.newlines(1);

        // .port
        e.prefix_operator(".");
        e.ident("port");
        e.infix_operator("=");
        e.expression("\"8080\"");
        e.semicolon();
        e.newlines(1);

        // .max_connections
        e.prefix_operator(".");
        e.ident("max_connections");
        e.infix_operator("=");
        e.expression("100");
        e.semicolon();
        e.newlines(1);

        // .probe
        e.prefix_operator(".");
        e.ident("probe");
        e.infix_operator("=");
        e.expression("my_probe");
        e.semicolon();
        e.newlines(1);

        // .connect_timeout
        e.prefix_operator(".");
        e.ident("connect_timeout");
        e.infix_operator("=");
        e.expression("5s");
        e.semicolon();
        e.newlines(1);

        // .first_byte_timeout
        e.prefix_operator(".");
        e.ident("first_byte_timeout");
        e.infix_operator("=");
        e.expression("90s");
        e.semicolon();
        e.newlines(1);

        // .between_bytes_timeout
        e.prefix_operator(".");
        e.ident("between_bytes_timeout");
        e.infix_operator("=");
        e.expression("2s");
        e.semicolon();
        e.newlines(1);

        // .asdf
        e.prefix_operator(".");
        e.ident("asdf");
        e.infix_operator("=");
        e.prefix_operator("!");
        e.expression("true");
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // acl
        e.acl_keyword();
        e.ident("purge");
        e.body_start();
        e.newlines(1);
        e.string("\"localhost\"");
        e.semicolon();
        e.newlines(1);
        e.string("\"127.0.0.1\"");
        e.acl_mask_op();
        e.number("16");
        e.semicolon();
        e.newlines(1);
        e.string("\"::1\"");
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // sub vcl_recv
        e.sub_keyword();
        e.ident("vcl_recv");
        e.body_start();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Host");
        e.infix_operator("=");
        e.call_ident("regsub");
        e.l_paren();
        e.expression("req.http.Host");
        e.comma();
        e.expression("\":[0-9]+\"");
        e.comma();
        e.expression("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // unset statement
        e.unset_keyword();
        e.ident("req.http.proxy");
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.url");
        e.infix_operator("=");
        e.call_ident("std.querysort");
        e.l_paren();
        e.expression("req.url");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.url");
        e.infix_operator("=");
        e.call_ident("regsub");
        e.l_paren();
        e.expression("req.url");
        e.comma();
        e.expression("\"\\?$\"");
        e.comma();
        e.expression("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Surrogate-Capability");
        e.infix_operator("=");
        e.expression("\"key=ESI/1.0\"");
        e.semicolon();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.call_ident("std.healthy");
        e.l_paren();
        e.expression("req.backend_hint");
        e.r_paren();
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.grace");
        e.infix_operator("=");
        e.expression("10s");
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.prefix_operator("!");
        e.expression("req.http.X-Forwarded-Proto");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.call_ident("std.port");
        e.l_paren();
        e.expression("server.ip");
        e.r_paren();
        e.infix_operator("==");
        e.expression("443");
        e.infix_operator("||");
        e.call_ident("std.port");
        e.l_paren();
        e.expression("server.ip");
        e.r_paren();
        e.infix_operator("==");
        e.expression("8443");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.X-Forwarded-Proto");
        e.infix_operator("=");
        e.expression("\"https\"");
        e.semicolon();
        e.newlines(1);
        e.body_end();

        // else statement
        e.else_keyword();
        e.body_start();

        // set statement
        e.set_keyword();
        e.ident("req.http.X-Forwarded-Proto");
        e.infix_operator("=");
        e.expression("\"https\"");
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.expression("req.http.Upgrade");
        e.infix_operator("~");
        e.expression("\"(?i)websocket\"");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // return statement
        e.return_keyword();
        e.l_paren();
        e.varnish_step_keyword("pipe");
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.expression("req.url");
        e.infix_operator("~");
        e.expression(
            "\"(\\?|&)(utm_source|utm_medium|utm_campaign|utm_content|gclid|cx|ie|cof|siteurl)=\"",
        );
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.url");
        e.infix_operator("=");
        e.call_ident("regsuball");
        e.l_paren();
        e.expression("req.url");
        e.comma();
        e.expression("\"&(utm_source|utm_medium|utm_campaign|utm_content|gclid|cx|ie|cof|siteurl)=([A-z0-9_\\-\\.%25]+)\"");
        e.comma();
        e.expression("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.url");
        e.infix_operator("=");
        e.call_ident("regsuball");
        e.l_paren();
        e.expression("req.url");
        e.comma();
        e.expression("\"\\?(utm_source|utm_medium|utm_campaign|utm_content|gclid|cx|ie|cof|siteurl)=([A-z0-9_\\-\\.%25]+)\"");
        e.comma();
        e.expression("\"?\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.url");
        e.infix_operator("=");
        e.call_ident("regsub");
        e.l_paren();
        e.expression("req.url");
        e.comma();
        e.expression("\"\\?&\"");
        e.comma();
        e.expression("\"?\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.url");
        e.infix_operator("=");
        e.call_ident("regsub");
        e.l_paren();
        e.expression("req.url");
        e.comma();
        e.expression("\"\\?$\"");
        e.comma();
        e.expression("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.expression("req.method");
        e.infix_operator("==");
        e.expression("\"PURGE\"");
        e.r_paren();
        e.body_start();
        e.comment("// test");
        e.newlines(1);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.prefix_operator("!");
        e.expression("client.ip");
        e.infix_operator("~");
        e.expression("purge");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // return statement
        e.return_keyword();
        e.l_paren();
        e.varnish_step_keyword("synth");
        e.l_paren();
        e.expression("405");
        e.comma();
        e.expression("client.ip");
        e.infix_operator("+");
        e.expression("\" is not allowed to send PURGE requests.\"");
        e.r_paren();
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // return statement
        e.comment("# test");
        e.newlines(1);
        e.return_keyword();
        e.l_paren();
        e.varnish_step_keyword("purge");
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);
        e.comment("/*\n        test\n    */");
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.expression("req.method");
        e.infix_operator("!=");
        e.expression("\"GET\"");
        e.infix_operator("&&");
        e.expression("req.method");
        e.infix_operator("!=");
        e.expression("\"HEAD\"");
        e.infix_operator("&&");
        e.expression("req.method");
        e.infix_operator("!=");
        e.expression("\"PUT\"");
        e.infix_operator("&&");
        e.expression("req.method");
        e.infix_operator("!=");
        e.expression("\"POST\"");
        e.infix_operator("&&");
        e.expression("req.method");
        e.infix_operator("!=");
        e.expression("\"TRACE\"");
        e.infix_operator("&&");
        e.expression("req.method");
        e.infix_operator("!=");
        e.expression("\"OPTIONS\"");
        e.infix_operator("&&");
        e.expression("req.method");
        e.infix_operator("!=");
        e.expression("\"PATCH\"");
        e.infix_operator("&&");
        e.expression("req.method");
        e.infix_operator("!=");
        e.expression("\"DELETE\"");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // return statement
        e.return_keyword();
        e.l_paren();
        e.varnish_step_keyword("pipe");
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.expression("req.method");
        e.infix_operator("!=");
        e.expression("\"GET\"");
        e.infix_operator("&&");
        e.expression("req.method");
        e.infix_operator("!=");
        e.expression("\"HEAD\"");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // return statement
        e.return_keyword();
        e.l_paren();
        e.varnish_step_keyword("pass");
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.expression("req.url");
        e.infix_operator("~");
        e.expression("\"^[^?]*\\.(7z|avi|bmp|bz2|css|csv|doc|docx|eot|flac|flv|gif|gz|ico|jpeg|jpg|js|less|mka|mkv|mov|mp3|mp4|mpeg|mpg|odt|ogg|ogm|opus|otf|pdf|png|ppt|pptx|rar|rtf|svg|svgz|swf|tar|tbz|tgz|ttf|txt|txz|wav|webm|webp|woff|woff2|xls|xlsx|xml|xz|zip)(\\?.*)?$\"");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // unset statement
        e.unset_keyword();
        e.ident("req.http.Cookie");
        e.semicolon();
        e.newlines(1);

        // return statement
        e.return_keyword();
        e.l_paren();
        e.varnish_step_keyword("hash");
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // set statement
        e.set_keyword();
        e.ident("req.http.Cookie");
        e.infix_operator("=");
        e.call_ident("regsuball");
        e.l_paren();
        e.expression("req.http.Cookie");
        e.comma();
        e.expression("\"(__utm|_ga|_opt)[a-z_]*=[^;]+(; )?\"");
        e.comma();
        e.expression("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Cookie");
        e.infix_operator("=");
        e.call_ident("regsuball");
        e.l_paren();
        e.expression("req.http.Cookie");
        e.comma();
        e.expression("\"(__)?hs[a-z_\\-]+=[^;]+(; )?\"");
        e.comma();
        e.expression("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Cookie");
        e.infix_operator("=");
        e.call_ident("regsuball");
        e.l_paren();
        e.expression("req.http.Cookie");
        e.comma();
        e.expression("\"hubspotutk=[^;]+(; )?\"");
        e.comma();
        e.expression("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Cookie");
        e.infix_operator("=");
        e.call_ident("regsuball");
        e.l_paren();
        e.expression("req.http.Cookie");
        e.comma();
        e.expression("\"_hj[a-zA-Z]+=[^;]+(; )?\"");
        e.comma();
        e.expression("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Cookie");
        e.infix_operator("=");
        e.call_ident("regsuball");
        e.l_paren();
        e.expression("req.http.Cookie");
        e.comma();
        e.expression("\"(NID|DSID|__gads|GED_PLAYLIST_ACTIVITY|ACLK_DATA|ANID|AID|IDE|TAID|_gcl_[a-z]*|FLC|RUL|PAIDCONTENT|1P_JAR|Conversion|VISITOR_INFO1[a-z_]*)=[^;]+(; )?\"");
        e.comma();
        e.expression("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Cookie");
        e.infix_operator("=");
        e.call_ident("regsuball");
        e.l_paren();
        e.expression("req.http.Cookie");
        e.comma();
        e.expression("\"^;\\s*\"");
        e.comma();
        e.expression("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.expression("req.http.cookie");
        e.infix_operator("~");
        e.expression("\"^\\s*$\"");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // unset statement
        e.unset_keyword();
        e.ident("req.http.cookie");
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // sub vcl_hash
        e.sub_keyword();
        e.ident("vcl_hash");
        e.body_start();
        e.newlines(1);
        e.call_ident("hash_data");
        e.l_paren();
        e.expression("req.http.X-Forwarded-Proto");
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // sub vcl_backend_response
        e.sub_keyword();
        e.ident("vcl_backend_response");
        e.body_start();
        e.newlines(1);
        e.if_keyword();
        e.l_paren();
        e.expression("bereq.url");
        e.infix_operator("~");
        e.expression("\"^[^?]*\\.(7z|avi|bmp|bz2|css|csv|doc|docx|eot|flac|flv|gif|gz|ico|jpeg|jpg|js|less|mka|mkv|mov|mp3|mp4|mpeg|mpg|odt|ogg|ogm|opus|otf|pdf|png|ppt|pptx|rar|rtf|svg|svgz|swf|tar|tbz|tgz|ttf|txt|txz|wav|webm|webp|woff|woff2|xls|xlsx|xml|xz|zip)(\\?.*)?$\"");
        e.r_paren();
        e.body_start();
        e.newlines(1);
        e.unset_keyword();
        e.ident("beresp.http.Set-Cookie");
        e.semicolon();
        e.newlines(1);
        e.set_keyword();
        e.ident("beresp.ttl");
        e.infix_operator("=");
        e.expression("1d");
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);
        e.if_keyword();
        e.l_paren();
        e.expression("beresp.http.Surrogate-Control");
        e.infix_operator("~");
        e.expression("\"ESI/1.0\"");
        e.r_paren();
        e.body_start();
        e.newlines(1);
        e.unset_keyword();
        e.ident("beresp.http.Surrogate-Control");
        e.semicolon();
        e.newlines(1);
        e.set_keyword();
        e.ident("beresp.do_esi");
        e.infix_operator("=");
        e.expression("true");
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);
        e.set_keyword();
        e.ident("beresp.grace");
        e.infix_operator("=");
        e.expression("6h");
        e.semicolon();
        e.newlines(1);
        e.body_end();

        e.file_end();

        assert_eq!(writer.buffer(), crate::EXAMPLE);
    }
}
