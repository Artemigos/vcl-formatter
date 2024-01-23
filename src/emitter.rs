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
    fn infix_operator(&mut self, op: &str);
    fn backend_keyword(&mut self);
    fn none_keyword(&mut self);
    fn acl_keyword(&mut self);
    fn sub_keyword(&mut self);
    fn set_keyword(&mut self);
    fn call_keyword(&mut self);
    fn new_keyword(&mut self);
    fn l_paren(&mut self);
    fn r_paren(&mut self);
    fn comma(&mut self);
    fn unset_keyword(&mut self);
    fn if_keyword(&mut self);
    fn else_keyword(&mut self);
    fn return_keyword(&mut self);
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
            let _ = self.materialized_nest_levels.pop();
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

    fn keyword(&mut self, kw: &str) {
        self.flush_preceding_whitespace();
        write!(self.write, "{kw}").unwrap();
        self.needs_whitespace = true;
    }
}

impl<'a> Emitter for StandardEmitter<'a> {
    fn vcl_keyword(&mut self) {
        self.keyword("vcl");
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
        self.keyword("include");
    }

    fn string(&mut self, string: &str) {
        self.flush_preceding_whitespace();
        write!(self.write, "{}", string).unwrap();
        self.needs_whitespace = true;
    }

    fn import_keyword(&mut self) {
        self.keyword("import");
    }

    fn ident(&mut self, ident: &str) {
        self.flush_preceding_whitespace();
        write!(self.write, "{}", ident).unwrap();
        self.needs_whitespace = true;
        self.ident_before_lparen = true;
    }

    fn from_keyword(&mut self) {
        self.keyword("from");
    }

    fn probe_keyword(&mut self) {
        self.keyword("probe");
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
        self.in_acl = false;
    }

    fn prefix_operator(&mut self, op: &str) {
        self.flush_preceding_whitespace();
        write!(self.write, "{}", op).unwrap();
    }

    fn infix_operator(&mut self, op: &str) {
        self.needs_whitespace = false;
        if op == "/" && self.in_acl {
            write!(self.write, "{op}").unwrap();
        } else {
            write!(self.write, " {op}").unwrap();
            self.needs_whitespace = true;
            self.allow_line_break = true;
            self.ident_before_lparen = false;
        }
    }

    fn backend_keyword(&mut self) {
        self.keyword("backend");
    }

    fn none_keyword(&mut self) {
        self.keyword("none");
    }

    fn acl_keyword(&mut self) {
        self.keyword("acl");
        self.in_acl = true;
    }

    fn sub_keyword(&mut self) {
        self.keyword("sub");
    }

    fn set_keyword(&mut self) {
        self.keyword("set");
    }

    fn l_paren(&mut self) {
        if self.ident_before_lparen {
            self.needs_whitespace = false;
            self.ident_before_lparen = false;
        }
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
        self.keyword("unset");
    }

    fn if_keyword(&mut self) {
        self.keyword("if");
        self.ident_before_lparen = false;
    }

    fn else_keyword(&mut self) {
        self.new_line_pending = false;
        self.needs_whitespace = true;
        self.keyword("else");
    }

    fn return_keyword(&mut self) {
        self.keyword("return");
        self.ident_before_lparen = false;
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
        self.keyword("call");
    }

    fn new_keyword(&mut self) {
        self.keyword("new");
    }
}

#[cfg(test)]
mod test {
    use crate::emitter::Emitter;

    #[test]
    fn emit_example_vcl() {
        // let buf: Vec<u8> = Vec::new();
        // let mut writer = std::io::BufWriter::new(buf);
        let mut writer = std::io::stdout().lock();
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
        e.string("\"HEAD / HTTP/1.1\"");
        e.newlines(1);
        e.string("\"Host: localhost\"");
        e.newlines(1);
        e.string("\"Connection: close\"");
        e.newlines(1);
        e.string("\"User-Agent: Varnish Health Probe\"");
        e.semicolon();
        e.newlines(1);

        // .interval
        e.prefix_operator(".");
        e.ident("interval");
        e.infix_operator("=");
        e.number("10s");
        e.semicolon();
        e.newlines(1);

        // .timeout
        e.prefix_operator(".");
        e.ident("timeout");
        e.infix_operator("=");
        e.number("5s");
        e.semicolon();
        e.newlines(1);

        // .window
        e.prefix_operator(".");
        e.ident("window");
        e.infix_operator("=");
        e.number("5");
        e.semicolon();
        e.newlines(1);

        // .threshold
        e.prefix_operator(".");
        e.ident("threshold");
        e.infix_operator("=");
        e.number("3");
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
        e.string("\"127.0.0.1\"");
        e.semicolon();
        e.newlines(1);

        // .port
        e.prefix_operator(".");
        e.ident("port");
        e.infix_operator("=");
        e.string("\"8080\"");
        e.semicolon();
        e.newlines(1);

        // .max_connections
        e.prefix_operator(".");
        e.ident("max_connections");
        e.infix_operator("=");
        e.number("100");
        e.semicolon();
        e.newlines(1);

        // .probe
        e.prefix_operator(".");
        e.ident("probe");
        e.infix_operator("=");
        e.ident("my_probe");
        e.semicolon();
        e.newlines(1);

        // .connect_timeout
        e.prefix_operator(".");
        e.ident("connect_timeout");
        e.infix_operator("=");
        e.number("5s");
        e.semicolon();
        e.newlines(1);

        // .first_byte_timeout
        e.prefix_operator(".");
        e.ident("first_byte_timeout");
        e.infix_operator("=");
        e.number("90s");
        e.semicolon();
        e.newlines(1);

        // .between_bytes_timeout
        e.prefix_operator(".");
        e.ident("between_bytes_timeout");
        e.infix_operator("=");
        e.number("2s");
        e.semicolon();
        e.newlines(1);

        // .asdf
        e.prefix_operator(".");
        e.ident("asdf");
        e.infix_operator("=");
        e.prefix_operator("!");
        e.number("true");
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
        e.infix_operator("/");
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
        e.ident("regsub");
        e.l_paren();
        e.ident("req.http.Host");
        e.comma();
        e.string("\":[0-9]+\"");
        e.comma();
        e.string("\"\"");
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
        e.ident("std.querysort");
        e.l_paren();
        e.ident("req.url");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.url");
        e.infix_operator("=");
        e.ident("regsub");
        e.l_paren();
        e.ident("req.url");
        e.comma();
        e.string("\"\\?$\"");
        e.comma();
        e.string("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Surrogate-Capability");
        e.infix_operator("=");
        e.string("\"key=ESI/1.0\"");
        e.semicolon();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.ident("std.healthy");
        e.l_paren();
        e.ident("req.backend_hint");
        e.r_paren();
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.grace");
        e.infix_operator("=");
        e.number("10s");
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.prefix_operator("!");
        e.ident("req.http.X-Forwarded-Proto");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.ident("std.port");
        e.l_paren();
        e.ident("server.ip");
        e.r_paren();
        e.infix_operator("==");
        e.number("443");
        e.infix_operator("||");
        e.ident("std.port");
        e.l_paren();
        e.ident("server.ip");
        e.r_paren();
        e.infix_operator("==");
        e.number("8443");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.X-Forwarded-Proto");
        e.infix_operator("=");
        e.string("\"https\"");
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
        e.string("\"https\"");
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.ident("req.http.Upgrade");
        e.infix_operator("~");
        e.string("\"(?i)websocket\"");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // return statement
        e.return_keyword();
        e.l_paren();
        e.ident("pipe");
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.ident("req.url");
        e.infix_operator("~");
        e.string("\"(\\?|&)(utm_source|utm_medium|utm_campaign|utm_content|gclid|cx|ie|cof|siteurl)=\"");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.url");
        e.infix_operator("=");
        e.ident("regsuball");
        e.l_paren();
        e.ident("req.url");
        e.comma();
        e.string("\"&(utm_source|utm_medium|utm_campaign|utm_content|gclid|cx|ie|cof|siteurl)=([A-z0-9_\\-\\.%25]+)\"");
        e.comma();
        e.string("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.url");
        e.infix_operator("=");
        e.ident("regsuball");
        e.l_paren();
        e.ident("req.url");
        e.comma();
        e.string("\"\\?(utm_source|utm_medium|utm_campaign|utm_content|gclid|cx|ie|cof|siteurl)=([A-z0-9_\\-\\.%25]+)\"");
        e.comma();
        e.string("\"?\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.url");
        e.infix_operator("=");
        e.ident("regsub");
        e.l_paren();
        e.ident("req.url");
        e.comma();
        e.string("\"\\?&\"");
        e.comma();
        e.string("\"?\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.url");
        e.infix_operator("=");
        e.ident("regsub");
        e.l_paren();
        e.ident("req.url");
        e.comma();
        e.string("\"\\?$\"");
        e.comma();
        e.string("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.ident("req.method");
        e.infix_operator("==");
        e.string("\"PURGE\"");
        e.r_paren();
        e.body_start();
        e.comment("// test");
        e.newlines(1);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.prefix_operator("!");
        e.ident("client.ip");
        e.infix_operator("~");
        e.ident("purge");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // return statement
        e.return_keyword();
        e.l_paren();
        e.ident("synth");
        e.l_paren();
        e.number("405");
        e.comma();
        e.ident("client.ip");
        e.infix_operator("+");
        e.string("\" is not allowed to send PURGE requests.\"");
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
        e.ident("purge");
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
        e.ident("req.method");
        e.infix_operator("!=");
        e.string("\"GET\"");
        e.infix_operator("&&");
        e.ident("req.method");
        e.infix_operator("!=");
        e.string("\"HEAD\"");
        e.infix_operator("&&");
        e.ident("req.method");
        e.infix_operator("!=");
        e.string("\"PUT\"");
        e.infix_operator("&&");
        e.ident("req.method");
        e.infix_operator("!=");
        e.string("\"POST\"");
        e.infix_operator("&&");
        e.ident("req.method");
        e.infix_operator("!=");
        e.string("\"TRACE\"");
        e.infix_operator("&&");
        e.ident("req.method");
        e.infix_operator("!=");
        e.string("\"OPTIONS\"");
        e.infix_operator("&&");
        e.ident("req.method");
        e.infix_operator("!=");
        e.string("\"PATCH\"");
        e.infix_operator("&&");
        e.ident("req.method");
        e.infix_operator("!=");
        e.string("\"DELETE\"");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // return statement
        e.return_keyword();
        e.l_paren();
        e.ident("pipe");
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.ident("req.method");
        e.infix_operator("!=");
        e.string("\"GET\"");
        e.infix_operator("&&");
        e.ident("req.method");
        e.infix_operator("!=");
        e.string("\"HEAD\"");
        e.r_paren();
        e.body_start();
        e.newlines(1);

        // return statement
        e.return_keyword();
        e.l_paren();
        e.ident("pass");
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.ident("req.url");
        e.infix_operator("~");
        e.string("\"^[^?]*\\.(7z|avi|bmp|bz2|css|csv|doc|docx|eot|flac|flv|gif|gz|ico|jpeg|jpg|js|less|mka|mkv|mov|mp3|mp4|mpeg|mpg|odt|ogg|ogm|opus|otf|pdf|png|ppt|pptx|rar|rtf|svg|svgz|swf|tar|tbz|tgz|ttf|txt|txz|wav|webm|webp|woff|woff2|xls|xlsx|xml|xz|zip)(\\?.*)?$\"");
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
        e.ident("hash");
        e.r_paren();
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);

        // set statement
        e.set_keyword();
        e.ident("req.http.Cookie");
        e.infix_operator("=");
        e.ident("regsuball");
        e.l_paren();
        e.ident("req.http.Cookie");
        e.comma();
        e.string("\"(__utm|_ga|_opt)[a-z_]*=[^;]+(; )?\"");
        e.comma();
        e.string("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Cookie");
        e.infix_operator("=");
        e.ident("regsuball");
        e.l_paren();
        e.ident("req.http.Cookie");
        e.comma();
        e.string("\"(__)?hs[a-z_\\-]+=[^;]+(; )?\"");
        e.comma();
        e.string("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Cookie");
        e.infix_operator("=");
        e.ident("regsuball");
        e.l_paren();
        e.ident("req.http.Cookie");
        e.comma();
        e.string("\"hubspotutk=[^;]+(; )?\"");
        e.comma();
        e.string("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Cookie");
        e.infix_operator("=");
        e.ident("regsuball");
        e.l_paren();
        e.ident("req.http.Cookie");
        e.comma();
        e.string("\"_hj[a-zA-Z]+=[^;]+(; )?\"");
        e.comma();
        e.string("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Cookie");
        e.infix_operator("=");
        e.ident("regsuball");
        e.l_paren();
        e.ident("req.http.Cookie");
        e.comma();
        e.string("\"(NID|DSID|__gads|GED_PLAYLIST_ACTIVITY|ACLK_DATA|ANID|AID|IDE|TAID|_gcl_[a-z]*|FLC|RUL|PAIDCONTENT|1P_JAR|Conversion|VISITOR_INFO1[a-z_]*)=[^;]+(; )?\"");
        e.comma();
        e.string("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(1);

        // set statement
        e.set_keyword();
        e.ident("req.http.Cookie");
        e.infix_operator("=");
        e.ident("regsuball");
        e.l_paren();
        e.ident("req.http.Cookie");
        e.comma();
        e.string("\"^;\\s*\"");
        e.comma();
        e.string("\"\"");
        e.r_paren();
        e.semicolon();
        e.newlines(2);

        // if statement
        e.if_keyword();
        e.l_paren();
        e.ident("req.http.cookie");
        e.infix_operator("~");
        e.string("\"^\\s*$\"");
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
        e.ident("hash_data");
        e.l_paren();
        e.ident("req.http.X-Forwarded-Proto");
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
        e.ident("bereq.url");
        e.infix_operator("~");
        e.string("\"^[^?]*\\.(7z|avi|bmp|bz2|css|csv|doc|docx|eot|flac|flv|gif|gz|ico|jpeg|jpg|js|less|mka|mkv|mov|mp3|mp4|mpeg|mpg|odt|ogg|ogm|opus|otf|pdf|png|ppt|pptx|rar|rtf|svg|svgz|swf|tar|tbz|tgz|ttf|txt|txz|wav|webm|webp|woff|woff2|xls|xlsx|xml|xz|zip)(\\?.*)?$\"");
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
        e.number("1d");
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);
        e.if_keyword();
        e.l_paren();
        e.ident("beresp.http.Surrogate-Control");
        e.infix_operator("~");
        e.string("\"ESI/1.0\"");
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
        e.number("true");
        e.semicolon();
        e.newlines(1);
        e.body_end();
        e.newlines(2);
        e.set_keyword();
        e.ident("beresp.grace");
        e.infix_operator("=");
        e.number("6h");
        e.semicolon();
        e.newlines(1);
        e.body_end();

        e.file_end();

        // assert_eq!(writer.buffer(), crate::EXAMPLE);
    }
}
