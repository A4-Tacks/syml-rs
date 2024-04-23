pub use parser::*;
use peg::RuleResult;

use crate::Value;

trait StrExt {
    fn indent(&self, pos: usize, n: usize) -> RuleResult<()>;
    fn ident_start(&self, pos: usize) -> RuleResult<()>;
    fn ident_continue(&self, pos: usize) -> RuleResult<()>;
}
impl StrExt for str {
    fn indent(&self, pos: usize, n: usize) -> RuleResult<()> {
        let s = &self[pos..];
        for i in 0..n {
            if !s[i..].starts_with(' ') {
                return RuleResult::Failed;
            }
        }
        RuleResult::Matched(pos+n, ())
    }
    fn ident_start(&self, pos: usize) -> RuleResult<()> {
        let Some(ch) = self[pos..].chars().next() else {
            return RuleResult::Failed;
        };
        if unicode_ident::is_xid_start(ch) {
            RuleResult::Matched(pos+ch.len_utf8(), ())
        } else {
            RuleResult::Failed
        }
    }
    fn ident_continue(&self, pos: usize) -> RuleResult<()> {
        let Some(ch) = self[pos..].chars().next() else {
            return RuleResult::Failed;
        };
        if unicode_ident::is_xid_continue(ch) {
            RuleResult::Matched(pos+ch.len_utf8(), ())
        } else {
            RuleResult::Failed
        }
    }
}

peg::parser!(grammar parser() for str {
    pub(crate) rule simple_literal_start()
        =   [ '!' | '#' | '$' | '%' | '&' | '(' | ')' | '*' | '+'
            | '.' | '/' | '0'..='9' | '<' | '=' | '>' | '?' | '@'
            | 'A'..='Z' | '\\'| '^' | '_' | '`' | 'a'..='z' | '|'
            | '~' ] / "-" !" " / ##ident_start()

    pub(crate) rule simple_literal_continue()
        =   [ '!' | '#' | '$' | '%' | '&' | '(' | ')' | '*' | '+'
            | '.' | '/' | '0'..='9' | '<' | '=' | '>' | '?' | '@'
            | 'A'..='Z' | '\\'| '^' | '_' | '`' | 'a'..='z' | '|'
            | '~' | '-' | '\''] / ##ident_continue()

    pub(crate) rule simple_literal() -> &'input str
        = s:$(
            simple_literal_start()
            simple_literal_continue()*
        )


    rule _()
        = [' ' | '\t']*

    rule eof()
        = ![_]

    rule nl_noeof()
        = "\r"? "\n"

    rule nl()
        = nl_noeof()
        / eof()

    rule comment()
        = ";" (!nl() [_])*

    rule cnl()
        = (_ comment()? nl_noeof())+
        / eof()

    rule close_args<T, S>(v: rule<T>, sep: rule<S>) -> Vec<T>
        = elems:(elem:v() ++ sep() { elem }) sep()? { elems }

    pub(super) rule indent(n: usize)
        = ##indent(n)


    pub(crate) rule literal_string_body()
        = (!nl() [^ '\''])*

    rule literal_string() -> &'input str
        = "'" s:$(literal_string_body()) "'" { s }

    rule hex()
        = ['0'..='9' | 'a'..='f' | 'A'..='F']

    rule string_escaped() -> char
        = "\\" ch:(
            ch:['\\' | '\'' | '"' | ' ' | '\t'] { ch }
            / "n" { '\n' }
            / "r" { '\r' }
            / "t" { '\t' }
            / "x" s:$(hex()*<2>) { u8::from_str_radix(s, 16).unwrap().into() }
            / s:( "u" s:(s:$(hex()*<4>) { s } / "{" s:$(hex()*<1,8>) "}" { s }) { s }
                / "U" s:$(hex()*<8>) { s }
                ) {?
                    match char::from_u32(u32::from_str_radix(s,16).unwrap()) {
                        Some(ch) => Ok(ch),
                        None => Err("Valid Unicode char"),
                    }
                }
        ) { ch }

    rule string_ignore_empty()
        = "\\" comment()? nl_noeof() _

    rule string() -> String
        = "\"" string_ignore_empty()?
            s:(ch:string_escaped() string_ignore_empty()? { ch }
                / !nl() ch:[^ '\\' | '"'] string_ignore_empty()? { ch }
            )*
            "\""
        { s.into_iter().collect() }

    pub rule simple_val() -> String
        = s:simple_literal() { s.into() }
        / s:literal_string() { s.into() }
        / s:string() { s }


    rule inline_list() -> Vec<Value>
        = "[" _ vals:close_args(<inline_value()>, <_ "," _>)? _ "]"
        { vals.unwrap_or_default() }

    rule inline_table() -> Value
        = "{" _ vals:close_args(<
            k:simple_val() _ ":" _ v:inline_value() { (k, v) }
        >, <_ "," _>)? _ "}"
        { vals.unwrap_or_default().into() }

    pub rule inline_value() -> Value
        = v:inline_list()   { v.into() }
        / v:inline_table()  { v }
        / v:simple_val()    { v.into() }


    rule table_val(indent_level: usize) -> Value
        = cnl() v:(indent(indent_level) v:list(indent_level) { v }
            / indent(indent_level+2) v:ivalue_non_inline(indent_level+2) { v }
            ) { v }
        / _ v:inline_value() { v }

    rule table(indent_level: usize) -> Value
        = tab:(
            k:(k:simple_val() _ ":" { k }) ++ _ v:table_val(indent_level)
            {
                let mut k = k;
                let tail = k.pop().unwrap();
                k.into_iter()
                    .rfold((tail, v), |tab, key| {
                        (key, [tab].into())
                    })
            }
        ) ++ (cnl() indent(indent_level))
        { tab.into() }

    rule list(indent_level: usize) -> Value
        = v:("- " v:ivalue(indent_level+2) {v}) ++ (cnl() indent(indent_level))
        { v.into() }

    rule ivalue_non_inline(indent_level: usize) -> Value
        = v:(list(indent_level) / table(indent_level))
        { v }

    rule ivalue(indent_level: usize) -> Value
        = v:(ivalue_non_inline(indent_level) / inline_value())
        { v }


    /// parse to value node
    ///
    /// # Examples
    /// ```
    /// use syml::{Value, parser};
    ///
    /// let value = parser::value("- 1\n- {a:1,b:2}");
    /// let expect = ["1".into(), Value::from([("a", "1"),("b", "2")])];
    /// assert_eq!(value.unwrap(), expect.into());
    /// ```
    pub rule value() -> Value
        = cnl()? v:ivalue(0) cnl() { v }
});

#[cfg(test)]
mod tests {
    use crate::Value;
    use super::parser;

    macro_rules! map {
        ($($k:literal : $v:expr),* $(,)?) => {
            Value::Table(::linked_hash_map::LinkedHashMap::from_iter(
                [$((String::from($k), Value::from($v))),*]
            ))
        };
    }

    #[test]
    fn indent_test() {
        let tests = [
            ("", 0),
            (" ", 1),
            ("  ", 2),
            ("    ", 4),
        ];
        for (src, indent) in tests {
            parser::indent(src, indent).unwrap();
        }
    }

    #[test]
    fn literal_parse_test() {
        let tests = [
            (r#"abc"#, "abc"),
            (r#"a'bc"#, "a'bc"),
            (r#"'ab" c'"#, "ab\" c"),
            (r#""ab\r\ncd""#, "ab\r\ncd"),
            (r#""\x1b\x00\u001b\u0000""#, "\x1b\0\u{001b}\u{0}"),
            (r#""a\\\"b""#, "a\\\"b"),
            (r#""\u{04f60}""#, "你"),
            (r#"'你好'"#, "你好"),
            (r#"'你\n好'"#, "你\\n好"),
            (r#"'ab'"#, "ab"),
            (r#"ab'"#, "ab'"),
            (r#"foo-bar"#, "foo-bar"),
            ("\"abc\\\n    def\"", "abcdef"),
            ("\"abc\\\n   \\ def\"", "abc def"),
            ("\"abc\\\n \t  \\\tdef\"", "abc\tdef"),
            ("\"abc\\\n;intoken \t def\"", "abc;intoken \t def"),
            ("\"abc\\\n;intoken \t def\"", "abc;intoken \t def"),
        ];
        for (src, expect) in tests {
            assert_eq!(parser::simple_val(src), Ok(expect.into()));
        }
    }

    #[test]
    fn value_test() {
        let tests = [
            ("a", "a".into()),
            ("a\n", "a".into()),
            ("a\r\n", "a".into()),
            ("-a", "-a".into()),
            ("- a", ["a"].into()),
            ("- [a]", [Value::from(["a"])].into()),
            ("[]", [""; 0].into()),
            ("[a]", ["a"].into()),
            ("[a,]", ["a"].into()),
            ("[a,b]", ["a", "b"].into()),
            ("{}", Value::Table(Default::default())),
            ("{a:1}", map!("a": "1")),
            ("{a:1,}", map!("a": "1")),
            ("{a:1,b:2}", map!("a": "1","b": "2")),
            ("[{a:1,b:2}]", [map!("a": "1","b": "2")].into()),
        ];
        for (src, dst) in tests {
            assert_eq!(parser::value(src), Ok(dst));
        }
    }
}
