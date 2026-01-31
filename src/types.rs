use clap::Subcommand;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ComponentType {
    Page,
    Layout,
    Ui,
    Api,
    Util,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DeployTarget {
    Vercel,
    Netlify,
    Docker,
    Static,
    Github,
}

#[derive(Subcommand)]
pub enum DbCommands {
    /// Initialize database
    Init {
        /// Database name
        name: Option<String>,
    },

    /// Run migrations
    Migrate {
        /// Migration directory
        #[arg(short, long)]
        dir: Option<std::path::PathBuf>,
    },

    /// Seed database with data
    Seed {
        /// Seed file
        file: Option<std::path::PathBuf>,
    },

    /// Query database
    Query {
        /// SQL query
        query: String,
    },
}

#[derive(Subcommand)]
pub enum ApiCommands {
    /// Generate API endpoint
    Generate {
        /// Endpoint path
        path: String,

        /// HTTP method
        #[arg(short, long, default_value = "GET")]
        method: String,
    },

    /// Test API endpoint
    Test {
        /// Endpoint URL
        url: String,
    },

    /// Generate OpenAPI documentation
    Docs {
        /// Output file
        #[arg(short, long)]
        output: Option<std::path::PathBuf>,
    },
}

#[derive(Subcommand)]
pub enum FsCommands {
    /// List files
    Ls {
        /// Directory path
        path: Option<std::path::PathBuf>,
    },

    /// Create file or directory
    Mk {
        /// Path to create
        path: std::path::PathBuf,

        /// Create as directory
        #[arg(short, long)]
        dir: bool,
    },

    /// Remove file or directory
    Rm {
        /// Path to remove
        path: std::path::PathBuf,

        /// Recursive removal
        #[arg(short, long)]
        recursive: bool,
    },

    /// Copy file or directory
    Cp {
        /// Source path
        source: std::path::PathBuf,

        /// Destination path
        dest: std::path::PathBuf,

        /// Recursive copy
        #[arg(short, long)]
        recursive: bool,
    },

    /// Move file or directory
    Mv {
        /// Source path
        source: std::path::PathBuf,

        /// Destination path
        dest: std::path::PathBuf,
    },
}

#[derive(Subcommand)]
pub enum PluginCommands {
    /// Install plugin
    Install {
        /// Plugin name or path
        name: String,
    },

    /// List installed plugins
    List,

    /// Remove plugin
    Remove {
        /// Plugin name
        name: String,
    },

    /// Update plugin
    Update {
        /// Plugin name
        name: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Get configuration value
    Get {
        /// Key to get
        key: Option<String>,
    },

    /// Set configuration value
    Set {
        /// Key to set
        key: String,

        /// Value to set
        value: String,
    },

    /// List all configuration
    List,

    /// Reset configuration to defaults
    Reset,
}
