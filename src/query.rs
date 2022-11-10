use arrayvec::ArrayVec;

use momoden_password::{Password, PasswordChar};

/// '?' の最大個数。
const COUNT_ANY_MAX: usize = 3;

#[derive(Debug)]
pub(crate) enum Query {
    Password(Password),
    Pattern(Pattern),
}

impl Query {
    pub(crate) fn parse(s: &str) -> Result<Self, QueryParseError> {
        let mut cs = s.chars().filter(|&c| !c.is_whitespace()).peekable();
        let v: ArrayVec<char, { Password::MAX_LEN }> =
            cs.by_ref().take(Password::MAX_LEN).collect();

        if v.is_empty() {
            return Err(QueryParseError::Empty);
        }
        if cs.peek().is_some() {
            return Err(QueryParseError::TooLong);
        }

        let v: PatternVec = v
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                PatternChar::from_char(c).ok_or(QueryParseError::InvalidChar { pos: i, ch: c })
            })
            .collect::<Result<_, _>>()?;

        let count_any = v.iter().filter(|&&e| e == PatternChar::Any).count();
        if count_any > COUNT_ANY_MAX {
            return Err(QueryParseError::TooManyAny);
        }

        let this = if count_any > 0 {
            Self::Pattern(Pattern::new(v))
        } else {
            let v: PasswordVec = v
                .into_iter()
                .map(|e| {
                    let PatternChar::Password(pc) = e else { unreachable!() };
                    pc
                })
                .collect();
            Self::Password(Password::new(&v).unwrap())
        };

        Ok(this)
    }
}

type PasswordVec = ArrayVec<PasswordChar, { Password::MAX_LEN }>;
type PatternVec = ArrayVec<PatternChar, { Password::MAX_LEN }>;

#[derive(Debug)]
pub(crate) struct Pattern(PatternVec);

impl Pattern {
    fn new(inner: PatternVec) -> Self {
        Self(inner)
    }

    fn as_slice(&self) -> &[PatternChar] {
        self.0.as_slice()
    }
}

impl std::ops::Deref for Pattern {
    type Target = [PatternChar];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum PatternChar {
    Password(PasswordChar),
    Any,
}

impl PatternChar {
    pub(crate) fn from_char(c: char) -> Option<Self> {
        match c {
            '?' => Some(Self::Any),
            _ => PasswordChar::from_char(c).map(Self::Password),
        }
    }
}

#[derive(Debug)]
pub(crate) enum QueryParseError {
    Empty,
    TooLong,
    TooManyAny,
    InvalidChar { pos: usize, ch: char },
}

impl std::fmt::Display for QueryParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("入力が空"),
            Self::TooLong => write!(f, "入力は {} 文字まで", Password::MAX_LEN),
            Self::TooManyAny => write!(f, "'?' は {COUNT_ANY_MAX} 個まで"),
            Self::InvalidChar { pos, ch } => write!(f, "{} 個目の文字が無効: '{ch}'", pos + 1),
        }
    }
}
