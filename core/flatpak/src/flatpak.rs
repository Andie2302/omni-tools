use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel {
    User,
    Sudo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    Quiet,
    Interactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlatpakScope {
    User,
    System,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatpakApp {
    pub name: String,
    pub app_id: String,
    pub version: String,
    pub branch: String,
}

pub struct FlatpakManager {
    scope: FlatpakScope,
    remote: String,
}

impl FlatpakManager {
    pub fn new(scope: FlatpakScope) -> Self {
        Self {
            scope,
            remote: "flathub".to_string(),
        }
    }

    pub fn with_remote(mut self, remote: &str) -> Self {
        self.remote = remote.to_string();
        self
    }

    fn privilege(&self) -> PrivilegeLevel {
        match self.scope {
            FlatpakScope::User => PrivilegeLevel::User,
            FlatpakScope::System => PrivilegeLevel::Sudo,
        }
    }

    fn apply_scope_arg(&self, args: &mut Arguments) {
        let flag = match self.scope {
            FlatpakScope::User => "user",
            FlatpakScope::System => "system",
        };

        args.push(
            Argument::builder()
                .prefix("--")
                .key(flag)
                .build()
                .expect("Valid scope argument"),
        );
    }

    /// Hilfsmethode zur Ausführung der erstellten `Command`-Instanz über `std::process::Command`
    fn execute(
        cmd: Command<'_>,
        privilege: PrivilegeLevel,
        mode: ExecutionMode,
    ) -> Result<String, String> {
        let mut sys_cmd = match privilege {
            PrivilegeLevel::User => cmd.to_exec_cmd(),
            PrivilegeLevel::Sudo => {
                let mut sudo = ProcessCommand::new("sudo");
                sudo.arg(cmd.path);
                sudo.args(cmd.to_args());
                sudo
            }
        };

        match mode {
            ExecutionMode::Interactive => {
                let status = sys_cmd
                    .status()
                    .map_err(|e| format!("Fehler beim Starten des Befehls: {}", e))?;

                if status.success() {
                    Ok(String::new())
                } else {
                    Err(format!("Prozess beendet mit Status: {}", status))
                }
            }
            ExecutionMode::Quiet => {
                let output = sys_cmd
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .map_err(|e| format!("Fehler beim Ausführen des Befehls: {}", e))?;

                if output.status.success() {
                    String::from_utf8(output.stdout)
                        .map_err(|e| format!("Ungültige UTF-8 Ausgabe: {}", e))
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(stderr.trim().to_string())
                }
            }
        }
    }

    pub fn install(&self, app_id: &str, non_interactive: bool) -> Result<String, String> {
        let mut args = Arguments::new();

        args.push(Argument::builder().key("install").build().unwrap());
        self.apply_scope_arg(&mut args);

        if non_interactive {
            args.push(
                Argument::builder()
                    .prefix("--")
                    .key("noninteractive")
                    .build()
                    .unwrap(),
            );
            args.push(Argument::builder().prefix("-").key("y").build().unwrap());
        }

        args.push(Argument::builder().key(&self.remote[..]).build().unwrap());
        args.push(Argument::builder().key(app_id).build().unwrap());

        let mode = if non_interactive {
            ExecutionMode::Quiet
        } else {
            ExecutionMode::Interactive
        };

        let cmd = Command::new("flatpak", args);
        Self::execute(cmd, self.privilege(), mode)
    }

    pub fn is_installed(&self, app_id: &str) -> bool {
        let mut args = Arguments::new();

        args.push(Argument::builder().key("info").build().unwrap());
        self.apply_scope_arg(&mut args);
        args.push(Argument::builder().key(app_id).build().unwrap());

        let cmd = Command::new("flatpak", args);
        Self::execute(cmd, self.privilege(), ExecutionMode::Quiet).is_ok()
    }

    pub fn list_apps(&self) -> Result<Vec<FlatpakApp>, String> {
        let mut args = Arguments::new();

        args.push(Argument::builder().key("list").build().unwrap());
        self.apply_scope_arg(&mut args);
        args.push(Argument::builder().prefix("--").key("app").build().unwrap());

        // Spalten-Format vorgeben für einfaches Tab-Parsing
        args.push(
            Argument::builder()
                .prefix("--")
                .key("columns")
                .separator("=")
                .value("name,application,version,branch")
                .build()
                .unwrap(),
        );

        let cmd = Command::new("flatpak", args);
        let raw_output = Self::execute(cmd, self.privilege(), ExecutionMode::Quiet)?;

        let mut apps = Vec::new();
        for line in raw_output.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 4 {
                apps.push(FlatpakApp {
                    name: parts[0].trim().to_string(),
                    app_id: parts[1].trim().to_string(),
                    version: parts[2].trim().to_string(),
                    branch: parts[3].trim().to_string(),
                });
            }
        }

        Ok(apps)
    }

    pub fn uninstall(&self, app_id: &str) -> Result<String, String> {
        let mut args = Arguments::new();

        args.push(Argument::builder().key("uninstall").build().unwrap());
        self.apply_scope_arg(&mut args);
        args.push(Argument::builder().prefix("-").key("y").build().unwrap());
        args.push(Argument::builder().key(app_id).build().unwrap());

        let cmd = Command::new("flatpak", args);
        Self::execute(cmd, self.privilege(), ExecutionMode::Quiet)
    }
}