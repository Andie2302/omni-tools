use command::{Argument, Arguments, Command};

pub struct Flatpak;

impl Flatpak {
    
    
    pub fn install(app_id: &str) -> Result<bool, std::io::Error> {
        let mut args = Arguments::new();
        args.push(Argument::from("install"));
        args.push(Argument::from("-y"));
        args.push(Argument::from(app_id));

        Command::new("flatpak", args).exec_stream_logs()
    }

    
    pub fn update() -> Result<bool, std::io::Error> {
        let mut args = Arguments::new();
        args.push(Argument::from("update"));
        args.push(Argument::from("-y"));

        Command::new("flatpak", args).exec_stream_logs()
    }

    
    pub fn list_apps() -> Result<Vec<String>, std::io::Error> {
        let mut args = Arguments::new();
        args.push(Argument::from("list"));
        //args.push(Argument::from("--app"));
        //args.push(Argument::from("--columns=application"));

        
        let output = Command::new("flatpak", args).exec_read()?;

        
        let apps = output
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        Ok(apps)
    }

    
    pub fn uninstall(app_id: &str) -> Result<bool, std::io::Error> {
        let mut args = Arguments::new();
        args.push(Argument::from("uninstall"));
        args.push(Argument::from("-y"));
        args.push(Argument::from("--delete-data"));
        args.push(Argument::from(app_id));

        Command::new("flatpak", args).exec_stream_logs()
    }
}