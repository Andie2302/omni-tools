use crate::arguments::Arguments;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel {
    User,
    Sudo,
    Pkexec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    Interactive,
    Quiet,
}

pub struct CommandExecutor;

impl CommandExecutor {
    pub fn execute(
        program: &str,
        args: &Arguments,
        privilege: PrivilegeLevel,
        mode: ExecutionMode,
    ) -> Result<String, String> {
        let raw_args = args.to_arg_strings();
        let (actual_program, final_args) = Self::build_command_array(program, &raw_args, privilege);

        let mut cmd = Command::new(&actual_program);
        cmd.args(&final_args);

        match mode {
            ExecutionMode::Interactive => {
                cmd.stdin(Stdio::inherit());
                cmd.stdout(Stdio::inherit());
                cmd.stderr(Stdio::inherit());

                let mut child = cmd
                    .spawn()
                    .map_err(|e| format!("Fehler beim Starten von '{actual_program}': {e}"))?;

                let status = child
                    .wait()
                    .map_err(|e| format!("Fehler beim Warten auf Prozess: {e}"))?;

                if status.success() {
                    Ok(String::new())
                } else {
                    Err(format!(
                        "Befehl '{actual_program}' schlug fehl mit Exit-Code: {:?}",
                        status.code()
                    ))
                }
            }
            ExecutionMode::Quiet => {
                let output = cmd
                    .output()
                    .map_err(|e| format!("Fehler beim Ausführen von '{actual_program}': {e}"))?;

                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
                } else {
                    let err = String::from_utf8_lossy(&output.stderr);
                    Err(format!("Befehl '{actual_program}' fehlgeschlagen: {err}"))
                }
            }
        }
    }

    fn build_command_array<'a>(
        program: &'a str,
        args: &'a [String],
        privilege: PrivilegeLevel,
    ) -> (String, Vec<String>) {
        match privilege {
            PrivilegeLevel::User => (program.to_string(), args.to_vec()),
            PrivilegeLevel::Sudo => {
                let mut sudo_args = vec![program.to_string()];
                sudo_args.extend_from_slice(args);
                ("sudo".to_string(), sudo_args)
            }
            PrivilegeLevel::Pkexec => {
                let mut pkexec_args = vec![program.to_string()];
                pkexec_args.extend_from_slice(args);
                ("pkexec".to_string(), pkexec_args)
            }
        }
    }
}