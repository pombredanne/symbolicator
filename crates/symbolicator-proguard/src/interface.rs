use std::sync::Arc;

use serde::{Deserialize, Serialize};
use symbolic::common::DebugId;
use symbolicator_service::types::Scope;
use symbolicator_sources::SourceConfig;

/// A request for symbolication/remapping of a JVM event.
#[derive(Debug, Clone)]
pub struct SymbolicateJvmStacktraces {
    /// The scope of this request which determines access to cached files.
    pub scope: Scope,
    /// A list of external sources to load debug files.
    pub sources: Arc<[SourceConfig]>,
    /// The exceptions to symbolicate/remap.
    pub exceptions: Vec<JvmException>,
    /// A list of proguard files to use for remapping.
    pub modules: Vec<JvmModule>,
    /// Whether to apply source context for the stack frames.
    pub apply_source_context: bool,
}

/// A stack frame in a JVM stacktrace.
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct JvmFrame {
    /// The frame's function name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,

    /// The source file name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// The module name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,

    /// The source file's absolute path.
    pub abs_path: String,

    /// The line number within the source file, starting at `1` for the first line.
    pub lineno: u32,

    /// Source context before the context line.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_context: Vec<String>,

    /// The context line if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_line: Option<String>,

    /// Post context after the context line
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub post_context: Vec<String>,

    /// Whether the frame is related to app-code (rather than libraries/dependencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app: Option<bool>,
}

/// An exception in a JVM event.
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct JvmException {
    /// The type (class name) of the exception.
    #[serde(rename = "type")]
    ty: String,
    /// The module in which the exception is defined.
    module: String,
    /// The stacktrace associated with the exception.
    stacktrace: JvmStacktrace,
}

/// A JVM stacktrace.
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct JvmStacktrace {
    /// The stacktrace's frames.
    frames: Vec<JvmFrame>,
}

/// A JVM module (proguard file).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct JvmModule {
    /// The file's UUID.
    ///
    /// This is used to download the file from symbol sources.
    uuid: DebugId,
}