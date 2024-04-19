use std::fmt::{Arguments, Write};

use linked_hash_map::LinkedHashMap;

use crate::Value;

macro_rules! match_variant {
    ($ty:tt => $e:expr => {
        $var:ident : $(|)? $($variant:ident)|+ => $do:expr
    }) => {
        match $e {
            $(
                $ty::$variant($var) => $do,
            )+
        }
    };
}

fn do_indent<F: FnMut(Arguments<'_>)>(f: &mut F, indent: usize) {
    for _ in 0..indent {
        f(format_args!("  "));
    }
}

pub trait SYMLSerialize {
    /// Serialize to a shorter form
    ///
    /// # Examples
    /// ```
    /// # use syml::{Value, parser, SYMLSerialize};
    /// # use core::fmt::Write;
    /// let value = Value::from([
    ///     "2".into(),
    ///     Value::from(["3", "4", "5"]),
    ///     [("x", "6")].into()
    /// ]);
    /// let mut buf = String::new();
    /// value.serialize_min(&mut |args| write!(buf, "{args}").unwrap());
    /// assert_eq!(&buf, "[2,[3,4,5],{x:6}]");
    /// ```
    fn serialize_min<F: FnMut(Arguments<'_>)>(&self, f: &mut F);
    /// Serialize to standard form
    ///
    /// # Examples
    /// ```
    /// # use syml::{Value, parser, SYMLSerialize};
    /// # use core::fmt::Write;
    /// let value = Value::from([
    ///     "2".into(),
    ///     Value::from(["3", "4", "5"]),
    ///     [("x", "6")].into()
    /// ]);
    /// let mut buf = String::new();
    /// value.serialize(&mut |args| write!(buf, "{args}").unwrap(), 0);
    /// buf.push('\n');
    /// assert_eq!(&buf, "\
    /// - 2
    /// - - 3
    ///   - 4
    ///   - 5
    /// - x: 6
    /// ");
    /// ```
    fn serialize<F: FnMut(Arguments<'_>)>(&self, f: &mut F, indent: usize) {
        let _ = indent;
        self.serialize_min(f)
    }
    /// Same as serialize, but collect to string
    ///
    /// # Examples
    /// ```
    /// # use syml::{Value, parser, SYMLSerialize};
    /// # use core::fmt::Write;
    /// let value = Value::from([
    ///     "2".into(),
    ///     Value::from(["3", "4", "5"]),
    ///     [("x", "6")].into()
    /// ]);
    /// let mut buf = String::new();
    /// value.serialize(&mut |args| write!(buf, "{args}").unwrap(), 0);
    /// assert_eq!(buf, value.serialize_to_string(0));
    /// ```
    fn serialize_to_string(&self, indent: usize) -> String {
        let mut buf = String::new();
        self.serialize(
            &mut |args| write!(buf, "{args}").unwrap(),
            indent,
        );
        buf
    }
    /// Same as serialize_min, but collect to string
    ///
    /// # Examples
    /// ```
    /// # use syml::{Value, parser, SYMLSerialize};
    /// # use core::fmt::Write;
    /// let value = Value::from([
    ///     "2".into(),
    ///     Value::from(["3", "4", "5"]),
    ///     [("x", "6")].into()
    /// ]);
    /// let mut buf = String::new();
    /// value.serialize_min(&mut |args| write!(buf, "{args}").unwrap());
    /// assert_eq!(buf, value.serialize_min_to_string());
    fn serialize_min_to_string(&self) -> String {
        let mut buf = String::new();
        self.serialize_min(
            &mut |args| write!(buf, "{args}").unwrap(),
        );
        buf
    }
}
impl SYMLSerialize for Value {
    fn serialize<F: FnMut(Arguments<'_>)>(&self, f: &mut F, indent: usize) {
        match_variant!(Value => self => {
            v: String | Array | Table => v.serialize(f, indent)
        })
    }
    fn serialize_min<F: FnMut(Arguments<'_>)>(&self, f: &mut F) {
        match_variant!(Value => self => {
            v: String | Array | Table => v.serialize_min(f)
        })
    }
}
impl SYMLSerialize for String {
    fn serialize_min<F: FnMut(Arguments<'_>)>(&self, f: &mut F) {
        if self.is_empty() {
            return f(format_args!("''"));
        }
        if crate::parser::simple_literal(self).is_ok() {
            return f(format_args!("{self}"));
        }
        if self.chars()
            .all(|ch| ch != '\''
                && (ch == '"' || ch.escape_debug().size_hint().0 == 1))
        {
            return f(format_args!("'{self}'"));
        }
        f(format_args!("\""));
        for ch in self.chars() {
            if ch == '\'' {
                f(format_args!("'"));
                continue;
            }
            let mut esc = ch.escape_debug();
            match esc.size_hint().0 {
                1 => f(format_args!("{ch}")),
                2 => f(format_args!("\\{}", esc.nth(1).unwrap())),
                _ if u8::try_from(ch).is_ok() => {
                    f(format_args!("\\x{:02x}", ch as u8))
                },
                _ if u16::try_from(ch).is_ok() => {
                    f(format_args!("\\u{:04x}", ch as u16))
                },
                _  => f(format_args!("\\u{{{:01x}}}", ch as u32)),
            }
        }
        f(format_args!("\""));
    }
}
impl SYMLSerialize for [Value] {
    fn serialize_min<F: FnMut(Arguments<'_>)>(&self, f: &mut F) {
        f(format_args!("["));
        match self {
            [] => (),
            [head, next @ ..] => {
                head.serialize_min(f);
                for val in next {
                    f(format_args!(","));
                    val.serialize_min(f)
                }
            },
        }
        f(format_args!("]"));
    }
    fn serialize<F: FnMut(Arguments<'_>)>(&self, f: &mut F, indent: usize) {
        if self.is_empty() { return self.serialize_min(f); }
        f(format_args!("- "));
        self[0].serialize(f, indent+1);
        for val in &self[1..] {
            f(format_args!("\n"));
            do_indent(f, indent);
            f(format_args!("- "));
            val.serialize(f, indent+1);
        }
    }
}
impl SYMLSerialize for LinkedHashMap<String, Value> {
    fn serialize_min<F: FnMut(Arguments<'_>)>(&self, f: &mut F) {
        f(format_args!("{{"));
        self.iter().fold(true, |head, (k, v)| {
            if !head { f(format_args!(",")); }
            k.serialize_min(f);
            f(format_args!(":"));
            v.serialize_min(f);
            false
        });
        f(format_args!("}}"));
    }
    fn serialize<F: FnMut(Arguments<'_>)>(&self, f: &mut F, indent: usize) {
        if self.is_empty() { return self.serialize_min(f); }
        self.iter().fold(true, |head, (k, v)| {
            if !head {
                f(format_args!("\n"));
                do_indent(f, indent);
            }
            k.serialize_min(f);
            f(format_args!(":"));
            if v.is_string() || v.is_empty() {
                f(format_args!(" "));
                v.serialize_min(f);
            } else {
                let inc = if !v.is_array() { 1 } else { 0 };
                f(format_args!("\n"));
                do_indent(f, indent+inc);
                v.serialize(f, indent+inc);
            }
            false
        });
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use crate::parser;
    use super::*;

    #[test]
    fn serialize_min_test() {
        let tests = [
            ("{}", "{}"),
            ("[]", "[]"),
            ("''", "''"),
            ("a", "a"),
            ("a'b", "a'b"),
            ("'a b'", "'a b'"),
            ("'a\x1bb'", r#""a\x1bb""#),
            ("'a \u{5fc3}'", r#"'a 心'"#),
            ("'a\u{10ffff}'", r#""a\u{10ffff}""#),
            ("{a:1,b:2,c:[3,4]}", "{a:1,b:2,c:[3,4]}"),
            ("{名字:小明}", "{名字:小明}"),
        ];
        for (src, dst) in tests {
            let val = parser::value(src).unwrap();
            let mut s = String::new();
            val.serialize_min(
                &mut |args| write!(s, "{args}").unwrap());
            assert_eq!(s, dst);
        }
    }

    #[test]
    fn serialize_test() {
        let tests = [
            ("{a:1,b:[1,2]}", "a: 1\nb:\n- 1\n- 2"),
            ("{a:1,b:{x:1,y:2}}", "a: 1\nb:\n  x: 1\n  y: 2"),
            ("{l:[[1,2],[3,4]]}", "l:\n- - 1\n  - 2\n- - 3\n  - 4"),
            ("{l:[[1,[2, 3]],[3,[4,5]]]}", "l:\n- - 1\n  - - 2\n    - 3\n- - 3\n  - - 4\n    - 5"),
            ("{a:1,b:{x:1,y:{n:2,i:3}},c:3}", "a: 1\nb:\n  x: 1\n  y:\n    n: 2\n    i: 3\nc: 3"),
            ("{}", "{}"),
            ("[]", "[]"),
            ("234", "234"),
            ("{a:[],b:{},c:''}", "a: []\nb: {}\nc: ''"),
            ("[[],{},'']", "- []\n- {}\n- ''"),
            ("[[],{a:1},'']", "- []\n- a: 1\n- ''"),
            ("[[],{a:1,b:2},'']", "- []\n- a: 1\n  b: 2\n- ''"),
        ];
        for (src, dst) in tests {
            let val = parser::value(src).unwrap();
            let mut s = String::new();
            val.serialize(
                &mut |args| write!(s, "{args}").unwrap(),
                0
            );
            assert_eq!(s, dst);
        }
    }

    #[test]
    fn serialize_str_escape_test() {
        let tests = [
            (r#""\n""#, r#""\n""#),
            (r#""\x01""#, r#""\x01""#),
            (r#""\x1b""#, r#""\x1b""#),
            (r#""\u001b""#, r#""\x1b""#),
            (r#""\u0378""#, r#""\u0378""#),
            (r#""\u{10ffff}""#, r#""\u{10ffff}""#),
        ];
        for (src, dst) in tests {
            let val = parser::value(src).unwrap();
            let mut s = String::new();
            val.serialize_min(
                &mut |args| write!(s, "{args}").unwrap());
            assert_eq!(s, dst);
        }
    }
}
