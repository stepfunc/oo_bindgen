use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use heck::{ToKebabCase, ToLowerCamelCase, ToShoutySnakeCase, ToUpperCamelCase};
use lazy_static::lazy_static;
use thiserror::Error;

/// Names in oo_bindgen are subset of allowed C-style identifiers. They are
/// enforce that names are a limited snake case.
///
/// The may only contain:
///
/// lowercase ascii a ..z
/// OR
/// ascii digits 0..9
/// OR
/// underscores
///
/// Additionally:
///
/// - They MUST begin with lowercase ascii
/// - They CANNOT contain double underscores, e.g. foo__bar
/// - They CANNOT end with an underscore, e.g. foo_bar_
/// - They cannot equal certain reserved identifiers in C and other languages

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    validated: Rc<String>,
}

impl From<Name> for String {
    fn from(x: Name) -> Self {
        x.validated.to_string()
    }
}

impl std::ops::Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.validated.as_str()
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.validated.as_str())
    }
}

impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        &self.as_ref() == other
    }
}

impl PartialEq<String> for Name {
    fn eq(&self, other: &String) -> bool {
        self.as_ref() == other
    }
}

impl PartialEq<str> for Name {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

pub trait IntoName {
    fn into_name(self) -> Result<Name, BadName>;
}

impl<T> IntoName for T
where
    T: AsRef<str>,
{
    fn into_name(self) -> Result<Name, BadName> {
        Name::create(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadName {
    pub(crate) name: String,
    pub(crate) error: NameError,
}

impl BadName {
    fn new(name: String, error: NameError) -> Self {
        Self { name, error }
    }
}

impl std::error::Error for BadName {}

impl Display for BadName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {} error: {}", self.name, self.error)
    }
}

#[non_exhaustive]
#[derive(Error, Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum NameError {
    #[error("Name is an empty string")]
    IsEmpty,
    #[error("Name contains invalid character '{}'", c)]
    CharacterNeverAllowed { c: char },
    #[error("Name must start with lowercase ascii but first character is '{}'", c)]
    FirstCharacterNotLowercaseAscii { c: char },
    #[error(
        "'{}' is a reserved identifier in backend language '{}' ",
        id,
        language
    )]
    ReservedIdentifier {
        id: &'static str,
        language: &'static str,
    },
    #[error("'{}' is a sub-phrase reserved by oo-bindgen itself", phrase)]
    BindgenConflict { phrase: &'static str },
    #[error("Names cannot contain double underscores")]
    ContainsDoubleUnderscore,
    #[error("Names cannot end in underscores")]
    LastCharacterIsUnderscore,
}

impl NameError {
    fn character_never_allowed(c: char) -> NameError {
        NameError::CharacterNeverAllowed { c }
    }

    fn first_character_not_lower_case_ascii(c: char) -> NameError {
        NameError::FirstCharacterNotLowercaseAscii { c }
    }

    fn reserved_identifier(id: &'static str, language: &'static str) -> NameError {
        NameError::ReservedIdentifier { id, language }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.validated.as_str()
    }
}

impl Name {
    /// convert to CamelCase
    pub(crate) fn camel_case(&self) -> String {
        self.validated.to_upper_camel_case()
    }

    /// convert to CAPITAL_SNAKE_CASE
    pub(crate) fn capital_snake_case(&self) -> String {
        self.validated.to_shouty_snake_case()
    }

    /// convert to mixedCase
    pub(crate) fn mixed_case(&self) -> String {
        self.validated.to_lower_camel_case()
    }

    /// convert to kebab-case
    pub(crate) fn kebab_case(&self) -> String {
        self.validated.to_kebab_case()
    }

    /// Create a validated Name
    pub fn create<S: AsRef<str>>(value: S) -> Result<Self, BadName> {
        Self::create_impl(value.as_ref())
    }

    /// Append a name to this one
    #[must_use]
    pub(crate) fn append(&self, other: &Name) -> Self {
        Self {
            validated: Rc::new(format!(
                "{}_{}",
                self.validated.as_str(),
                other.validated.as_str()
            )),
        }
    }

    fn is_allowed(c: char) -> bool {
        c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'
    }

    fn create_impl(value: &str) -> Result<Self, BadName> {
        if let Some((keyword, lang)) = KEYWORD_MAP.get(value) {
            return Err(BadName::new(
                value.to_string(),
                NameError::reserved_identifier(keyword, lang),
            ));
        }

        match value.chars().next() {
            Some(c) => {
                if !c.is_ascii_lowercase() {
                    return Err(BadName::new(
                        value.to_string(),
                        NameError::first_character_not_lower_case_ascii(c),
                    ));
                }
            }
            None => return Err(BadName::new(value.to_string(), NameError::IsEmpty)),
        }

        if let Some(bad_character) = value.chars().find(|c| !Self::is_allowed(*c)) {
            return Err(BadName::new(
                value.to_string(),
                NameError::character_never_allowed(bad_character),
            ));
        }

        if value.contains("__") {
            return Err(BadName::new(
                value.to_string(),
                NameError::ContainsDoubleUnderscore,
            ));
        }

        if let Some('_') = value.chars().last() {
            return Err(BadName::new(
                value.to_string(),
                NameError::LastCharacterIsUnderscore,
            ));
        }

        for phrase in OO_BINDGEN_RESERVED_PHRASES {
            if value.contains(phrase) {
                return Err(BadName::new(
                    value.to_string(),
                    NameError::BindgenConflict { phrase },
                ));
            }
        }

        Ok(Name {
            validated: Rc::new(value.to_string()),
        })
    }
}

lazy_static! {
    static ref KEYWORD_MAP: KeywordMap = KeywordMap::new();
}

const RUST_KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in",
    "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "try", "type",
    "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

const C_KEYWORDS: &[&str] = &[
    "auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else",
    "enum", "extern", "float", "for", "goto", "if", "int", "long", "register", "return", "short",
    "signed", "sizeof", "static", "struct", "switch", "typedef", "union", "unsigned", "void",
    "volatile", "while",
];

const CPP_KEYWORDS: &[&str] = &[
    "alignas",
    "alignof",
    "and",
    "and_eq",
    "asm",
    "auto",
    "bitand",
    "bitor",
    "bool",
    "break",
    "case",
    "catch",
    "char",
    "char16_t",
    "char32_t",
    "char8_t",
    "class",
    "co_await",
    "co_return",
    "co_yield",
    "compl",
    "concept",
    "const",
    "const_cast",
    "consteval",
    "constexpr",
    "constinit",
    "continue",
    "declaration",
    "decltype",
    "default",
    "delete",
    "directive",
    "do",
    "double",
    "dynamic_cast",
    "else",
    "enum",
    "explicit",
    "export",
    "extern",
    "false",
    "float",
    "for",
    "friend",
    "goto",
    "if",
    "inline",
    "int",
    "long",
    "mutable",
    "namespace",
    "new",
    "noexcept",
    "not",
    "not_eq",
    "nullptr",
    "operator",
    "or",
    "or_eq",
    "private",
    "protected",
    "public",
    "register",
    "reinterpret_cast",
    "requires",
    "return",
    "short",
    "signed",
    "sizeof",
    "static",
    "static_assert",
    "static_cast",
    "struct",
    "switch",
    "template",
    "this",
    "thread_local",
    "throw",
    "true",
    "try",
    "typedef",
    "typeid",
    "typename",
    "union",
    "unsigned",
    "using",
    "using",
    "virtual",
    "void",
    "volatile",
    "wchar_t",
    "while",
    "xor",
    "xor_eq",
];

const JAVA_KEYWORDS: &[&str] = &[
    "abstract",
    "assert",
    "boolean",
    "break",
    "byte",
    "case",
    "catch",
    "char",
    "class",
    "const",
    "continue",
    "default",
    "do",
    "double",
    "else",
    "enum",
    "extends",
    "final",
    "finally",
    "float",
    "for",
    "goto",
    "if",
    "implements",
    "import",
    "instanceof",
    "int",
    "interface",
    "long",
    "native",
    "new",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "short",
    "static",
    "strictfp",
    "super",
    "switch",
    "synchronized",
    "this",
    "throw",
    "throws",
    "transient",
    "try",
    "void",
    "volatile",
    "while",
];

const CSHARP_KEYWORDS: &[&str] = &[
    "abstract",
    "as",
    "base",
    "bool",
    "break",
    "byte",
    "case",
    "catch",
    "char",
    "checked",
    "class",
    "const",
    "continue",
    "decimal",
    "default",
    "delegate",
    "do",
    "double",
    "else",
    "enum",
    "event",
    "explicit",
    "extern",
    "false",
    "finally",
    "fixed",
    "float",
    "for",
    "foreach",
    "goto",
    "if",
    "implicit",
    "in",
    "int",
    "interface",
    "internal",
    "is",
    "lock",
    "long",
    "namespace",
    "new",
    "null",
    "object",
    "operator",
    "out",
    "override",
    "params",
    "private",
    "protected",
    "public",
    "readonly",
    "ref",
    "return",
    "sbyte",
    "sealed",
    "short",
    "sizeof",
    "stackalloc",
    "static",
    "string",
    "struct",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "uint",
    "ulong",
    "unchecked",
    "unsafe",
    "ushort",
    "using",
    "virtual",
    "void",
    "volatile",
    "while",
];

/// keywords reserved by oo_bindgen itself
/// these strings cannot show up anywhere in a Name
///
/// this allows backends to use temporary variables that can never
/// conflict with a Name used in an API
const OO_BINDGEN_RESERVED_PHRASES: &[&str] = &[
    "oo_bindgen",
    // reserved name for a helper class in the java backend
    "backend_library_loader",
];

type KeyWordMap = HashMap<&'static str, (&'static str, &'static str)>;

struct KeywordMap {
    map: KeyWordMap,
}

impl KeywordMap {
    fn new() -> Self {
        let mut map = KeyWordMap::new();

        for x in RUST_KEYWORDS {
            map.insert(x, (x, "Rust"));
        }

        for x in C_KEYWORDS {
            map.insert(x, (x, "C"));
        }

        for x in CPP_KEYWORDS {
            map.insert(x, (x, "C++"));
        }

        for x in JAVA_KEYWORDS {
            map.insert(x, (x, "Java"));
        }

        for x in CSHARP_KEYWORDS {
            map.insert(x, (x, "C#"));
        }

        Self { map }
    }

    fn get(&self, x: &str) -> Option<&(&'static str, &'static str)> {
        self.map.get(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_valid_names() {
        assert!(Name::create("a1").is_ok());
        assert!(Name::create("abc_def").is_ok());
        assert!(Name::create("a1_d_2").is_ok());
    }

    #[test]
    fn cannot_equal_reserved_identifiers() {
        assert_eq!(
            Name::create("alignas").err().unwrap().error,
            NameError::reserved_identifier("alignas", "C++")
        );
        assert_eq!(
            Name::create("implements").err().unwrap().error,
            NameError::reserved_identifier("implements", "Java")
        );
        assert_eq!(
            Name::create("delegate").err().unwrap().error,
            NameError::reserved_identifier("delegate", "C#")
        );
        assert_eq!(
            Name::create("box").err().unwrap().error,
            NameError::reserved_identifier("box", "Rust")
        );
    }

    #[test]
    fn cannot_be_empty() {
        assert_eq!(Name::create("").err().unwrap().error, NameError::IsEmpty);
    }

    #[test]
    fn cannot_contain_uppercase() {
        assert_eq!(
            Name::create("aBc").err().unwrap().error,
            NameError::character_never_allowed('B')
        );
    }

    #[test]
    fn cannot_lead_with_underscores_or_numbers() {
        assert_eq!(
            Name::create("_abc").err().unwrap().error,
            NameError::first_character_not_lower_case_ascii('_')
        );
        assert_eq!(
            Name::create("1abc").err().unwrap().error,
            NameError::first_character_not_lower_case_ascii('1')
        );
    }

    #[test]
    fn last_character_cannot_be_underscore() {
        assert_eq!(
            Name::create("abc_").err().unwrap().error,
            NameError::LastCharacterIsUnderscore
        );
    }

    #[test]
    fn cannot_contain_double_underscore() {
        assert_eq!(
            Name::create("abc_def__ghi").err().unwrap().error,
            NameError::ContainsDoubleUnderscore
        );
    }

    #[test]
    fn cannot_contain_a_sub_phrase_reserved_by_oo_bindgen() {
        assert_eq!(
            Name::create("blah_blah_oo_bindgen_blad")
                .err()
                .unwrap()
                .error,
            NameError::BindgenConflict {
                phrase: "oo_bindgen"
            }
        );
    }

    #[test]
    fn can_append_string() {
        assert_eq!(
            Name::create("abc")
                .unwrap()
                .append(&Name::create("def").unwrap())
                .as_ref(),
            "abc_def"
        );
    }

    #[test]
    fn names_are_compared_by_inner_value() {
        assert_eq!(Name::create("abc"), Name::create("abc"));
    }

    #[test]
    fn name_not_equal_works_as_expected() {
        assert_ne!(Name::create("abc"), Name::create("def"));
    }
}
