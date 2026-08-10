use command::{Argument, Arguments, Command};

pub struct Flatpak;

impl Flatpak {
    /// Installiert eine App ohne Nachfragen (`-y`).
    /// Streamt die Fortschrittsanzeige live in die Konsole.
    pub fn install(app_id: &str) -> Result<bool, std::io::Error> {
        let mut args = Arguments::new();
        args.push(Argument::from("install"));
        args.push(Argument::from("-y"));
        args.push(Argument::from(app_id));

        Command::new("flatpak", args).exec_stream_logs()
    }

    /// Führt ein System-Update aller Flatpaks aus (`flatpak update -y`).
    pub fn update() -> Result<bool, std::io::Error> {
        let mut args = Arguments::new();
        args.push(Argument::from("update"));
        args.push(Argument::from("-y"));

        Command::new("flatpak", args).exec_stream_logs()
    }

    /// Liest alle installierten App-IDs aus (z. B. für Backups/Restores).
    pub fn list_apps() -> Result<Vec<String>, std::io::Error> {
        let mut args = Arguments::new();
        args.push(Argument::from("list"));
        args.push(Argument::from("--app"));
        args.push(Argument::from("--columns=application"));

        // Liest die Konsolenausgabe als String ein
        let output = Command::new("flatpak", args).exec_read()?;

        // Wandelt die zeilenweise Ausgabe in ein Vec<String> um
        let apps = output
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        Ok(apps)
    }

    /// Deinstalliert eine App inklusive ihrer Daten (`--delete-data`).
    pub fn uninstall(app_id: &str) -> Result<bool, std::io::Error> {
        let mut args = Arguments::new();
        args.push(Argument::from("uninstall"));
        args.push(Argument::from("-y"));
        args.push(Argument::from("--delete-data"));
        args.push(Argument::from(app_id));

        Command::new("flatpak", args).exec_stream_logs()
    }
}