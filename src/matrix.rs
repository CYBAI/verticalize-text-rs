use std::ascii::AsciiExt;

static HALFWIDTH_SPACE: &'static str = " ";
static FULLWIDTH_SPACE: &'static str = "　";

#[derive(Debug)]
pub struct Matrix {
  contents: Vec<Vec<String>>
}

impl Matrix {
  pub fn new(contents: Vec<Vec<String>>) -> Matrix {
    Matrix { contents }
  }

  pub fn fill_blank(&self) -> Matrix {
    let max_len = self.max_len();

    let filled_matrix = self.contents.iter().map(|v| {
      let mut cloned_v = v.clone();
      cloned_v.remove(0);
      cloned_v.pop();

      let need_to_fill = max_len - cloned_v.len();

      let empty_string_vec = match self.is_ascii(v) {
          true => HALFWIDTH_SPACE.to_string(),
          false => FULLWIDTH_SPACE.to_string(),
      };

      let mut filled_vec = vec![empty_string_vec; need_to_fill];
      if need_to_fill > 0 {
        cloned_v.append(&mut filled_vec);
      }

      cloned_v.clone()
    }).collect();

    Matrix { contents: filled_matrix }
  }

  pub fn transpose(&self) -> Matrix {
    let col_len = self.contents.len();
    let row_len = self.contents[0].len();

    let mut transposed_vec: Vec<Vec<String>> = Vec::new();
    let mut row_vec: Vec<String> = Vec::new();

    for i in 0..row_len {
      for j in 0..col_len {
        row_vec.push(self.contents[j][i].clone());
      }
      transposed_vec.push(row_vec.clone());
      row_vec.clear();
    }

    Matrix { contents: transposed_vec }
  }

  pub fn is_ascii(&self, v: &Vec<String>) -> bool {
    v.iter().all(|s| s.is_ascii())
  }

  pub fn max_len(&self) -> usize {
    self.contents.iter().fold(0, |sum, v| {
      if sum < v.len() {
        v.len()
      } else {
        sum
      }
    })
  }

  pub fn print(&self) {
    self.contents.iter().for_each(|v| {
      let line = v.iter().fold(String::from(""), |l, w| {
        l + w
      });

      println!("{}", line);
    })
  }
}

