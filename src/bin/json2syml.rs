use serde_json as json;
use std::{
    fmt,
    io::{stdout, Write},
    process::exit,
};
use syml::{cli_utils::read_input, SYMLSerialize};

const HELP: &str = "\
USAGE: json2syml [<FILE | -h | --help> [%]]\n\
convert JSON to SYML\n\
\n\
FILE: source file\n\
\x20   no given this arg or value is `-`, it from stdin\n\
\n\
%:\n\
\x20   is long output\n\
";

fn to_syml(val: json::Value) -> syml::Value {
    use json::Value as JV;
    use syml::Value as SV;
    match val {
        JV::Null => SV::String("null".into()),
        JV::Bool(bool) => SV::String(bool.to_string()),
        JV::Number(num) => SV::String(num.to_string()),
        JV::String(s) => s.into(),
        JV::Array(arr) => {
            SV::Array(arr.into_iter().map(to_syml).collect())
        },
        JV::Object(obj) => {
            SV::Table(obj.into_iter()
                .map(|(k, v)| (k, to_syml(v)))
                .collect())
        },
    }
}

fn main() {
    let input = match read_input(HELP) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Read input error: {e}");
            exit(e.raw_os_error().unwrap_or(2))
        },
    };
    let val = match json::from_str(&input.src) {
        Ok(val) => val,
        Err(e) => {
            let (line, column) = (e.line(), e.column());
            eprintln!("ParseError: in line {line} col {column}");
            eprintln!("  err: {}", e);
            exit(3);
        },
    };
    let syml_val = to_syml(val);
    let mut output = stdout();
    fn conv<F: FnMut(fmt::Arguments<'_>)>(f: F) -> F {
        f
    }
    let mut out = conv(|args| {
        output.write_fmt(args).unwrap();
    });
    if input.is_long_output {
        syml_val.serialize(&mut out, 0);
    } else {
        syml_val.serialize_min(&mut out);
    }
    println!();
}
