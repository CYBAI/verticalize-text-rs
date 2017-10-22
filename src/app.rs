use clap::{App, Arg};

const ABOUT: &'static str = "
  verticalize

  This is inspired by hor2vec and tate.
";

const USAGE: &'static str = "
  verticalize [-h] [-s SEP] [--ld {l2r,r2l}] [--wd {t2b,b2t}] [--nr] <FILEPATH>
";

pub fn app() -> App<'static, 'static> {
  App::new("Change horizontal texts to vertical")
          .version("0.1.0")
          .author("cybai <cyb.ai.815@gmail.com>")
          .about(ABOUT)
          .usage(USAGE)
          .help_message("Prints help information. Use --help for more details.")
          .arg(Arg::with_name("SEPARATOR")
               .short("s")
               .long("seprator")
               .alias("sep")
               .help("The seperator between lines. Default is '', you can use ' ', '|' or any other strings.")
               .default_value(""))
          .arg(Arg::with_name("LINE_DIRECTION")
               .long("line-direction")
               .alias("ld")
               .possible_values(&["l2r", "r2l"])
               .help("The reading direction of each line. Default is 'l2r' (left to right), you can choose 'r2l' (right to left).")
               .default_value("l2r"))
          .arg(Arg::with_name("WORD_DIRECTION")
               .long("word-direction")
               .alias("wd")
               .possible_values(&["t2b", "b2t"])
               .help("The reading direction of each word/character. Default is 't2b' (top to bottom), you can choose 'b2t' (bottom to top).")
               .default_value("t2b"))
          .arg(Arg::with_name("NO_ROTATE")
               .long("no-rotate")
               .alias("nr")
               .help("With this option given, verticalize won't rotate the input"))
          .arg(Arg::with_name("FILEPATH")
               .help("The file you'd like to change the words into vertical")
               .index(1)
               .takes_value(true))
}

