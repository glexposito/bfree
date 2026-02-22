use super::MEMINFO_PATH;

#[derive(Debug)]
pub enum LinuxMemError {
    Io(std::io::Error),
    ParseLine { line: usize, content: String },
    MissingKey(&'static str),
    UnsupportedUnit { key: &'static str, unit: String },
}

impl std::fmt::Display for LinuxMemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinuxMemError::Io(e) => write!(f, "failed to read {MEMINFO_PATH}: {e}"),
            LinuxMemError::ParseLine { line, content } => {
                write!(f, "failed to parse {MEMINFO_PATH} line {line}: {content}")
            }
            LinuxMemError::MissingKey(key) => write!(f, "missing key in {MEMINFO_PATH}: {key}"),
            LinuxMemError::UnsupportedUnit { key, unit } => {
                write!(f, "unsupported unit for {key} in {MEMINFO_PATH}: {unit}")
            }
        }
    }
}

impl From<std::io::Error> for LinuxMemError {
    fn from(e: std::io::Error) -> Self {
        LinuxMemError::Io(e)
    }
}
