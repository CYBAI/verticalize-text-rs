use app;

#[derive(Debug)]
pub struct Args {
  pub separator: String,
  pub line_direction: String,
  pub word_direction: String,
  pub no_rotate: bool,
  pub filepath: Option<String>
}

impl Args {
  pub fn parse() -> Result<Args, &'static str> {
    let matches = app::app().get_matches();

    let filepath = match matches.value_of("FILEPATH") {
        Some(path) => Some(path.to_string()),
        None => None,
    };

    Ok(Args {
      separator: matches.value_of("SEPARATOR").unwrap().to_string(),
      line_direction: matches.value_of("LINE_DIRECTION").unwrap().to_string(),
      word_direction: matches.value_of("WORD_DIRECTION").unwrap().to_string(),
      no_rotate: matches.is_present("NO_ROTATE"),
      filepath,
    })
  }
}

