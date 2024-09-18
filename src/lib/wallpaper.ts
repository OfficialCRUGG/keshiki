import * as os from "@tauri-apps/api/os";
import * as dialog from "@tauri-apps/api/dialog";
import { Command } from "@tauri-apps/api/shell";

export async function setWallpaper(filePath: string) {
  const operatingSystem = await os.type();

  if (operatingSystem === "Darwin") {
    await runCommand(
      `osascript -e "tell application \\"System Events\\" to tell every desktop to set picture to \\"${filePath}\\" as POSIX file"`,
    );
  } else if (operatingSystem === "Windows_NT") {
    dialog.message("Windows is not supported yet", {
      okLabel: "Okay",
      title: "Error",
      type: "error",
    });
  } else {
    dialog.message("Linux is not supported yet", {
      okLabel: "Okay",
      title: "Error",
      type: "error",
    });
  }
}

async function runCommand(cmd: string) {
  const command = new Command("sh", ["-c", cmd]);
  await command.spawn();
  command.stdout.on("data", (data) => {
    console.log(data.toString());
  });
  command.stderr.on("data", (data) => {
    console.log(data.toString());
  });
}
