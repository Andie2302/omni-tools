/*
use command::{Argument, ArgumentBuilder, Arguments};

pub struct FlatpakInstall<'a> {
    app_id: &'a str,
    arguments: Arguments<'a>
}

impl<'a> FlatpakInstall<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            arguments: Default::default(),
        }
    }

    // --- Flag-Methoden (ohne Parameter) ---

    pub fn help(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("help").build().unwrap());
        self
    }

    pub fn user(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("user").build().unwrap());
        self
    }

    pub fn system(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("system").build().unwrap());
        self
    }

    pub fn no_pull(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("no-pull").build().unwrap());
        self
    }

    pub fn no_deploy(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("no-deploy").build().unwrap());
        self
    }

    pub fn no_related(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("no-related").build().unwrap());
        self
    }

    pub fn no_deps(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("no-deps").build().unwrap());
        self
    }

    pub fn no_auto_pin(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("no-auto-pin").build().unwrap());
        self
    }

    pub fn no_static_deltas(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("no-static-deltas").build().unwrap());
        self
    }

    pub fn runtime(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("runtime").build().unwrap());
        self
    }

    pub fn app(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("app").build().unwrap());
        self
    }

    pub fn include_sdk(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("include-sdk").build().unwrap());
        self
    }

    pub fn include_debug(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("include-debug").build().unwrap());
        self
    }

    pub fn bundle(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("bundle").build().unwrap());
        self
    }

    pub fn from(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("from").build().unwrap());
        self
    }

    pub fn image(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("image").build().unwrap());
        self
    }

    pub fn assumeyes(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("assumeyes").build().unwrap());
        self
    }

    pub fn reinstall(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("reinstall").build().unwrap());
        self
    }

    pub fn noninteractive(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("noninteractive").build().unwrap());
        self
    }

    pub fn or_update(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("or-update").build().unwrap());
        self
    }

    pub fn verbose(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("verbose").build().unwrap());
        self
    }

    pub fn ostree_verbose(mut self) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("ostree-verbose").build().unwrap());
        self
    }

    // --- Parameter-Methoden (ohne Allokation via 2 Elements) ---

    pub fn installation(mut self, value: &'a str) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("installation").separator("=").value(value).build().unwrap());
        self
    }

    pub fn arch(mut self, value: &'a str) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("arch").separator("=").value(value).build().unwrap());
        self
    }

    pub fn gpg_file(mut self, value: &'a str) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("gpg-file").separator("=").value(value).build().unwrap());
        self
    }

    pub fn subpath(mut self, value: &'a str) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("subpath").separator("=").value(value).build().unwrap());
        self
    }

    pub fn sideload_repo(mut self, value: &'a str) -> Self {
        self.arguments.push(ArgumentBuilder::new().prefix("--").key("sideload-repo").separator("=").value(value).build().unwrap());
        self
    }

    // --- Finale Methode ---
    pub fn finish(mut self) -> Arguments<'a> {
        // 2. app_id als letztes Positional-Argument anhängen
        self.arguments.push(
            ArgumentBuilder::new()
                .value(self.app_id)
                .build()
                .unwrap_or_default()
        );

        self.arguments
    }
}
// */