#[derive(Debug)]
pub enum Error {
  Discord(String),
  TokioReceive(String),
  TokioTimeout(String),
  Io(std::io::Error)
}

impl std::error::Error for Error { }

impl std::fmt::Display for Error {
  #[track_caller]
  #[inline(always)]
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let message = match self {
        Error::Discord(error) => format!("DiscordError: {}", error),
        Error::TokioReceive(error) => format!("TokioReceiveError: {}", error),
        Error::TokioTimeout(error) => format!("TokioTimeoutError: {}", error),
        Error::Io(error) => format!("IoError: {}", error),
    };
    write!(f, "{}", message)
  }
}

impl From<tokio::sync::watch::error::RecvError> for Error {
  #[track_caller]
  #[inline(always)]
  fn from(error: tokio::sync::watch::error::RecvError) -> Self {
    Self::TokioReceive(format!("{:?}", error))
  }
}

impl From<tokio::time::error::Elapsed> for Error {
  #[track_caller]
  #[inline(always)]
  fn from(error: tokio::time::error::Elapsed) -> Self {
    Self::TokioTimeout(format!("{:?}", error))
  }
}

impl From<std::io::Error> for Error {
  #[track_caller]
  #[inline(always)]
  fn from(error: std::io::Error) -> Self {
    Self::Io(error)
  }
}