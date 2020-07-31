#[derive(Debug, Clone)]
pub enum DocElement {
    Text(String),
    Reference(String),
    Warning(String),
}

#[derive(Debug, Clone)]
pub struct Doc {
    content: Vec<DocElement>,
}

impl Doc {
    pub fn new(elements: Vec<DocElement>) -> Self {
        Self { content: elements }
    }

    pub fn empty() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    pub fn text(content: &str) -> Self {
        Self {
            content: vec![DocElement::Text(content.to_owned())],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl<'a> IntoIterator for &'a Doc {
    type Item = &'a DocElement;
    type IntoIter = std::slice::Iter<'a, DocElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.content.iter()
    }
}

impl Default for Doc {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<&str> for Doc {
    fn from(from: &str) -> Self {
        Doc::text(from)
    }
}

pub struct DocBuilder {
    elements: Vec<DocElement>,
}

impl DocBuilder {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn build(self) -> Doc {
        Doc::new(self.elements)
    }

    pub fn text(mut self, text: &str) -> Self {
        self.elements.push(DocElement::Text(text.to_owned()));
        self
    }

    pub fn reference(mut self, typename: &str) -> Self {
        self.elements
            .push(DocElement::Reference(typename.to_owned()));
        self
    }

    pub fn warn(mut self, text: &str) -> Self {
        self.elements.push(DocElement::Warning(text.to_owned()));
        self
    }
}

impl From<DocBuilder> for Doc {
    fn from(from: DocBuilder) -> Self {
        from.build()
    }
}
