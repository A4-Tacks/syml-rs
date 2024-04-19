use std::{
    fs::{self, read_dir},
    io::{self, stderr, IsTerminal},
    path::PathBuf,
};
use syml::parser;

fn to_json(value: syml::Value) -> json::JsonValue {
    use json::JsonValue as JV;
    use syml::Value as SV;
    match value {
        SV::String(s) => JV::String(s),
        SV::Array(arr) => JV::Array(arr.into_iter().map(to_json).collect()),
        SV::Table(map) => {
            JV::Object(map.into_iter()
                .map(|(k, v)| (k, to_json(v)))
                .collect())
        },
    }
}

fn color(code: u8) {
    if stderr().is_terminal() {
        eprint!("\x1b[{code}m");
    }
}

fn main() -> io::Result<()> {
    let dir = PathBuf::from_iter(["tests", "parse_datas"]);
    let files = read_dir(dir)?
        .map(|enter| enter.and_then(|enter| {
            assert!(enter.file_type()?.is_file());
            Ok(enter.path())
        }))
        .filter(|path| path.as_ref().is_ok_and(|path| {
            path.file_name()
                .map(|name| name.to_str())
                .flatten()
                .is_some_and(|name| name.ends_with(".syml"))
        }))
        .collect::<Result<Vec<_>, _>>()?;

    for file in files {
        color(0);
        eprint!("test {} ...", file.to_string_lossy());
        let dst_file = {
            let mut file = file.clone();
            assert!(file.set_extension("json"), "{:?}", file);
            file
        };
        let src = fs::read_to_string(file)?;
        let json_src = fs::read_to_string(dst_file)?;

        match parser::value(&src) {
            Ok(value) => {
                let value = to_json(value);
                let json_obj = json::parse(&json_src).unwrap();
                if value == json_obj {
                    color(92);
                    eprintln!(" ok");
                    continue;
                }
                color(91);
                eprintln!(" fail");
                color(0);
                eprintln!("syml: {}", value.to_string());
                eprintln!("json: {}", json_obj.to_string());
            },
            Err(e) => {
                color(91);
                eprintln!(" [parse failed]");
                color(0);
                eprintln!("{e:#?}");
            },
        };
    }

    Ok(())
}
