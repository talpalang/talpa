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
  Enum(Enum),
  Array(Box<Type>),

  /// This references to another type
  TypeRef(String),
}

pub enum DetectType {
  Int,
  I8,
  I16,
  I32,
  I64,
  UInt,
  U8,
  U16,
  U32,
  U64,
  String,
  Char,
  Struct,
  Enum,
  Array,
}

impl DetectType {
  fn to_type(&self) -> Option<Type> {
    Some(match self {
      Self::Int => Type::Int,
      Self::I8 => Type::I8,
      Self::I16 => Type::I16,
      Self::I32 => Type::I32,
      Self::I64 => Type::I64,
      Self::UInt => Type::UInt,
      Self::U8 => Type::U8,
      Self::U16 => Type::U16,
      Self::U32 => Type::U32,
      Self::U64 => Type::U64,
      Self::String => Type::String,
      Self::Char => Type::Char,
      Self::Array | Self::Struct | Self::Enum => return None,
    })
  }
}

impl MatchString for DetectType {
  fn get_string(&self) -> &'static str {
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
      Self::Array => "[]",
      Self::Struct => "struct",
      Self::Enum => "enum",
    }
  }
}

pub fn parse_type<'a>(p: &'a mut Parser, go_back_one: bool) -> Result<Type, LocationError> {
  if go_back_one {
    p.index -= 1;
  }

  if let None = p.next_while(" \t\n") {
    return p.unexpected_eof();
  }
  p.index -= 1;

  match p.try_match(vec![
    &DetectType::Int,
    &DetectType::I8,
    &DetectType::I16,
    &DetectType::I32,
    &DetectType::I64,
    &DetectType::UInt,
    &DetectType::U8,
    &DetectType::U16,
    &DetectType::U32,
    &DetectType::U64,
    &DetectType::String,
    &DetectType::Char,
    &DetectType::Struct,
    &DetectType::Enum,
    &DetectType::Array,
  ]) {
    Some(&DetectType::Array) => {
      let res = parse_type(p, false)?;
      return Ok(Type::Array(Box::new(res)));
    }
    Some(matched_type) => {
      let add_to_substract = if let Some(c) = p.next_char() {
        if let &DetectType::Struct = matched_type {
          if c == '{' || c == ' ' || c == '\n' {
            let res = parse_struct(p, true, c == '{')?;
            return Ok(Type::Struct(res));
          }
        } else if let &DetectType::Enum = matched_type {
          if c == '{' || c == ' ' || c == '\n' {
            let res = parse_enum(p, true, c == '{')?;
            return Ok(Type::Enum(res));
          }
        } else if !valid_name_char(c) {
          if let Some(v) = matched_type.to_type() {
            p.index -= 1;
            return Ok(v);
          }
        }
        1
      } else {
        0
      };
      p.index -= matched_type.get_string().len() + add_to_substract;
    }
    _ => {}
  };

  let mut type_name = NameBuilder::new();
  while let Some(c) = p.next_char() {
    match c {
      _ if valid_name_char(c) => type_name.push(c),
      _ => {
        p.index -= 1;
        let type_string = type_name.to_string(p)?;
        return Ok(Type::TypeRef(type_string));
      }
    }
  }
  p.unexpected_eof()
}

#[derive(Debug, Clone)]
pub struct Enum {
  name: Option<String>,
  fields: Vec<EnumField>,
}

#[derive(Debug, Clone)]
pub struct EnumField {
  name: String,
  value: Option<Action>,
}

pub fn parse_enum<'a>(
  p: &'a mut Parser,
  inline: bool,
  back_one: bool,
) -> Result<Enum, LocationError> {
  if back_one {
    p.index -= 1;
  }

  let mut res = Enum {
    name: None,
    fields: vec![],
  };

  if inline {
    // Find the struct opening
    match p.next_while(" \t") {
      Some('{') => {}
      Some(c) => return p.unexpected_char(c),
      None => return p.unexpected_eof(),
    }
  } else {
    // Parse the struct name
    let first_name_char = match p.next_while(" \t\n") {
      None => return p.unexpected_eof(),
      Some('{') => {
        return p.error(TokenizeError::Custom(
          "Struct requires name for example: \"struct foo {}\"",
        ))
      }
      Some(c) if !valid_name_char(c) => return p.unexpected_char(c),
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
        _ if valid_name_char(c) => struct_name.push(c),
        _ => return p.unexpected_char(c),
      }
    }

    res.name = Some(struct_name.to_string(p)?);
  }

  // Parse the enum fields
  loop {
    // Parse field name
    let first_name_char = match p.next_while(" \t\n") {
      None => return p.unexpected_eof(),
      Some('}') => break, // end of enum
      Some(c) if !valid_name_char(c) => return p.unexpected_char(c),
      Some(c) => c,
    };
    let mut field_name_builder = NameBuilder::new_with_char(first_name_char);
    while let Some(c) = p.next_char() {
      match c {
        _ if valid_name_char(c) => field_name_builder.push(c),
        ' ' | '\t' => break,
        '\n' => {
          p.index -= 1;
          break;
        }
        _ => return p.unexpected_char(c),
      }
    }
    let mut to_add = EnumField {
      name: field_name_builder.to_string(p)?,
      value: None,
    };

    // Parse the = symbol
    match p.next_while(" \t") {
      Some('=') => {
        let action = ParseAction::start(p, false, ActionToExpect::Assignment(","))?;
        match p.next_while(" \t") {
          Some('}') => {
            res.fields.push(to_add);
            break;
          }
          Some('\n') => {}
          Some(c) => return p.unexpected_char(c),
          None => return p.unexpected_eof(),
        }
        to_add.value = Some(action);
      }
      Some('}') => {
        res.fields.push(to_add);
        break;
      }
      Some('\n') => {}
      Some(c) => return p.unexpected_char(c),
      None => return p.unexpected_eof(),
    };

    res.fields.push(to_add);
  }

  Ok(res)
}

#[derive(Debug, Clone)]
pub struct Struct {
  /// The struct name if it's a named struct, inline structs don't have names
  name: Option<String>,
  /// The struct fields
  fields: HashMap<String, Type>,
}

pub fn parse_struct<'a>(
  p: &'a mut Parser,
  inline: bool,
  back_one: bool,
) -> Result<Struct, LocationError> {
  if back_one {
    p.index -= 1;
  }

  let mut res = Struct {
    name: None,
    fields: HashMap::new(),
  };

  if inline {
    // Find the struct opening
    match p.next_while(" \t") {
      Some('{') => {}
      Some(c) => return p.unexpected_char(c),
      None => return p.unexpected_eof(),
    }
  } else {
    // Parse the struct name
    let first_name_char = match p.next_while(" \t\n") {
      None => return p.unexpected_eof(),
      Some('{') => {
        return p.error(TokenizeError::Custom(
          "Struct requires name for example: \"struct foo {}\"",
        ))
      }
      Some(c) if !valid_name_char(c) => return p.unexpected_char(c),
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
        _ if valid_name_char(c) => struct_name.push(c),
        _ => return p.unexpected_char(c),
      }
    }

    res.name = Some(struct_name.to_string(p)?);
  }

  // Parse struct fields
  loop {
    // Parse field name
    let first_name_char = match p.next_while(" \t\n") {
      None => return p.unexpected_eof(),
      Some('}') => break, // end of struct
      Some(c) if !valid_name_char(c) => return p.unexpected_char(c),
      Some(c) => c,
    };
    let mut field_name_builder = NameBuilder::new_with_char(first_name_char);
    while let Some(c) = p.next_char() {
      match c {
        _ if valid_name_char(c) => field_name_builder.push(c),
        ' ' | '\t' => break,
        _ => return p.unexpected_char(c),
      }
    }
    let field_name = field_name_builder.to_string(p)?;

    // Parse field type
    if let None = p.next_while(" \t") {
      return p.unexpected_eof();
    };
    let parsed_type = parse_type(p, true)?;

    res.fields.insert(field_name, parsed_type);
  }

  Ok(res)
}
