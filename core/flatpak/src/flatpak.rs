use command::argument::Argument;
use command::arguments::Arguments;
use command::command::{CommandExecutor, ExecutionMode, PrivilegeLevel};
use command::delimiter::DelimiterString;

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
        match self.scope {
            FlatpakScope::User => args.push(Argument::flag("user", "--")),
            FlatpakScope::System => args.push(Argument::flag("system", "--")),
        }
    }

    pub fn install(&self, app_id: &str, non_interactive: bool) -> Result<String, String> {
        let mut args = Arguments::new(" ");
        args.push(Argument::new(
            DelimiterString::new("install", None, None),
            None,
            None,
            None,
        ));

        self.apply_scope_arg(&mut args);

        if non_interactive {
            args.push(Argument::flag("noninteractive", "--"));
            args.push(Argument::flag("y", "-"));
        }

        args.push(Argument::new(
            DelimiterString::new(&self.remote, None, None),
            None,
            None,
            None,
        ));
        args.push(Argument::new(
            DelimiterString::new(app_id, None, None),
            None,
            None,
            None,
        ));

        let mode = if non_interactive {
            ExecutionMode::Quiet
        } else {
            ExecutionMode::Interactive
        };

        CommandExecutor::execute("flatpak", &args, self.privilege(), mode)
    }

    pub fn is_installed(&self, app_id: &str) -> bool {
        let mut args = Arguments::new(" ");
        args.push(Argument::new(
            DelimiterString::new("info", None, None),
            None,
            None,
            None,
        ));

        self.apply_scope_arg(&mut args);

        args.push(Argument::new(
            DelimiterString::new(app_id, None, None),
            None,
            None,
            None,
        ));

        CommandExecutor::execute("flatpak", &args, self.privilege(), ExecutionMode::Quiet).is_ok()
    }

    /// Listet installierte Apps auf und parst sie in eine Struktur
    pub fn list_apps(&self) -> Result<Vec<FlatpakApp>, String> {
        let mut args = Arguments::new(" ");
        args.push(Argument::new(
            DelimiterString::new("list", None, None),
            None,
            None,
            None,
        ));

        self.apply_scope_arg(&mut args);
        args.push(Argument::flag("app", "--"));
        // Spalten-Format vorgeben für einfaches Tab-Parsing
        args.push(Argument::key_value(
            "columns",
            "--",
            "=",
            "name,application,version,branch",
        ));

        let raw_output =
            CommandExecutor::execute("flatpak", &args, self.privilege(), ExecutionMode::Quiet)?;

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
        let mut args = Arguments::new(" ");
        args.push(Argument::new(
            DelimiterString::new("uninstall", None, None),
            None,
            None,
            None,
        ));

        self.apply_scope_arg(&mut args);
        args.push(Argument::flag("y", "-"));
        args.push(Argument::new(
            DelimiterString::new(app_id, None, None),
            None,
            None,
            None,
        ));

        CommandExecutor::execute("flatpak", &args, self.privilege(), ExecutionMode::Quiet)
    }
}