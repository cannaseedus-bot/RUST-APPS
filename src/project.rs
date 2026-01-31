use crate::types::ComponentType;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub template: String,
    pub framework: String,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub root: PathBuf,
    pub config: ProjectConfig,
}

pub struct GeneratedComponent {
    pub path: PathBuf,
}

impl Project {
    pub fn new(name: &str, template: &str, framework: &str) -> Result<Self> {
        Ok(Self {
            root: PathBuf::from(name),
            config: ProjectConfig {
                name: name.to_string(),
                template: template.to_string(),
                framework: framework.to_string(),
            },
        })
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let root = path.as_ref().to_path_buf();
        let config_path = root.join("nexus.yaml");
        let contents = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Missing config at {}", config_path.display()))?;
        let config: ProjectConfig = serde_yaml::from_str(&contents)
            .with_context(|| format!("Invalid config at {}", config_path.display()))?;
        Ok(Self { root, config })
    }

    pub fn create_structure(&self) -> Result<()> {
        std::fs::create_dir_all(self.root.join("src"))?;
        std::fs::create_dir_all(self.root.join("dist"))?;
        std::fs::create_dir_all(self.root.join("templates"))?;
        Ok(())
    }

    pub fn generate_files(&self) -> Result<()> {
        let config_path = self.root.join("nexus.yaml");
        let config_contents = serde_yaml::to_string(&self.config)?;
        std::fs::write(&config_path, config_contents)?;
        self.write_file(
            "README.md",
            &format!(
                "# {}\n\nGenerated with Nexus Studio AI.\n",
                self.config.name
            ),
        )?;
        self.write_file(
            "src/App.js",
            "export default function App() {\n  return <h1>Nexus Studio AI</h1>;\n}\n",
        )?;
        Ok(())
    }

    pub fn write_file(&self, relative_path: &str, contents: &str) -> Result<()> {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, contents)?;
        Ok(())
    }

    pub fn generate_component(
        &self,
        template: &str,
        name: &str,
        framework: &str,
    ) -> Result<GeneratedComponent> {
        let extension = match framework {
            "react" | "nextjs" => "tsx",
            "vue" => "vue",
            "svelte" => "svelte",
            "angular" => "component.ts",
            _ => "jsx",
        };
        let filename = format!("{}.{}", name, extension);
        let path = self.root.join("src/components").join(filename);
        std::fs::create_dir_all(path.parent().unwrap())?;
        let contents = format!(
            "// Generated {} component\n\nexport default function {}() {{\n  return <div>{}</div>;\n}}\n",
            template, name, name
        );
        std::fs::write(&path, contents)?;
        Ok(GeneratedComponent { path })
    }
}

impl ComponentType {
    pub fn template_name(&self) -> &'static str {
        match self {
            ComponentType::Page => "page",
            ComponentType::Layout => "layout",
            ComponentType::Ui => "ui",
            ComponentType::Api => "api",
            ComponentType::Util => "util",
        }
    }
}
