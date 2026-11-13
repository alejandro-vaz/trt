use {
    super::information::GIT_DIRTY,
    core::{
        fmt::{
            Debug,
            Display,
            Formatter,
            Result as Format
        },
        num::NonZero
    }
};

#[derive(Default, Debug)]
pub enum Version {
    #[default]
    Empty,
    #[allow(unused)]
    NonEmpty {
        prefix: Vec<usize>,
        last: NonZero<usize>
    }
}

impl Version {
    pub const COMPILER: Option<Self> = (!GIT_DIRTY.unwrap()).then_some(Version::Empty);
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> Format {
        match self {
            Version::Empty => write!(f, "v"),
            Version::NonEmpty {
                prefix,
                last
            } => write!(
                f,
                "v{}.{last}",
                prefix
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(".")
            )
        }
    }
}
