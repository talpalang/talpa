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

struct ParseTypeStateTypeName {
  name: NameBuilder,
}

enum ParseTypeState {
  TypeName(ParseTypeStateTypeName),
}

pub struct ParseType<'a> {
  p: &'a mut Parser,
  res: Type,
  state: ParseTypeState,
}

impl<'a> ParseType<'a> {
  pub fn start(p: &'a mut Parser, go_back_one: bool) -> Result<Type, ParsingError> {
    if go_back_one {
      p.index -= 1;
    }
    let mut s = Self {
      p,
      res: Type::empty(),
      state: ParseTypeState::TypeName(ParseTypeStateTypeName {
        name: NameBuilder::new(),
      }),
    };
    s.parse()?;
    Ok(s.res)
  }
  fn parse(&mut self) -> Result<(), ParsingError> {
    self.p.try_match(&vec![
      (Type::Int, ""),
      (Type::I8, ""),
      (Type::I16, ""),
      (Type::I32, ""),
      (Type::I64, ""),
      (Type::UInt, ""),
      (Type::U8, ""),
      (Type::U16, ""),
      (Type::U32, ""),
      (Type::U64, ""),
      (Type::String, ""),
      (Type::Char, ""),
    ]);

    while let Some(c) = self.p.next_char() {
      match &mut self.state {
        ParseTypeState::TypeName(meta) => match c {
          '=' | ')' | '}' | ',' => {
            self.p.index -= 1;
            let type_string = meta.name.to_string(self.p)?;
            self.res = Type::TypeString(type_string);
            return Ok(());
          }
          _ => {
            meta.name.push(c);
          }
        },
      }
    }
    Ok(())
  }
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
    let parsed_type = ParseType::start(p, true)?;

    res.fields.insert(field_name, parsed_type);
  }

  Ok(res)
}
