use std::process::Command;

pub fn set_wallpaper_macos(path: &str) -> Result<(), String> {
  let script = format!(
    "tell application \"System Events\" to set picture of every desktop to \"{}\"",
    path.display()
  );

  let output = Command::new("osascript")
    .arg("-e")
    .arg(script)
    .status()?;

  if output.success() {
    Ok(())
  } else {
    Err("Failed to set wallpaper")
  }
}