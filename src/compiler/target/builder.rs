use core::fmt;

pub trait BuildItems {
  fn get_items<'a>(&'a mut self) -> &'a mut Vec<Item>;

  fn add_enter_after(&self) -> bool {
    return false;
  }

  fn function(&mut self, before_contents: Inline, contents: Block) {
    let block = Item::Block(before_contents.items, contents.items);
    self.get_items().push(block);
    self.if_enter();
  }

  fn inline(&mut self, contents: Inline) {
    let inline = Item::Inline(contents.items);
    self.get_items().push(inline);
    self.if_enter();
  }

  fn code(&mut self, code: impl Into<String>) {
    self.get_items().push(Item::Code(code.into()));
    self.if_enter();
  }

  fn if_enter(&mut self) {
    if self.add_enter_after() {
      self.enter();
    }
  }
  fn enter(&mut self) {
    self.get_items().push(Item::Enter);
  }

  // Unused:
  // fn comment<'a>(&mut self, message: &'a str) {
  //   for item in str_to_list(message) {
  //     self.get_items().push(Item::Comment(item.into()));
  //   }
  // }
}

pub struct Block {
  items: Vec<Item>,
}

pub struct Inline {
  items: Vec<Item>,
}

impl Block {
  pub fn new() -> Self {
    Self { items: vec![] }
  }
}

impl Inline {
  pub fn new() -> Self {
    Self { items: vec![] }
  }
  pub fn from_str(input: impl Into<String>) -> Self {
    Self {
      items: vec![Item::Code(input.into())],
    }
  }
}

impl BuildItems for Block {
  fn get_items<'a>(&'a mut self) -> &'a mut Vec<Item> {
    &mut self.items
  }
}

impl BuildItems for Inline {
  fn get_items<'a>(&'a mut self) -> &'a mut Vec<Item> {
    &mut self.items
  }
}

#[derive(Clone)]
pub enum Item {
  Code(String),
  Enter,

  /// This can be used to have multiple Items one 1 line
  Inline(Vec<Item>),

  /// The first argument is the prefix of the block,
  /// after that the Vec with items will be wrapped in the data inside LangBuilder::block
  Block(Vec<Item>, Vec<Item>),
  // Unused:
  // Comment(String),
}

impl Item {
  fn get_lines(self, builder: &LangBuilder) -> Vec<Option<String>> {
    match self {
      Self::Code(data) => vec![Some(data)],
      Self::Enter => vec![None],
      Self::Inline(items) => {
        let mut out: Vec<String> = vec![];
        for item in items {
          out.push(item.get_line(builder));
        }
        vec![Some(out.join(""))]
      }
      Self::Block(prefix, items) => {
        let mut prefix_items: Vec<String> = vec![];
        for item in prefix {
          prefix_items.push(item.get_line(builder));
        }
        let prefix_str = prefix_items.join("") + &builder.block.0;

        if items.len() == 0 {
          return vec![Some(prefix_str + &builder.block.1)];
        }

        let mut res = vec![Some(prefix_str)];

        for item in items {
          for line in item.get_lines(builder) {
            if let Some(line_data) = line {
              res.push(Some(format!("{}{}", builder.tabs_or_spaces, line_data)));
            } else {
              res.push(None);
            }
          }
        }

        res.push(Some(builder.block.1.clone()));
        res
      }
    }
  }
  fn get_line(self, builder: &LangBuilder) -> String {
    match self {
      Self::Code(data) => data,
      Self::Enter => String::new(),
      Self::Inline(items) => {
        let mut out: Vec<String> = vec![];
        for item in items {
          out.push(item.get_line(builder));
        }
        out.join("")
      }
      Self::Block(_, _) => String::new(), // TODO this is not yet used inside a inline but i would like to also support this
    }
  }
}

pub enum TabsOrSpaces {
  // Unused:
  // Tabs,
  Spaces(u8),
}

impl fmt::Display for TabsOrSpaces {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Spaces(amount) => write!(f, "{:1$}", " ", *amount as usize),
    }
  }
}

pub struct LangBuilder {
  pub tabs_or_spaces: TabsOrSpaces,
  pub comments: String,
  items: Vec<Item>,
  /// This will be shown before and after a block of code with ofcourse enters between and
  /// the contents will have tabs or spaces depending on the configuration
  pub block: (String, String),
}

impl LangBuilder {
  pub fn new() -> Self {
    Self {
      tabs_or_spaces: TabsOrSpaces::Spaces(2),
      comments: String::from("// "),
      block: (" {".into(), "}".into()),
      items: vec![],
    }
  }
}

impl fmt::Display for LangBuilder {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut res: Vec<String> = vec![];

    for item in self.items.clone() {
      for line in item.get_lines(self) {
        res.push(if let Some(line_data) = line {
          line_data
        } else {
          String::new()
        });
      }
    }

    write!(f, "{}", res.join("\n"))
  }
}

impl BuildItems for LangBuilder {
  fn get_items<'a>(&'a mut self) -> &'a mut Vec<Item> {
    &mut self.items
  }
  fn add_enter_after(&self) -> bool {
    true
  }
}
