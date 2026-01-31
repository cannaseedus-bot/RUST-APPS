use anyhow::Result;
use clap::{Parser, Subcommand};
use chrono::Local;
use colored::*;
use std::io::Write;
use std::path::PathBuf;

mod commands;
mod config;
mod project;
mod ai;
mod web;
mod builder;
mod types;

use config::Config;
use types::{
    ApiCommands, ConfigCommands, DbCommands, DeployTarget, FsCommands, PluginCommands,
    ComponentType,
};

#[derive(Parser)]
#[command(
    name = "nexus",
    author = "Nexus Studio Team",
    version = "1.0.0",
    about = "🚀 Build apps instantly with Phi-3 AI - Rust CLI",
    long_about = r#"
Nexus Studio AI CLI - High-performance app builder with AI integration

Features:
  🤖 Phi-3 AI integration for code generation
  🎯 Component-based architecture
  ⚡ Lightning-fast builds with Rust
  📱 Web interface with Warp server
  🐳 Docker deployment ready
  🔧 Extensible plugin system

Examples:
  nexus new my-app                    # Create new project
  nexus component button PrimaryButton  # Generate component
  nexus build                         # Build project
  nexus serve                         # Start development server
  nexus ai "create login form"        # Generate code with AI
  nexus web                           # Launch web interface
"#
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    verbose: bool,

    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project
    New {
        /// Project name
        name: String,

        /// Template to use
        #[arg(short, long, default_value = "default")]
        template: String,

        /// Framework to use
        #[arg(short, long, default_value = "react")]
        framework: String,

        /// Initialize with AI assistance
        #[arg(short = 'a', long)]
        ai: bool,
    },

    /// Generate a component
    Component {
        /// Component type (page, layout, ui, api, util)
        #[arg(value_enum)]
        component_type: ComponentType,

        /// Component name
        name: String,

        /// Generate with AI
        #[arg(short = 'a', long)]
        ai: bool,

        /// Framework for the component
        #[arg(short, long, default_value = "react")]
        framework: String,
    },

    /// Build project
    Build {
        /// Build mode
        #[arg(short, long, default_value = "production")]
        mode: String,

        /// Target platform
        #[arg(short, long, default_value = "web")]
        target: String,

        /// Output directory
        #[arg(short, long)]
        out_dir: Option<PathBuf>,
    },

    /// Serve project locally
    Serve {
        /// Port to serve on
        #[arg(short, long, default_value_t = 3000)]
        port: u16,

        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Open browser automatically
        #[arg(short, long)]
        open: bool,
    },

    /// Deploy project
    Deploy {
        /// Deployment target
        #[arg(value_enum)]
        target: DeployTarget,

        /// Environment
        #[arg(short, long, default_value = "production")]
        env: String,

        /// Deploy with preview
        #[arg(short, long)]
        preview: bool,
    },

    /// AI code generation
    Ai {
        /// AI prompt
        prompt: String,

        /// AI model to use
        #[arg(short, long, default_value = "phi-3-mini")]
        model: String,

        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Framework for generated code
        #[arg(short, long, default_value = "react")]
        framework: String,
    },

    /// Database operations
    Db {
        #[command(subcommand)]
        db_command: DbCommands,
    },

    /// API operations
    Api {
        #[command(subcommand)]
        api_command: ApiCommands,
    },

    /// File system operations
    Fs {
        #[command(subcommand)]
        fs_command: FsCommands,
    },

    /// Start web interface
    Web {
        /// Port for web interface
        #[arg(short, long, default_value_t = 8080)]
        port: u16,

        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Enable AI features
        #[arg(short = 'a', long)]
        ai: bool,
    },

    /// Plugin management
    Plugin {
        #[command(subcommand)]
        plugin_command: PluginCommands,
    },

    /// Configuration management
    Config {
        #[command(subcommand)]
        config_command: ConfigCommands,
    },

    /// List available templates
    Templates,

    /// Show version and information
    Info,

    /// Clear cache and temporary files
    Clean,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
            let level = match record.level() {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Info => "INFO".green(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Trace => "TRACE".cyan(),
            };
            writeln!(buf, "{} [{}] {}", timestamp, level, record.args())
        })
        .init();

    let cli = Cli::parse();

    let config = if let Some(config_path) = &cli.config {
        Config::load(config_path)?
    } else {
        Config::default()
    };

    match &cli.command {
        Commands::New { name, template, framework, ai } => {
            commands::new_project(name, template, framework, *ai).await?;
        }

        Commands::Component { component_type, name, ai, framework } => {
            commands::create_component(component_type, name, *ai, framework).await?;
        }

        Commands::Build { mode, target, out_dir } => {
            commands::build_project(mode, target, out_dir.as_ref()).await?;
        }

        Commands::Serve { port, host, open } => {
            commands::serve_project(*port, host, *open).await?;
        }

        Commands::Deploy { target, env, preview } => {
            commands::deploy_project(target, env, *preview).await?;
        }

        Commands::Ai { prompt, model, output, framework } => {
            commands::ai_generate(prompt, model, output.as_ref(), framework).await?;
        }

        Commands::Db { db_command } => {
            commands::handle_db(db_command).await?;
        }

        Commands::Api { api_command } => {
            commands::handle_api(api_command).await?;
        }

        Commands::Fs { fs_command } => {
            commands::handle_fs(fs_command).await?;
        }

        Commands::Web { port, host, ai } => {
            commands::start_web_server(*port, host, *ai).await?;
        }

        Commands::Plugin { plugin_command } => {
            commands::handle_plugin(plugin_command).await?;
        }

        Commands::Config { config_command } => {
            commands::handle_config(config_command, &config).await?;
        }

        Commands::Templates => {
            commands::list_templates().await?;
        }

        Commands::Info => {
            commands::show_info().await?;
        }

        Commands::Clean => {
            commands::clean_cache().await?;
        }
    }

    Ok(())
}
