use zed_extension_api::{self as zed, serde_json, settings::LspSettings, Result};

struct RPMSpecExtension;

impl zed::Extension for RPMSpecExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let command = worktree
            .which("rpm_lsp_server")
            .ok_or_else(|| "rpm_lsp_server must be installed manually. See https://github.com/dcermak/rpm-spec-language-server.".to_string())?;

        let args = LspSettings::for_worktree("rpm_lsp_server", worktree)
            .map(|lsp_settings| {
                lsp_settings
                    .binary
                    .and_then(|binary| binary.arguments)
                    .unwrap_or(vec!["--stdio".to_string()])
            })
            .unwrap_or_default();

        Ok(zed::Command {
            command,
            args,
            env: worktree.shell_env(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        LspSettings::for_worktree("rpm_lsp_server", worktree)
            .map(|lsp_settings| lsp_settings.initialization_options.clone())
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("rpm_lsp_server", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "rpm_lsp_server": settings
        })))
    }
}

zed::register_extension!(RPMSpecExtension);
