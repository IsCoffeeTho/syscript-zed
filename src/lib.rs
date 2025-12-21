use zed_extension_api::{self as zed};

struct Syscript {}

impl zed::Extension for Syscript {
    fn new() -> Self {
        Self {}
    }
    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
    
        // zed::set_language_server_installation_status(
        //     language_server_id,
        //     &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        // );

        // let release = zed::latest_github_release(
        //     "IsCoffeeTho/syscript",
        //     zed::GithubReleaseOptions {
        //         require_assets: true,
        //         pre_release: false,
        //     },
        // )?;

        // let syscript_dir = format!("syscript-{}", release.version);

        // if !fs::metadata(&syscript_dir).map_or(false, |stat| stat.is_dir()) {
        //     zed::set_language_server_installation_status(
        //         language_server_id,
        //         &zed::LanguageServerInstallationStatus::Downloading,
        //     );
        //     zed::download_file(
        //         &release.assets[0].download_url,
        //         &syscript_dir,
        //         zed::DownloadedFileType::Zip,
        //     )
        //     .map_err(|e| format!("failed to download file: {e}"))?;
        // }
        
        // let absolute_syscript_dir = Path::new(env::current_dir().unwrap().to_str().unwrap()).join(syscript_dir);
        
        println!("Hello World!");
        
        Ok(zed::Command {
        	command: "/home/coffee/Workspace/syscript/sysc".to_string(),
         	args: vec![
          		"--lsp".to_string()
          	],
           	env: Default::default()
        }) 
    }
}

zed::register_extension!(Syscript);
