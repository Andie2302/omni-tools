
use crate::commands::command::Command;

#[cfg(feature = "std")]
impl<'a> Command<'a> {
    pub fn exec_read(&self) -> std::io::Result<String> {
        let mut cmd: std::process::Command = self.into();
        let output = cmd.output()?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    pub fn exec_status(&self) -> std::io::Result<bool> {
        let mut cmd: std::process::Command = self.into();
        let status = cmd.status()?;
        Ok(status.success())
    }

    pub fn exec_stream_logs(&self) -> std::io::Result<bool> {
        let mut cmd: std::process::Command = self.into();
        let status = cmd
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()?;
        Ok(status.success())
    }
}