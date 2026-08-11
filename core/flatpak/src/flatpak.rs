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
    
    pub fn uninstall(app_id: &str) -> Result<bool, std::io::Error> {
        let mut args = Arguments::new();
        args.push(Argument::from("uninstall"));
        args.push(Argument::from("-y"));
        args.push(Argument::from("--delete-data"));
        args.push(Argument::from(app_id));

        Command::new("flatpak", args).exec_stream_logs()
    }
}
