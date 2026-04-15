use std::io;
use std::process::Command;

pub fn fetch_hyprctl_binds() -> io::Result<String> {
    let output = Command::new("hyprctl").arg("binds").output()?;

    if !output.status.success() {
        return Err(io::Error::other("hyprctl binds command failed"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
