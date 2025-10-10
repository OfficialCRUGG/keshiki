use std::process::Command;

pub fn set(path: &str) -> Result<(), String> {
  let script = format!(
    r#"Add-Type -TypeDefinition @"
using System.Runtime.InteropServices;
public class Wallpaper {{
    [DllImport("user32.dll", SetLastError = true)]
    public static extern bool SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);
}}
"@;
[Wallpaper]::SystemParametersInfo(20, 0, "{}" , 3)"#,
    path
  );

  let result = Command::new("powershell")
    .arg("-Command")
    .arg(script)
    .status()
    .map_err(|err| err.to_string())?;

  if result.success() {
    Ok(())
  } else {
    Err("Failed to set wallpaper".into())
  }
}
