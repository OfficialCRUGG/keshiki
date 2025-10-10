use std::process::Command;

pub fn set(path: &str) -> Result<(), String> {
  let script = format!(
    "tell application \"System Events\" to set picture of every desktop to \"{}\"",
    path
  );

  let result = Command::new("osascript")
    .arg("-e")
    .arg(script)
    .status()
    .map_err(|err| err.to_string())?;

  if result.success() {
    Ok(())
  } else {
    Err("Failed to set wallpaper".into())
  }
}