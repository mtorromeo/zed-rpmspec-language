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
        let path = worktree
            .which("rpm_lsp_server")
            .ok_or_else(|| "rpm_lsp_server must be installed manually. See https://github.com/dcermak/rpm-spec-language-server.".to_string())?;

        let arguments = LspSettings::for_worktree("rpm_lsp_server", worktree)
            .map(|lsp_settings| {
                lsp_settings
                    .binary
                    .and_then(|binary| binary.arguments)
                    // If no arguments are provided, default to enabling the HTTP server.
                    .unwrap_or(vec!["--stdio".to_string()])
            })
            .unwrap_or_default();

        Ok(zed::Command {
            command: path,
            args: arguments,
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

    // fn label_for_completion(
    //     &self,
    //     _language_server_id: &zed_extension_api::LanguageServerId,
    //     completion: Completion,
    // ) -> Option<zed_extension_api::CodeLabel> {
    //     let prefix = match completion.kind? {
    //         CompletionKind::Method | CompletionKind::Function => "def ",
    //         CompletionKind::Constructor
    //         | CompletionKind::Class
    //         | CompletionKind::Interface
    //         | CompletionKind::Module => "class ",
    //         CompletionKind::Variable => "var ",
    //         CompletionKind::Field
    //         | CompletionKind::Constant
    //         | CompletionKind::Value
    //         | CompletionKind::Property => "val ",
    //         CompletionKind::Enum => "enum ",
    //         CompletionKind::Keyword => "",
    //         _ => return None,
    //     };
    //     let name = completion.label;
    //     let code = format!("{prefix}{name}");
    //     let code_len = code.len();
    //     Some(CodeLabel {
    //         code,
    //         spans: vec![CodeLabelSpan::code_range(prefix.len()..code_len)],
    //         filter_range: (0..name.len()).into(),
    //     })
    // }

    // fn label_for_symbol(
    //     &self,
    //     _language_server_id: &zed_extension_api::LanguageServerId,
    //     symbol: Symbol,
    // ) -> Option<CodeLabel> {
    //     let prefix = match symbol.kind {
    //         SymbolKind::Module
    //         | SymbolKind::Class
    //         | SymbolKind::Interface
    //         | SymbolKind::Constructor => "class ",
    //         SymbolKind::Method | SymbolKind::Function => "def ",
    //         SymbolKind::Variable => "var ",
    //         SymbolKind::Property | SymbolKind::Field | SymbolKind::Constant => "val ",
    //         _ => "",
    //     };
    //     let name = symbol.name;
    //     let code = format!("{prefix}{name}");
    //     let code_len = code.len();
    //     Some(CodeLabel {
    //         code,
    //         spans: vec![CodeLabelSpan::code_range(prefix.len()..code_len)],
    //         filter_range: (0..name.len()).into(),
    //     })
    // }
}

zed::register_extension!(RPMSpecExtension);
