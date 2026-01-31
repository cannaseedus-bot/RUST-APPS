use crate::project::Project;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct BuildResult {
    pub output_dir: PathBuf,
    pub size_mb: f64,
    pub file_count: usize,
    pub build_time: f64,
    pub warnings: Option<Vec<String>>,
}

pub struct ProjectBuilder<'a> {
    project: &'a Project,
}

impl<'a> ProjectBuilder<'a> {
    pub fn new(project: &'a Project) -> Self {
        Self { project }
    }

    pub async fn build(&self, mode: &str, target: &str, out_dir: Option<&Path>) -> Result<BuildResult> {
        let start = Instant::now();
        let output_dir = out_dir
            .map(PathBuf::from)
            .unwrap_or_else(|| self.project.root.join("dist"));
        std::fs::create_dir_all(&output_dir)?;
        let output_file = output_dir.join("index.html");
        let contents = format!(
            "<html><body><h1>Nexus Studio AI</h1><p>Mode: {}</p><p>Target: {}</p></body></html>",
            mode, target
        );
        std::fs::write(&output_file, contents)?;
        let file_count = 1;
        let size_mb = (std::fs::metadata(&output_file)?.len() as f64) / (1024.0 * 1024.0);
        let build_time = start.elapsed().as_secs_f64();
        Ok(BuildResult {
            output_dir,
            size_mb,
            file_count,
            build_time,
            warnings: None,
        })
    }
}
