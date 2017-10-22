extern crate clap;

mod app;
mod args;
mod matrix;

use args::Args;
use matrix::Matrix;

use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;

fn main() {
  match Args::parse() {
    Ok(args) => run(args),
    Err(_) => println!("SOMETHING WENT WRONG!")
  }
}

fn run(args: Args) {
  if args.filepath.is_none() {
    repl()
  }

  let f = File::open(&args.filepath.as_ref().unwrap()).expect("Failed to read file");
  let reader = BufReader::new(f);

  let contents: Vec<Vec<String>> = reader.lines().map(|line| {
    match line {
      Ok(l) => l.split("").map(String::from).collect(),
      Err(e) => panic!("{}", e),
    }
  }).collect();

  Matrix::new(contents)
    .fill_blank()
    .transpose()
    .print()
}

fn repl() {
  println!("Hello world");
}
