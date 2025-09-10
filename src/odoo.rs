use std::{error::Error, fs::{self}, path::{Path}};

use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Odoo {
    cached_binary_path: Option<String>,
}

impl Odoo {

    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<String, Box<dyn Error>> {

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let version_dir = format!("{}", VERSION);
        fs::create_dir_all(version_dir.clone()).map_err(|err| format!("failed to create version directory: {err}"))?;

        let release = zed::github_release_by_tag_name(
            "odoo/odoo-ls",
            VERSION,
        )?;

        let asset_name = format!("odoo-{}-{}.zip", Odoo::platform(), VERSION);

        let asset = release.assets.iter().find(|asset| asset.name == asset_name).ok_or_else(
            || format!("Odoo: No asset found for asset name {}", asset_name)
        )?;

        let asset_typeshed = release.assets.iter().find(|asset| asset.name == "typeshed.zip").ok_or_else(
            || format!("Odoo: No asset found for asset name {}", "typeshed.zip")
        )?;

        let mut exe_name = String::from("odoo_ls_server");
        if cfg!(windows) {
            exe_name += ".exe";
        }

        let binary_path = format!(
            "{version_dir}/{bin_name}",
            bin_name = match zed::current_platform().0 {
                zed::Os::Windows => format!("{}.exe", "odoo_ls_server"),
                zed::Os::Mac | zed::Os::Linux => "odoo_ls_server".to_string(),
            }
        );

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let path_typeshed = Path::new(&version_dir).join("typeshed");
            if path_typeshed.exists() {
                fs::remove_dir_all(path_typeshed)?;
            }

            zed::download_file(&asset.download_url, &version_dir, zed::DownloadedFileType::Zip)
                .map_err(|err| format!("failed to download file: {err}"))?;

            zed::download_file(&asset_typeshed.download_url, &version_dir, zed::DownloadedFileType::Zip)
                .map_err(|err| format!("failed to download file: {err}"))?;

            zed::make_file_executable(&binary_path)?;

            let entries = fs::read_dir(".")
                .map_err(|err| format!("failed to list working directory {err}"))?;
            for entry in entries {
                let entry = entry.map_err(|err| format!("failed to load directory entry {err}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());

        Ok(binary_path)
    }

    fn platform() -> &'static str {
        let (platform, arch) = zed::current_platform();
        match (platform, arch) {
            (zed::Os::Linux, zed::Architecture::X8664) if cfg!(target_env = "musl") => "alpine-x64",
            (zed::Os::Linux, zed::Architecture::Aarch64) if cfg!(target_env = "musl") => "alpine-arm64",
            (zed::Os::Linux, zed::Architecture::X8664) => "linux-x64",
            (zed::Os::Linux, zed::Architecture::Aarch64) => "linux-arm64",
            (zed::Os::Windows, zed::Architecture::X8664) => "win32-x64",
            (zed::Os::Windows, zed::Architecture::Aarch64) => "win32-arm64",
            (zed::Os::Mac, zed::Architecture::X8664) => "darwin-x64",
            (zed::Os::Mac, zed::Architecture::Aarch64) => "darwin-arm64",
            (_os, arch) => {
                // fallback
                println!("Odoo: Warning: Unsupported platform {platform:?}-{arch:?}");
                Box::leak(format!("unknown").into_boxed_str())
            }
        }
    }
}

impl zed::Extension for Odoo {

    fn new() -> Self {
        Self {
            cached_binary_path: None
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree).map_err(|e| e.to_string())?,
            args: vec![],
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>, String> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>, String> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(Odoo);
