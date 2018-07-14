#![feature(slice_patterns)]

extern crate bfcrs;

use std::fs::{create_dir_all, read_to_string, OpenOptions};
use std::path::Path;

use bfcrs::backend::wasm::WasmBackend;
use bfcrs::backend::Backend;
use bfcrs::interpreter::run_program;
use bfcrs::interpreter::ConsoleIo;
use bfcrs::optimizer::optimize_parsed;
use bfcrs::parser::parse_program;
use bfcrs::types::State;

struct Config {
  compile: bool,
  run: bool,
  print_ir: bool,
  source_path: String,
  output_path: String,
}

impl Default for Config {
  fn default() -> Config {
    Config {
      compile: true,
      run: false,
      print_ir: false,
      source_path: "./bf/hello.bf".to_string(),
      output_path: "./bin/out.wasm".to_string(),
    }
  }
}

fn parse_args<'a>(args: Vec<String>) -> Config {
  fn parse_next(args: &[&str], config: Config) -> Config {
    match args {
      &["--build-and-run", ref rest..] => parse_next(
        rest,
        Config {
          compile: true,
          run: true,
          ..config
        },
      ),
      &["--only-run", ref rest..] => parse_next(
        rest,
        Config {
          compile: false,
          run: true,
          ..config
        },
      ),
      &["--print-ir", ref rest..] => parse_next(
        rest,
        Config {
          print_ir: true,
          ..config
        },
      ),
      &["--out", file_name, ref rest..] => parse_next(
        rest,
        Config {
          output_path: file_name.to_string(),
          ..config
        },
      ),
      &[source_path] => Config {
        source_path: source_path.to_string(),
        ..config
      },
      &[x, _..] => {
        panic!("Unknown parameter: {}", x);
      }
      &[] => config,
    }
  }

  let borrowed = args.iter().map(|x| &x[..]).collect::<Vec<_>>();
  parse_next(&borrowed[1..], Config::default())
}

fn main() {
  let config = parse_args(std::env::args().collect());

  println!("Reading {}...", config.source_path);

  let src = read_to_string(config.source_path).expect("Source file should exist.");

  println!("Parsing...");

  let parsed_program = parse_program(&src);

  println!("Optimizing...");

  let optimized_program = optimize_parsed(&parsed_program);

  if config.print_ir {
    println!("IR: {:?}", &optimized_program);
  }

  if config.compile {
    create_dir_all(Path::new(&config.output_path).parent().unwrap()).unwrap();

    let output_path = Path::new("./bin").join("out.wasm");

    let mut output_file = OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(output_path)
      .unwrap();

    let wasm_backend = WasmBackend;
    wasm_backend.compile_to_stream(&optimized_program, &mut output_file);
  }

  if config.run {
    run_program(&optimized_program, &mut State::new(), &mut ConsoleIo);
  }
}
