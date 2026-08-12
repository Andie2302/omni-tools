// `ArgumentList` (aus argument_alloc.rs) existiert nur mit Feature "alloc" –
// ohne dieses Gate würde das Modul kompilieren, sobald "std" aktiv ist,
// aber "alloc" fehlt, und dann schlägt `use crate::ArgumentList;` unten fehl.
#![cfg(all(feature = "std", feature = "alloc"))]

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use crate::ArgumentList;

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
impl AsRef<OsStr> for ExecutablePath {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

impl AsRef<OsStr> for CurrentDirectoryPath {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command<'a>
{
    pub current_directory: CurrentDirectoryPath,
    pub executable_path: ExecutablePath,
    pub argument_list: ArgumentList<'a>,
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