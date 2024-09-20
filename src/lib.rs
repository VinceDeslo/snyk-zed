use std::fs;

use serde::Deserialize;
use zed_extension_api::{
    self as zed,
    http_client::{HttpMethod, HttpRequest},
    serde_json, LanguageServerId, Result, Worktree,
};
mod init;
use init::SnykInitializationOptions;

#[derive(Deserialize)]
struct MetadataJson {
    pub version: String,
}

pub struct SnykExtension {
    cached_binary_path: Option<String>,
    initialization_options: SnykInitializationOptions,
}

impl SnykExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("snyk-ls") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let protocol_version = 3;

        let metadata_url =
            format!("https://static.snyk.io/snyk-ls/{protocol_version}/metadata.json");

        let metadata_response = zed::http_client::fetch(
            &HttpRequest::builder()
                .method(HttpMethod::Get)
                .header("Content-Type", "application/json")
                .url(metadata_url)
                .build()?,
        )?;

        let metadata: MetadataJson =
            serde_json::from_slice(&metadata_response.body).map_err(|err| err.to_string())?;

        let (platform, arch) = zed::current_platform();
        let download_url = format!(
            "https://static.snyk.io/snyk-ls/{protocol_version}/snyk-ls_{version}_{os}_{arch}",
            version = metadata.version,
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            },
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X86 => "386",
                zed::Architecture::X8664 => "amd64",
            }
        );

        let version_dir = format!("snyk-ls-{}", metadata.version);
        let binary_path = format!("{version_dir}/snyk-ls");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            fs::create_dir_all(&version_dir)
                .map_err(|e| format!("failed to create directory: {e}"))?;

            zed::download_file(
                &download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for SnykExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
            initialization_options: SnykInitializationOptions::default(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: Vec::new(),
            env: worktree.shell_env(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // override defaults
        let init_options = serde_json::to_value(&self.initialization_options).unwrap();
        Ok(Some(init_options))
    }
}

zed::register_extension!(SnykExtension);
