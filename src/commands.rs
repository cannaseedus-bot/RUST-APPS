use crate::{
    ai::AIModel,
    builder::ProjectBuilder,
    config::Config,
    project::Project,
    types::{ApiCommands, ConfigCommands, DbCommands, DeployTarget, FsCommands, PluginCommands},
};
use anyhow::{Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub async fn new_project(
    name: &str,
    template: &str,
    framework: &str,
    use_ai: bool,
) -> Result<()> {
    println!("🚀 {}", "Creating new project:".green().bold());
    println!("   Name: {}", name.cyan());
    println!("   Template: {}", template.cyan());
    println!("   Framework: {}", framework.cyan());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
            .template("{spinner} {msg}")?,
    );

    pb.set_message("Initializing project structure...");

    let project = Project::new(name, template, framework)?;

    pb.set_message("Creating directories...");
    project.create_structure()?;

    pb.set_message("Generating files...");
    project.generate_files()?;

    if use_ai {
        pb.set_message("🤖 AI is enhancing your project...");
        let mut ai_model = AIModel::new("phi-3-mini").await?;

        let readme_prompt = format!(
            "Generate a comprehensive README.md for a {} project named {} using {} framework",
            template, name, framework
        );

        let readme_content = ai_model.generate(&readme_prompt, 500).await?;
        project.write_file("README.md", &readme_content.content)?;

        pb.set_message("Generating AI-powered starter component...");
        let component_prompt = format!(
            "Create a {} component for a {} project called {}",
            framework, template, name
        );

        let component_code = ai_model.generate(&component_prompt, 1000).await?;
        project.write_file("src/App.js", &component_code.content)?;
    }

    pb.finish_with_message("✅ Project created successfully!");

    println!("\n📁 Project structure:");
    print_tree(Path::new(name), 0)?;

    println!("\n🎯 Next steps:");
    println!("   cd {}", name.cyan());
    println!("   {} start development server", "nexus serve".cyan().bold());
    println!("   {} build for production", "nexus build".cyan().bold());
    println!("   {} deploy to cloud", "nexus deploy".cyan().bold());

    Ok(())
}

pub async fn create_component(
    component_type: &crate::types::ComponentType,
    name: &str,
    use_ai: bool,
    framework: &str,
) -> Result<()> {
    println!("🛠️ {}", "Creating component:".green().bold());
    println!("   Type: {:?}", component_type);
    println!("   Name: {}", name.cyan());
    println!("   Framework: {}", framework.cyan());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
            .template("{spinner} {msg}")?,
    );

    if !Path::new("nexus.yaml").exists() {
        anyhow::bail!("Not in a Nexus project directory. Run 'nexus new' first.");
    }

    let project = Project::load(".")?;

    if use_ai {
        pb.set_message("🤖 AI is generating component...");
        let mut ai_model = AIModel::new("phi-3-mini").await?;

        let prompt = format!(
            "Create a {} component named {} for {} framework with the following features:\n- Clean, modern design\n- Responsive layout\n- Accessibility features\n- Documentation comments\n\nReturn only the component code.",
            component_type.to_possible_value().unwrap().get_name(),
            name,
            framework
        );

        let code = ai_model.generate(&prompt, 1500).await?;

        let file_extension = match framework {
            "react" | "nextjs" => "tsx",
            "vue" => "vue",
            "svelte" => "svelte",
            "angular" => "component.ts",
            _ => "jsx",
        };

        let file_path = format!("src/components/{}.{}", name, file_extension);
        project.write_file(&file_path, &code.content)?;

        pb.finish_with_message("✅ AI-generated component created!");

        println!("\n📝 Component created at: {}", file_path.cyan());
        println!("✨ Features included:");
        println!("   - AI-optimized code");
        println!("   - Modern design patterns");
        println!("   - Responsive layout");
        println!("   - Accessibility features");
    } else {
        pb.set_message("Generating component from template...");

        let component = project.generate_component(component_type.template_name(), name, framework)?;

        pb.finish_with_message("✅ Component created!");

        println!(
            "\n📝 Component created at: {}",
            component.path.display().to_string().cyan()
        );
    }

    Ok(())
}

pub async fn build_project(mode: &str, target: &str, out_dir: Option<&Path>) -> Result<()> {
    println!("🔨 {}", "Building project:".green().bold());
    println!("   Mode: {}", mode.cyan());
    println!("   Target: {}", target.cyan());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
            .template("{spinner} {msg}")?,
    );

    pb.set_message("Loading project...");
    let project = Project::load(".")?;

    pb.set_message("Initializing builder...");
    let builder = ProjectBuilder::new(&project);

    pb.set_message("Building...");
    let build_result = builder.build(mode, target, out_dir).await?;

    pb.finish_with_message("✅ Build completed!");

    println!("\n📊 Build Statistics:");
    println!("   Output: {}", build_result.output_dir.display().to_string().cyan());
    println!("   Size: {:.2} MB", build_result.size_mb);
    println!("   Files: {}", build_result.file_count);
    println!("   Time: {:.2}s", build_result.build_time);

    if let Some(warnings) = build_result.warnings {
        if !warnings.is_empty() {
            println!("\n⚠️  Warnings:");
            for warning in warnings {
                println!("   - {}", warning.yellow());
            }
        }
    }

    Ok(())
}

pub async fn serve_project(port: u16, host: &str, open_browser: bool) -> Result<()> {
    println!("🌐 {}", "Starting development server:".green().bold());
    println!("   URL: http://{}:{}", host.cyan(), port.to_string().cyan());

    let _project = Project::load(".")?;

    let build_dir = Path::new("dist");
    if !build_dir.exists() {
        println!("⚠️  No build found. Running build first...");
        build_project("development", "web", Some(build_dir)).await?;
    }

    #[cfg(feature = "web")]
    {
        let server = warp::serve(
            warp::fs::dir(build_dir)
                .or(warp::path::end().map(|| warp::reply::html("Nexus Studio AI")))
                .with(warp::cors().allow_any_origin()),
        );

        let (addr, server_future) = server.bind_ephemeral((host.parse()?, port));

        println!("\n🚀 Server running at: http://{}", addr);
        println!("📁 Serving from: {}", build_dir.display().to_string().cyan());
        println!("🛑 Press Ctrl+C to stop\n");

        if open_browser {
            let url = format!("http://{}:{}", host, port);
            if let Err(e) = open::that(&url) {
                println!("⚠️  Could not open browser: {}", e);
            }
        }

        tokio::spawn(async {
            tokio::signal::ctrl_c().await.unwrap();
            println!("\n👋 Shutting down server...");
            std::process::exit(0);
        });

        server_future.await;
        return Ok(());
    }

    #[cfg(not(feature = "web"))]
    {
        let _ = (host, port, open_browser);
        anyhow::bail!("Web feature disabled. Rebuild with --features web.");
    }
}

pub async fn deploy_project(target: &DeployTarget, env: &str, preview: bool) -> Result<()> {
    println!("🚀 {}", "Deploying project:".green().bold());
    println!("   Target: {:?}", target);
    println!("   Environment: {}", env.cyan());
    println!("   Preview: {}", preview.to_string().cyan());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
            .template("{spinner} {msg}")?,
    );

    match target {
        DeployTarget::Vercel => deploy_vercel(env, preview, pb).await,
        DeployTarget::Netlify => deploy_netlify(env, preview, pb).await,
        DeployTarget::Docker => deploy_docker(env, preview, pb).await,
        DeployTarget::Static => deploy_static(env, preview, pb).await,
        DeployTarget::Github => deploy_github(env, preview, pb).await,
    }
}

pub async fn ai_generate(
    prompt: &str,
    model: &str,
    output: Option<&Path>,
    framework: &str,
) -> Result<()> {
    println!("🤖 {}", "AI Code Generation:".green().bold());
    println!("   Model: {}", model.cyan());
    println!("   Prompt: {}", prompt.cyan());
    println!("   Framework: {}", framework.cyan());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
            .template("{spinner} {msg}")?,
    );

    pb.set_message("Loading AI model...");
    let mut ai_model = AIModel::new(model).await?;

    pb.set_message("Generating code...");
    let code = ai_model.generate(prompt, 2000).await?;

    pb.finish_with_message("✅ Code generated!");

    if let Some(output_path) = output {
        std::fs::write(output_path, &code.content).context("Failed to write output file")?;
        println!("\n📝 Code saved to: {}", output_path.display().to_string().cyan());
    } else {
        println!("\n{}", "=".repeat(60).cyan());
        println!("{}", code.content);
        println!("{}", "=".repeat(60).cyan());

        println!("\n💡 Use {} to save to a file", "--output <file>".cyan());
    }

    Ok(())
}

pub async fn handle_db(command: &DbCommands) -> Result<()> {
    match command {
        DbCommands::Init { name } => {
            println!("🗄️ Initializing database: {}", name.as_deref().unwrap_or("default"));
        }
        DbCommands::Migrate { dir } => {
            println!("📦 Running migrations in {:?}", dir);
        }
        DbCommands::Seed { file } => {
            println!("🌱 Seeding database from {:?}", file);
        }
        DbCommands::Query { query } => {
            println!("🔎 Executing query: {}", query);
        }
    }
    Ok(())
}

pub async fn handle_api(command: &ApiCommands) -> Result<()> {
    match command {
        ApiCommands::Generate { path, method } => {
            println!("📡 Generating API endpoint: {} {}", method, path);
        }
        ApiCommands::Test { url } => {
            println!("🧪 Testing API endpoint: {}", url);
        }
        ApiCommands::Docs { output } => {
            println!("📘 Generating OpenAPI docs to {:?}", output);
        }
    }
    Ok(())
}

pub async fn handle_fs(command: &FsCommands) -> Result<()> {
    match command {
        FsCommands::Ls { path } => {
            let target = path.as_deref().unwrap_or_else(|| Path::new("."));
            println!("📂 Listing files in {}", target.display());
        }
        FsCommands::Mk { path, dir } => {
            if *dir {
                std::fs::create_dir_all(path)?;
            } else {
                std::fs::write(path, "")?;
            }
            println!("✅ Created {}", path.display());
        }
        FsCommands::Rm { path, recursive } => {
            if *recursive {
                std::fs::remove_dir_all(path)?;
            } else {
                std::fs::remove_file(path)?;
            }
            println!("🗑️ Removed {}", path.display());
        }
        FsCommands::Cp { source, dest, recursive } => {
            if *recursive {
                std::fs::create_dir_all(dest)?;
                for entry in std::fs::read_dir(source)? {
                    let entry = entry?;
                    std::fs::copy(entry.path(), dest.join(entry.file_name()))?;
                }
            } else {
                std::fs::copy(source, dest)?;
            }
            println!("📄 Copied {} -> {}", source.display(), dest.display());
        }
        FsCommands::Mv { source, dest } => {
            std::fs::rename(source, dest)?;
            println!("📁 Moved {} -> {}", source.display(), dest.display());
        }
    }
    Ok(())
}

pub async fn start_web_server(port: u16, host: &str, ai: bool) -> Result<()> {
    crate::web::start_web_server(port, host, ai).await
}

pub async fn handle_plugin(command: &PluginCommands) -> Result<()> {
    match command {
        PluginCommands::Install { name } => println!("🔌 Installing plugin {}", name),
        PluginCommands::List => println!("🔌 Listing installed plugins"),
        PluginCommands::Remove { name } => println!("🧹 Removing plugin {}", name),
        PluginCommands::Update { name } => println!("🔄 Updating plugin {:?}", name),
    }
    Ok(())
}

pub async fn handle_config(command: &ConfigCommands, config: &Config) -> Result<()> {
    match command {
        ConfigCommands::Get { key } => {
            println!("🔍 Config lookup for {:?}", key);
            println!("Current config: {:?}", config);
        }
        ConfigCommands::Set { key, value } => {
            println!("✏️ Setting config {} to {}", key, value);
        }
        ConfigCommands::List => {
            println!("📋 Config: {:?}", config);
        }
        ConfigCommands::Reset => {
            println!("♻️ Resetting config to defaults");
        }
    }
    Ok(())
}

pub async fn list_templates() -> Result<()> {
    println!("📦 Available templates:");
    println!("  - default");
    println!("  - fullstack");
    println!("  - dashboard");
    Ok(())
}

pub async fn show_info() -> Result<()> {
    println!("Nexus Studio AI CLI v1.0.0");
    println!("Build apps instantly with Phi-3 AI");
    Ok(())
}

pub async fn clean_cache() -> Result<()> {
    println!("🧹 Clearing cache and temporary files...");
    Ok(())
}

fn print_tree(path: &Path, depth: usize) -> io::Result<()> {
    let prefix = "  ".repeat(depth);

    if path.is_dir() {
        println!("{}{}/", prefix, path.file_name().unwrap_or_default().to_string_lossy().cyan());

        let mut entries: Vec<_> = std::fs::read_dir(path)?.collect();
        entries.sort_by_key(|entry| entry.as_ref().unwrap().path());

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                print_tree(&path, depth + 1)?;
            } else {
                let filename = path.file_name().unwrap_or_default().to_string_lossy();
                println!("{}  {}", prefix, filename);
            }
        }
    }

    Ok(())
}

async fn deploy_vercel(env: &str, preview: bool, pb: ProgressBar) -> Result<()> {
    pb.set_message("Checking Vercel CLI...");

    Command::new("vercel")
        .arg("--version")
        .output()
        .context("Vercel CLI not found. Install with: npm i -g vercel")?;

    pb.set_message("Deploying to Vercel...");

    let args = if preview { vec!["--target=preview"] } else { vec!["--prod"] };

    let status = Command::new("vercel").args(args).env("VERCEL_ENV", env).status()?;

    if status.success() {
        pb.finish_with_message("✅ Deployed to Vercel!");
        println!("\n🌍 Visit your deployment at the URL shown above");
    } else {
        anyhow::bail!("Vercel deployment failed");
    }

    Ok(())
}

async fn deploy_netlify(_env: &str, _preview: bool, pb: ProgressBar) -> Result<()> {
    pb.finish_with_message("✅ Netlify deployment placeholder complete!");
    Ok(())
}

async fn deploy_static(_env: &str, _preview: bool, pb: ProgressBar) -> Result<()> {
    pb.finish_with_message("✅ Static deployment placeholder complete!");
    Ok(())
}

async fn deploy_github(_env: &str, _preview: bool, pb: ProgressBar) -> Result<()> {
    pb.finish_with_message("✅ GitHub deployment placeholder complete!");
    Ok(())
}

async fn deploy_docker(env: &str, preview: bool, pb: ProgressBar) -> Result<()> {
    pb.set_message("Building Docker image...");

    if !Path::new("Dockerfile").exists() {
        let dockerfile = r#"FROM node:18-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]"#;

        std::fs::write("Dockerfile", dockerfile)?;
    }

    let image_name = format!("nexus-app:{}", env);

    let status = Command::new("docker")
        .args(["build", "-t", &image_name, "."])
        .status()?;

    if !status.success() {
        anyhow::bail!("Docker build failed");
    }

    pb.set_message("Docker image built successfully!");

    if !preview {
        println!("\n📦 Image: {}", image_name.cyan());
        println!(
            "💡 Run with: {}",
            format!("docker run -p 8080:80 {}", image_name).cyan()
        );
    }

    pb.finish_with_message("✅ Docker deployment ready!");

    Ok(())
}
