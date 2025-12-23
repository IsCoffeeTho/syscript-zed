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
        
        // let absolute_syscript_dir = Path::new(env::current_dir().unwrap().to_str().unwrap()).join(syscript_dir);
        
        Err("Not going to happen".to_string())
        
        // Ok(zed::Command {
        // 	command: "/home/coffee/Workspace/personal/syscript/sysc".to_string(),
        //  	args: vec![
        //   		"--lsp".to_string()
        //   	],
        //    	env: Default::default()
        // }) 
    }
}

zed::register_extension!(Syscript);
