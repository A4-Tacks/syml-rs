use syml::cli_utils::read_input;
use peg::str::LineCol;
use serde_json as json;
use std::process::exit;

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

fn to_json(val: syml::Value) -> json::Value {
    match val {
        syml::Value::String(s) => json::Value::String(s),
        syml::Value::Array(arr) => {
            json::Value::Array(arr.into_iter()
                .map(to_json)
                .collect())
        },
        syml::Value::Table(table) => {
            json::Value::Object(table.into_iter()
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
    if input.is_long_output {
        println!("{json_val:#}");
    } else {
        println!("{json_val}");
    }
}
