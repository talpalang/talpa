use super::*;

#[derive(Debug, Clone)]
pub enum Type {
  /// Intager compiles into the default intager type of the target language
  Int,
  /// 8 bit intager
  I8,
  /// 16 bit intager
  I16,
  /// 32 bit intager
  I32,
  /// 64 bit intager
  I64,
  /// Unsigned intager compiles into the default intager type of the target language
  UInt,
  /// 8 bit unsigned intager
  U8,
  /// 16 bit unsigned intager
  U16,
  /// 32 bit unsigned intager
  U32,
  /// 64 bit unsigned intager
  U64,

  String,
  Char,
  Struct(Struct),
  Array(Box<Type>),

  /// This is the most un-unusable version of a type, it's just a string with the type
  /// This should not show up in stage 3 (tokens to a language) and it should be handled as critial error
  TypeString(String),
}

impl Into<&'static str> for Type {
  fn into(self) -> &'static str {
    match self {
      Self::Int => "int",
      Self::I8 => "i8",
      Self::I16 => "i16",
      Self::I32 => "i32",
      Self::I64 => "i64",
      Self::UInt => "uint",
      Self::U8 => "u8",
      Self::U16 => "u16",
      Self::U32 => "u32",
      Self::U64 => "u64",
      Self::String => "string",
      Self::Char => "char",
      Self::Struct(_) | Self::Array(_) | Self::TypeString(_) => "Unknown",
    }
  }
}

impl Type {
  fn empty() -> Self {
    Self::TypeString(String::new())
  }
}

pub fn parse_type<'a>(p: &'a mut Parser, go_back_one: bool) -> Result<Type, ParsingError> {
  if go_back_one {
    p.index -= 1;
  }

  match p.try_match(vec![
    (&Type::Int, Type::Int.into(), ""),
    (&Type::I8, Type::I8.into(), ""),
    (&Type::I16, Type::I16.into(), ""),
    (&Type::I32, Type::I32.into(), ""),
    (&Type::I64, Type::I64.into(), ""),
    (&Type::UInt, Type::UInt.into(), ""),
    (&Type::U8, Type::U8.into(), ""),
    (&Type::U16, Type::U16.into(), ""),
    (&Type::U32, Type::U32.into(), ""),
    (&Type::U64, Type::U64.into(), ""),
    (&Type::String, Type::String.into(), ""),
    (&Type::Char, Type::Char.into(), ""),
  ]) {
    Some(matched_type) => {
      let add_to_substract = if let Some(c) = p.next_char() {
        if !legal_name_char(c) {
          return Ok(matched_type.clone());
        }
        1
      } else {
        0
      };
      let name: &'static str = Type::U64.into();
      p.index -= name.len() + add_to_substract;
    }
    _ => {}
  };

  let mut type_name = NameBuilder::new();
  while let Some(c) = p.next_char() {
    match c {
      '=' | ')' | '}' | ',' => {
        p.index -= 1;
        let type_string = type_name.to_string(p)?;
        return Ok(Type::TypeString(type_string));
      }
      _ => {
        type_name.push(c);
      }
    }
  }
  p.unexpected_eof()
}

#[derive(Debug, Clone)]
pub struct Struct {
  /// The struct name if it's a named struct, inline structs don't have names
  name: Option<String>,
  /// The struct fields
  fields: HashMap<String, Type>,
}

pub fn parse_struct<'a>(p: &'a mut Parser, inline: bool) -> Result<Struct, ParsingError> {
  let mut res = Struct {
    name: None,
    fields: HashMap::new(),
  };

  if !inline {
    // Parse the struct name
    let first_name_char = match p.next_while(" \t\n") {
      None => return p.unexpected_eof(),
      Some('{') => {
        return p.error(ParsingErrorType::Custom(
          "Struct requires name for example: \"struct foo {}\"",
        ))
      }
      Some(c) if !legal_name_char(c) => return p.unexpected_char(c),
      Some(c) => c,
    };
    let mut struct_name = NameBuilder::new_with_char(first_name_char);
    while let Some(c) = p.next_char() {
      match c {
        ' ' | '\t' | '\n' => {
          if let Some('{') = p.next_while(" \t") {
            break;
          }
          return p.unexpected_char(c);
        }
        '{' => break,
        _ if legal_name_char(c) => struct_name.push(c),
        _ => return p.unexpected_char(c),
      }
    }

    res.name = Some(struct_name.to_string(p)?);
  } else {
    // Find the struct opening
    match p.next_while(" \t") {
      Some('{') => {}
      Some(c) => return p.unexpected_char(c),
      None => return p.unexpected_eof(),
    }
  }

  // Parse the struct fields
  loop {
    // Parse the field name
    let first_name_char = match p.next_while(" \t\n") {
      None => return p.unexpected_eof(),
      Some('}') => break, // end of struct
      Some(c) if !legal_name_char(c) => return p.unexpected_char(c),
      Some(c) => c,
    };
    let mut field_name_builder = NameBuilder::new_with_char(first_name_char);
    while let Some(c) = p.next_char() {
      match c {
        _ if legal_name_char(c) => field_name_builder.push(c),
        ' ' | '\t' => break,
        _ => return p.unexpected_char(c),
      }
    }
    let field_name = field_name_builder.to_string(p)?;

    // Parse the variable assignment
    if let None = p.next_while(" \t") {
      return p.unexpected_eof();
    };
    let parsed_type = parse_type(p, true)?;

    res.fields.insert(field_name, parsed_type);
  }

  Ok(res)
}
