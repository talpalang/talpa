use super::*;
use errors::LocationError;
use files::CodeLocation;
use statics::{valid_name_char, NameBuilder};
use strings::parse_static_str;

#[derive(Debug)]
pub struct Import {
  name: String,
  path: String_,
  location: CodeLocation,
}

pub fn parse_import(t: &mut Tokenizer) -> Result<Vec<Import>, LocationError> {
  let mut res: Vec<Import> = vec![];

  // Parse all import
  'main: loop {
    let index = t.index;
    let y = t.y;
    let restore_location = |t: &mut Tokenizer| {
      t.index = index;
      t.y = y;
    };

    // Parse a single import
    let location = t.last_index_location();
    let c_option = t.next_while(" \t\n");
    let mut c = if let Some(c) = c_option {
      c
    } else {
      break;
    };

    let mut import_name = NameBuilder::new();
    loop {
      match c {
        ' ' | '\t' | '\n' => break,
        cr if !valid_name_char(cr) => {
          restore_location(t);
          break 'main;
        }
        cr => {
          import_name.push(cr);
          c = t.must_next_char()?;
        }
      };
    }

    c = t.must_next_while(" \t\n")?;
    if c != '"' {
      restore_location(t);
      break 'main;
    }

    let path = parse_static_str(t)?;

    res.push(Import {
      name: import_name.to_string(t)?,
      path,
      location,
    })
  }

  Ok(res)
}
