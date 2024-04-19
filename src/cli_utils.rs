use std::{
    env::args_os,
    fs::File,
    io::{self, stderr, stdin, Read, Write},
    process::exit,
};

#[non_exhaustive]
pub struct Config {
    pub src: String,
    pub is_long_output: bool,
}

pub fn read_input(help: &str) -> io::Result<Config> {
    let mut args = args_os().skip(1);
    let mut input = String::new();
    match args.next().as_deref() {
        Some(arg) if arg == "-" => {
            stdin().read_to_string(&mut input)?;
        },
        None => {
            stdin().read_to_string(&mut input)?;
        },
        Some(arg) if arg == "-h" || arg == "--help" => {
            println!("utils from syml@{}", env!("CARGO_PKG_VERSION"));
            println!("repo: {}", env!("CARGO_PKG_REPOSITORY"));
            println!("{help}");
            exit(0);
        },
        Some(path) => {
            File::open(path)?.read_to_string(&mut input)?;
        },
    };
    let mut config = Config { src: input, is_long_output: false };
    loop {
        match args.next() {
            Some(arg) if arg == "%" => config.is_long_output = true,
            Some(arg) => {
                eprintln!("Error: Extra arg: ");
                stderr().write_all(arg.as_encoded_bytes())
                    .unwrap_or_else(|e| {
                        panic!("display extra arg failed: {e:?}")
                    });
                exit(2);
            },
            None => break,
        }
    }
    Ok(config)
}
