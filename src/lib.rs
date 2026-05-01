use zed_extension_api::{self as zed, settings::LspSettings, Result};

struct KokaExtension;

impl zed::Extension for KokaExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // mostly inspired by https://github.com/zed-extensions/haskell/blob/main/src/haskell.rs
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let default_args = vec!["--language-server".to_string(), "--lsstdio".to_string()];

        if let Some(binary_settings) = lsp_settings.binary {
            if let Some(path) = binary_settings.path {
                return Ok(zed::Command {
                    command: path,
                    args: binary_settings.arguments.unwrap_or(default_args),
                    env: worktree.shell_env(),
                });
            }
        }

        let path = worktree
            .which("koka")
            .ok_or_else(|| "koka must be installed".to_string())?;

        Ok(zed::Command {
            command: path,
            args: default_args,
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(KokaExtension);
