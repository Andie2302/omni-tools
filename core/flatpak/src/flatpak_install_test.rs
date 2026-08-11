pub struct FlatpakInstall<'a> {
    app_id: &'a str,
    raw_args: Vec<&'a str>,
}

impl<'a> FlatpakInstall<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            raw_args: vec!["install"],
        }
    }

    // --- Flag-Methoden (ohne Parameter) ---

    pub fn help(mut self) -> Self {
        self.raw_args.push("--help");
        self
    }

    pub fn user(mut self) -> Self {
        self.raw_args.push("--user");
        self
    }

    pub fn system(mut self) -> Self {
        self.raw_args.push("--system");
        self
    }

    pub fn no_pull(mut self) -> Self {
        self.raw_args.push("--no-pull");
        self
    }

    pub fn no_deploy(mut self) -> Self {
        self.raw_args.push("--no-deploy");
        self
    }

    pub fn no_related(mut self) -> Self {
        self.raw_args.push("--no-related");
        self
    }

    pub fn no_deps(mut self) -> Self {
        self.raw_args.push("--no-deps");
        self
    }

    pub fn no_auto_pin(mut self) -> Self {
        self.raw_args.push("--no-auto-pin");
        self
    }

    pub fn no_static_deltas(mut self) -> Self {
        self.raw_args.push("--no-static-deltas");
        self
    }

    pub fn runtime(mut self) -> Self {
        self.raw_args.push("--runtime");
        self
    }

    pub fn app(mut self) -> Self {
        self.raw_args.push("--app");
        self
    }

    pub fn include_sdk(mut self) -> Self {
        self.raw_args.push("--include-sdk");
        self
    }

    pub fn include_debug(mut self) -> Self {
        self.raw_args.push("--include-debug");
        self
    }

    pub fn bundle(mut self) -> Self {
        self.raw_args.push("--bundle");
        self
    }

    pub fn from(mut self) -> Self {
        self.raw_args.push("--from");
        self
    }

    pub fn image(mut self) -> Self {
        self.raw_args.push("--image");
        self
    }

    pub fn assumeyes(mut self) -> Self {
        self.raw_args.push("--assumeyes");
        self
    }

    pub fn reinstall(mut self) -> Self {
        self.raw_args.push("--reinstall");
        self
    }

    pub fn noninteractive(mut self) -> Self {
        self.raw_args.push("--noninteractive");
        self
    }

    pub fn or_update(mut self) -> Self {
        self.raw_args.push("--or-update");
        self
    }

    pub fn verbose(mut self) -> Self {
        self.raw_args.push("--verbose");
        self
    }

    pub fn ostree_verbose(mut self) -> Self {
        self.raw_args.push("--ostree-verbose");
        self
    }

    // --- Parameter-Methoden (ohne Allokation via 2 Elements) ---

    pub fn installation(mut self, value: &'a str) -> Self {
        self.raw_args.push("--installation");
        self.raw_args.push(value);
        self
    }

    pub fn arch(mut self, value: &'a str) -> Self {
        self.raw_args.push("--arch");
        self.raw_args.push(value);
        self
    }

    pub fn gpg_file(mut self, value: &'a str) -> Self {
        self.raw_args.push("--gpg-file");
        self.raw_args.push(value);
        self
    }

    pub fn subpath(mut self, value: &'a str) -> Self {
        self.raw_args.push("--subpath");
        self.raw_args.push(value);
        self
    }

    pub fn sideload_repo(mut self, value: &'a str) -> Self {
        self.raw_args.push("--sideload-repo");
        self.raw_args.push(value);
        self
    }

    // --- Finale Methode ---

    pub fn finish(mut self) -> Vec<&'a str> {
        self.raw_args.push(self.app_id);
        self.raw_args
    }
}