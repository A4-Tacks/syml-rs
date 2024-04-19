use syml::cli_utils::read_input;
use peg::str::LineCol;
use std::{process::exit, io::stdout};

const HELP: &str = "\
USAGE: syml2json [<FILE | -h | --help> [%]]\n\
convert SYML to JSON\n\
\n\
FILE: source file\n\
\x20   no given this arg or value is `-`, it from stdin\n\
\n\
%:\n\
\x20   is long output\n\
";

fn to_json(val: syml::Value) -> json::JsonValue {
    use json::JsonValue as JV;
    use syml::Value as SV;
    match val {
        SV::String(s) => JV::String(s),
        SV::Array(arr) => {
            JV::Array(arr.into_iter()
                .map(to_json)
                .collect())
        },
        SV::Table(table) => {
            JV::Object(table.into_iter()
                .map(|(k, v)| (k, to_json(v)))
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
    let val = match syml::parser::value(&input.src) {
        Ok(val) => val,
        Err(e) => {
            let LineCol { line, column, .. } = e.location;
            eprintln!("ParseError: in line {line} col {column}");
            eprintln!("  expected: {}", e.expected);
            exit(3);
        },
    };
    let json_val = to_json(val);
    let mut out = stdout();
    if input.is_long_output {
        json_val.write_pretty(&mut out, 4).unwrap();
    } else {
        json_val.write(&mut out).unwrap();
    }
}
