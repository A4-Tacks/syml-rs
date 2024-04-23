use std::{
    env::args_os,
    fs::File,
    io::{self, stderr, stdin, Read, Stdin, Write},
    process::exit,
};

macro_rules! impl_enum_froms {
    (impl From for $enum:ty {
        $($variant:ident : $ty:ty),*
        $(,)?
    }) => {
        $(
            impl From<$ty> for $enum {
                fn from(value: $ty) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    };
}

enum Reader {
    Stdin(Stdin),
    File(File),
}
impl Reader {
    pub fn read_to_string(
        &mut self,
        s: &mut String,
    ) -> io::Result<usize> {
        match self {
            Reader::Stdin(stdin) => stdin.read_to_string(s),
            Reader::File(file) => file.read_to_string(s),
        }
    }
}
impl_enum_froms!(impl From for Reader {
    Stdin: Stdin,
    File: File,
});

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct Config {
    pub src: String,
    pub is_long_output: bool,
    pub convert_number: bool,
    pub convert_boolean: bool,
    pub convert_null: bool,
}

pub fn read_input(help: &str) -> io::Result<Config> {
    let mut config = Config::default();
    let mut args = args_os().skip(1);
    let mut reader: Reader = match args.next().as_deref() {
        Some(arg) if arg == "-" => {
            stdin().into()
        },
        None => {
            stdin().into()
        },
        Some(arg) if arg == "-h" || arg == "--help" => {
            println!("utils from syml@{}", env!("CARGO_PKG_VERSION"));
            println!("repo: {}", env!("CARGO_PKG_REPOSITORY"));
            println!("{help}");
            exit(0);
        },
        Some(path) => {
            File::open(path)?.into()
        },
    };
    loop {
        match args.next() {
            Some(arg) if arg == "%" => config.is_long_output = true,
            Some(arg) if arg == "-n" => config.convert_number = true,
            Some(arg) if arg == "-b" => config.convert_boolean = true,
            Some(arg) if arg == "-N" => config.convert_null = true,
            Some(arg) if arg == "-w" => {
                [
                    config.convert_null,
                    config.convert_number,
                    config.convert_boolean,
                ] = [true; 3];
            },
            Some(arg) => {
                let mut stderr = stderr().lock();
                || -> Result<(), io::Error> {
                    stderr.write(b"Error: Extra arg: ")?;
                    stderr.write_all(arg.as_encoded_bytes())?;
                    writeln!(stderr)?;
                    Ok(())
                }().unwrap_or_else(|e| {
                    panic!("display extra arg failed: {e:?}")
                });
                exit(2);
            },
            None => break,
        }
    }
    reader.read_to_string(&mut config.src)?;
    Ok(config)
}
