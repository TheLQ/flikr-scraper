use std::backtrace::Backtrace;
use std::path::PathBuf;
use tracing::error;

pub type SResult<R> = Result<R, SError>;

#[derive(Debug, thiserror::Error)]
pub enum SError {
    #[error("Reqwest {0}")]
    Reqwest(#[from] reqwest::Error, Backtrace),

    #[error("Io {0} for {1}")]
    Io(std::io::Error, PathBuf, Backtrace),
}

impl SError {
    // pub fn req() -> impl Fn(reqwest::Error) -> SError {
    //     |e| Self::Reqwest(e, sbt())
    // }
    pub fn io(path: impl Into<PathBuf>) -> impl Fn(std::io::Error) -> SError {
        let path = path.into();
        move |e| Self::Io(e, path.clone(), sbt())
    }

    fn my_backtrace(&self) -> &Backtrace {
        match self {
            SError::Reqwest(_, bt) => bt,
            SError::Io(_, _, bt) => bt,
        }
    }
}

fn sbt() -> Backtrace {
    Backtrace::capture()
}

pub fn pretty_panic(err: SError) {
    error!("⛔⛔⛔ DEAD: {err}\n{}", err.my_backtrace());
}
