use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_state_dir")]
    pub state_dir: PathBuf,

    #[serde(default)]
    pub atcoder: AtCoderConfig,
}

fn default_state_dir() -> PathBuf {
    PathBuf::from(".submission-archiver/state")
}

#[derive(Debug, Deserialize)]
pub struct AtCoderConfig {
    #[serde(default)]
    pub enable: bool,

    pub user_id: String,

    #[serde(default = "default_atcoder_out_dir")]
    pub out_dir: PathBuf,

    #[serde(default)]
    pub out_format: OutFormat,

    #[serde(default)]
    pub use_index: UseIndex,

    #[serde(default)]
    pub archive_targets: ArchiveTargets,

    #[serde(default = "default_request_interval_ms")]
    pub request_interval_ms: u64,

    #[serde(default)]
    pub git: GitConfig,

    #[serde(default = "default_languages")]
    pub languages: Vec<LangRule>,
}

fn default_atcoder_out_dir() -> PathBuf {
    PathBuf::from("archive/atcoder")
}

fn default_request_interval_ms() -> u64 {
    400
}

impl Default for AtCoderConfig {
    fn default() -> Self {
        Self {
            enable: false,
            user_id: String::new(),
            out_dir: default_atcoder_out_dir(),
            out_format: OutFormat::File,
            use_index: UseIndex(false),
            archive_targets: ArchiveTargets::Default,
            request_interval_ms: default_request_interval_ms(),
            git: GitConfig::default(),
            languages: default_languages(),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutFormat {
    File,
    Directory,
}

impl Default for OutFormat {
    fn default() -> Self {
        OutFormat::File
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct UseIndex(pub bool);

impl Default for UseIndex {
    fn default() -> Self {
        UseIndex(false)
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveTargets {
    All,
    AcAll,
    AcLatest,
    Default,
}

impl Default for ArchiveTargets {
    fn default() -> Self {
        ArchiveTargets::Default
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitMode {
    PerFile,
    PerChunk,
    Once,
    None,
}

impl Default for CommitMode {
    fn default() -> Self {
        CommitMode::None
    }
}

#[derive(Debug, Deserialize)]
pub struct GitConfig {
    #[serde(default)]
    pub mode: CommitMode,

    #[serde(default = "default_chunk_size")]
    pub chunk_size: usize,

    #[serde(default = "default_per_file_template")]
    pub per_file_template: String,

    #[serde(default = "default_per_chunk_template")]
    pub per_chunk_template: String,

    #[serde(default = "default_once_template")]
    pub once_template: String,
}

fn default_chunk_size() -> usize {
    50
}

fn default_per_file_template() -> String {
    "[{platform}] archive({kind}): {contest_id}/{problem_id} ({id})".into()
}

fn default_per_chunk_template() -> String {
    "[{platform}] archive chunk: {count} files up to {last_id}".into()
}

fn default_once_template() -> String {
    "[{platform}] archive done: {total_files} files".into()
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            mode: CommitMode::None,
            chunk_size: default_chunk_size(),
            per_file_template: default_per_file_template(),
            per_chunk_template: default_per_chunk_template(),
            once_template: default_once_template(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct LangRule {
    #[serde(rename = "match")]
    pub prefix: String,
    pub id: String,
    pub ext: String,
}

fn default_languages() -> Vec<LangRule> {
    vec![
        LangRule {
            prefix: "TypeScript".into(),
            id: "ts".into(),
            ext: "ts".into(),
        },
        LangRule {
            prefix: "JavaScript".into(),
            id: "js".into(),
            ext: "js".into(),
        },
        LangRule {
            prefix: "C#".into(),
            id: "cs".into(),
            ext: "cs".into(),
        },
        LangRule {
            prefix: "C++".into(),
            id: "cpp".into(),
            ext: "cpp".into(),
        },
        LangRule {
            prefix: "Python".into(),
            id: "py".into(),
            ext: "py".into(),
        },
        LangRule {
            prefix: "PyPy".into(),
            id: "py".into(),
            ext: "py".into(),
        },
        LangRule {
            prefix: "Rust".into(),
            id: "rs".into(),
            ext: "rs".into(),
        },
        LangRule {
            prefix: "Go".into(),
            id: "go".into(),
            ext: "go".into(),
        },
        LangRule {
            prefix: "Java".into(),
            id: "java".into(),
            ext: "java".into(),
        },
        LangRule {
            prefix: "Kotlin".into(),
            id: "kt".into(),
            ext: "kt".into(),
        },
        LangRule {
            prefix: "Ruby".into(),
            id: "rb".into(),
            ext: "rb".into(),
        },
        LangRule {
            prefix: "Swift".into(),
            id: "swift".into(),
            ext: "swift".into(),
        },
        LangRule {
            prefix: "Haskell".into(),
            id: "hs".into(),
            ext: "hs".into(),
        },
        LangRule {
            prefix: "OCaml".into(),
            id: "ml".into(),
            ext: "ml".into(),
        },
        LangRule {
            prefix: "C".into(),
            id: "c".into(),
            ext: "c".into(),
        },
    ]
}

impl Config {
    pub fn load_from_path(path: &Path) -> anyhow::Result<Self> {
        let s = fs::read_to_string(path)?;
        let cfg: Config = toml::from_str(&s)?;
        Ok(cfg)
    }
}
