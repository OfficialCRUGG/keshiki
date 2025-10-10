use std::process::Command;

pub fn set(path: &str) -> Result<(), String> {
  let script = format!(
    "tell application \"System Events\" to set picture of every desktop to \"{}\"",
    path
  );

  let output = Command::new("osascript")
    .arg("-e")
    .arg(script)
    .status()
    .map_err(|err| err.to_string())?;

  if output.success() {
    Ok(())
  } else {
    Err(String::from("Failed to set wallpaper"))
  }
}