import { path, fs, invoke } from "@tauri-apps/api";
import type { FileEntry } from "@tauri-apps/api/fs";

export async function ensureAppdir() {
  const homeDir = await path.homeDir();
  const appDir = await path.join(homeDir, ".keshiki");

  if (await fs.exists(appDir)) {
    return appDir;
  } else {
    await fs.createDir(appDir);
    return appDir;
  }
}

async function getWallpaperDirectories() {
  const appDir = await ensureAppdir();
  const listFile = await path.join(appDir, "directories");
  if (await fs.exists(listFile)) {
    const list = await fs.readTextFile(listFile);
    return list.split("\n");
  } else {
    await fs.writeTextFile(listFile, "");
    return [];
  }
}

let supportedExtensions = ["jpg", "jpeg", "png"];

export async function fetchWallpapers() {
  const wallpaperDirs = await getWallpaperDirectories();
  let wallpapers: string[] = [];

  await Promise.all(
    wallpaperDirs.map(async (dir) => {
      await invoke("expand_scope", { folderPath: dir });
      const files = await fs.readDir(dir, {
        recursive: true,
      });
      flattenFiletree(files).forEach((file) => {
        console.log(file);
        if (supportedExtensions.includes(file.name?.split(".").pop() || "")) {
          wallpapers.push(file.path);
        }
      });
    }),
  );

  return wallpapers;
}

function flattenFiletree(filetree: FileEntry[]): FileEntry[] {
  let files: FileEntry[] = [];
  filetree.forEach((file) => {
    if (file.children) {
      files = files.concat(flattenFiletree(file.children));
    } else {
      files.push(file);
    }
  });
  return files;
}
