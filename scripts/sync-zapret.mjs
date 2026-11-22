// Копирует чистую сборку zapret-discord-youtube (из git, без локальных правок
// пользователя) в src-tauri/zapret — оттуда она попадает в установщик как ресурс.
//
// Источник: переменная ZAPRET_SRC или D:\zapret-discord-youtube, ревизия ZAPRET_REF или HEAD.
// Если источника нет, но src-tauri/zapret уже собран — оставляем как есть.
import { execSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const dest = join(root, "src-tauri", "zapret");
const src = process.env.ZAPRET_SRC || "D:\\zapret-discord-youtube";
const ref = process.env.ZAPRET_REF || "HEAD";

if (!existsSync(join(src, ".git"))) {
  if (existsSync(join(dest, "bundle.json"))) {
    console.log(`[sync-zapret] ${src} не найден, оставляю уже собранный ${dest}`);
    process.exit(0);
  }
  console.error(`[sync-zapret] нет git-репозитория zapret в ${src} (задайте ZAPRET_SRC)`);
  process.exit(1);
}

const git = (args) => execSync(`git -c safe.directory=* -C "${src}" ${args}`, { encoding: "utf8" }).trim();
const commit = git(`rev-parse --short ${ref}`);
const tmp = mkdtempSync(join(tmpdir(), "zapret-sync-"));
try {
  // git archive → tar.exe (есть в Windows 10+): берём файлы ровно как в коммите.
  execSync(`git -c safe.directory=* -C "${src}" archive --format=tar ${ref} | tar -x -C "${tmp}"`, {
    stdio: ["ignore", "ignore", "inherit"],
    shell: "cmd.exe",
  });

  rmSync(dest, { recursive: true, force: true });
  mkdirSync(dest, { recursive: true });

  for (const d of ["bin", "lua", "lists"]) cpSync(join(tmp, d), join(dest, d), { recursive: true });
  // Пользовательские файлы не поставляем: они создаются на месте и не должны затираться.
  for (const f of readdirSync(join(dest, "lists"))) {
    if (f.endsWith("-user.txt")) rmSync(join(dest, "lists", f));
  }
  mkdirSync(join(dest, "utils"), { recursive: true });
  cpSync(join(tmp, "utils", "targets.txt"), join(dest, "utils", "targets.txt"));
  // Папки с точкой ресурсы Tauri могут пропустить — кладём .service как _service.
  mkdirSync(join(dest, "_service"), { recursive: true });
  for (const f of ["hosts", "version.txt"]) cpSync(join(tmp, ".service", f), join(dest, "_service", f));
  for (const f of readdirSync(tmp)) {
    if (/^general.*\.bat$/i.test(f) || /^license/i.test(f)) cpSync(join(tmp, f), join(dest, f));
  }

  const version = readFileSync(join(tmp, ".service", "version.txt"), "utf8").trim();
  const bundle = { version, commit, ref, source: "Flowseal/zapret-discord-youtube" };
  writeFileSync(join(dest, "bundle.json"), JSON.stringify(bundle, null, 2));
  const n = readdirSync(dest).filter((f) => f.endsWith(".bat")).length;
  console.log(`[sync-zapret] zapret ${version} @ ${commit}: ${n} стратегий → ${dest}`);
} finally {
  rmSync(tmp, { recursive: true, force: true });
}
