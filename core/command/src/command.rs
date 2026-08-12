#![cfg(feature = "std")] 

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrentDirectoryPath(pub PathBuf);
impl AsRef<Path> for CurrentDirectoryPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExecutablePath(pub PathBuf);
impl AsRef<Path> for ExecutablePath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command<'a>
{
    current_directory: CurrentDirectoryPath,
    executable_path: ExecutablePath,
    argument_list: ArgumentList<'a>,
}

impl<'a> Command<'a> {
    pub fn new(
        current_directory: CurrentDirectoryPath,
        executable_path: ExecutablePath,
        argument_list: ArgumentList<'a>,
    ) -> Self {
        Self {
            current_directory,
            executable_path,
            argument_list,
        }
    }
    
}


