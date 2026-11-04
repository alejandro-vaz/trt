use {
    super::information::{
        GIT_COMMIT_HASH,
        GIT_DIRTY,
        PKG_VERSION
    },
    core::fmt::{
        Display,
        Formatter,
        Result as Formatted
    }
};

#[derive(Debug)]
pub enum Version {
    Experimental,
    Development,
    Release
}

pub static VERSION: Version = match GIT_DIRTY {
    None => Version::Release,
    Some(false) => Version::Development,
    Some(true) => Version::Experimental
};

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> Formatted {
        write!(
            f,
            "{}",
            match self {
                Version::Development =>
                    format!("#{}", unsafe { GIT_COMMIT_HASH.unwrap_unchecked() }),
                Version::Experimental => String::from("(experimental)"),
                Version::Release => format!("v{PKG_VERSION}")
            }
        )
    }
}
