//! Qt plugin system.

use std::io;
use std::{env, path::Path};

/// Sets the Qt plugin search path.
///
/// This must be called before calling other Qt functions to have any effect.
/// Specifically it must be called before `QCoreApplication` initialization
/// takes place.
pub fn set_search_path<I, P>(paths: I) -> Result<(), env::JoinPathsError>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let joined = env::join_paths(paths.into_iter().map(|p| p.as_ref().to_owned()))?;
    env::set_var("QT_PLUGIN_PATH", joined);
    Ok(())
}

/// Configures the Qt plugin search path for a deployment of
/// Qt in the directory of the application.
///
/// Such a deployment can be created using the `windeployqt.exe` tool.
///
/// This must be called before calling other Qt functions to have any effect.
/// Specifically it must be called before `QCoreApplication` initialization
/// takes place.
pub fn configure_local_deployment() -> io::Result<()> {
    let exe = env::current_exe()?;
    let exe_dir = exe
        .parent()
        .ok_or(io::Error::new(io::ErrorKind::Other, "cannot get executable directory"))?;
    set_search_path(&[exe_dir])
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
    Ok(())
}

/// Disable installation of Qt QPA signal handlers.
///
/// This must be called before calling other Qt functions to have any effect.
/// Specifically it must be called before `QCoreApplication` initialization
/// takes place.
pub fn disable_qpa_signal_handlers() {
    env::set_var("QT_QPA_NO_SIGNAL_HANDLER", "1");
}
