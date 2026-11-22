// Мок бэкенда для `npm run dev` в обычном браузере (без Tauri): только вёрстка.
const names = [
  "general (ALT)", ...Array.from({ length: 12 }, (_, i) => `general (ALT${i + 2})`), "general (EXP)",
  "general (FAKE TLS AUTO ALT)", "general (FAKE TLS AUTO ALT2)", "general (FAKE TLS AUTO ALT3)", "general (FAKE TLS AUTO)",
  "general (SIMPLE FAKE ALT)", "general (SIMPLE FAKE ALT2)", "general (SIMPLE FAKE)", "general",
];
const blocks = ["Domain Bypass Quic", "Discord Voice", "Discord Media", "Youtube", "Domain Bypass TLS", "IP Bypass TLS"];
const state: any = {
  admin: true,
  dir: "D:\\zapret-discord-youtube",
  version: "1.10.4",
  strategies: names.map((n, i) => ({ file: `${n}.bat`, name: n, blocks, methods: i % 2 ? ["fake", "multisplit"] : ["fake", "fakedsplit", "multisplit"] })),
  selected: "general (ALT).bat",
  status: { running: true, mode: "process", strategy: "general (ALT).bat", service_installed: false, last_error: null },
  game_filter: { mode: "disabled", tcp: "1024-65535", udp: "1024-65535" },
  ipset: "loaded",
  ipset_has_backup: false,
  mode: "process",
  autostart: false,
  close_to_tray: true,
  test_timeout: 4,
  test_parallel: 16,
  testing: false,
  dir_source: "bundled",
  bundled_version: "1.10.4 @ aa56e97",
  bundle_error: null,
  language: "ru",
};
const targets = ["DiscordMain", "DiscordGateway", "YouTubeWeb", "GoogleMain", "CloudflareWeb"];
const mkRes = (file: string, ok: number) => ({
  file,
  started: true,
  error: null,
  targets: targets.map((k, i) => ({
    key: k,
    kind: "url",
    checks: [["HTTP", i < ok ? "OK" : "ERROR"], ["TLS1.2", "OK"], ["TLS1.3", i % 2 ? "UNSUP" : "OK"]],
    ping_ms: 20 + i * 7,
    ping_tried: true,
    detail: null,
  })),
  ok: ok * 2 + 3,
  fail: 5 - ok,
  unsup: 2,
  blocked: 0,
  ping_ok: 5,
  ping_fail: 0,
});
const run = {
  id: 1795340000,
  kind: "standard",
  total: 3,
  index: 3,
  current: null,
  results: [mkRes("general.bat", 2), mkRes("general (ALT).bat", 5), { ...mkRes("general (ALT2).bat", 0), started: false, error: "winws2 сразу завершился", targets: [] }],
  best: "general (ALT).bat",
  finished: true,
  cancelled: false,
  message: null,
};

export async function mockInvoke(cmd: string, args: any = {}): Promise<any> {
  await new Promise((r) => setTimeout(r, 120));
  switch (cmd) {
    case "overview":
      return structuredClone(state);
    case "enable":
      state.status = { ...state.status, running: true, mode: state.mode, strategy: state.selected };
      return;
    case "disable":
      state.status = { ...state.status, running: false, mode: "off", strategy: null };
      return;
    case "select_strategy":
      state.selected = args.file;
      if (state.status.running) state.status.strategy = args.file;
      return;
    case "set_mode":
      state.mode = args.mode;
      return;
    case "set_game_filter":
      state.game_filter = args.gf;
      return;
    case "set_ipset":
      state.ipset = args.mode;
      return;
    case "list_read":
      return "# Never leave this file empty\ndomain.example.abc\n";
    case "list_write":
      return { warnings: [] };
    case "test_last":
      return run;
    case "test_history":
      return [run];
    case "log_tail":
      return ["winws2 started", "lua init ok"];
    case "hosts_status":
      return {
        source: state.language === "en" ? "build (.service\\hosts)" : "сборка (.service\\hosts)",
        // Группы и размеры как в настоящем .service\hosts.
        groups: ([["discord.com", 8], ["discordapp.net", 8], ["github.com", 1], ["githubusercontent.com", 36], ["t.me", 1], ["telegram.dog", 1], ["telegram.me", 1], ["telegram.org", 21], ["telegram.space", 1], ["telesco.pe", 8], ["tg.dev", 2]] as [string, number][]).map(
          ([g, n]) => ({ name: g, entries: Array.from({ length: n }, (_, i) => ({ ip: "140.82.121.3", host: i ? `h${i}.${g}` : g, in_block: true, manual: false })) }),
        ),
        custom: [],
        block_present: true,
        outdated: false,
        hosts_path: "C:\\Windows\\System32\\drivers\\etc\\hosts",
      };
    case "set_autostart":
      state.autostart = args.enabled;
      return args.enabled;
    case "set_language":
      state.language = args.lang;
      return;
    case "set_close_to_tray":
      state.close_to_tray = args.enabled;
      return;
    default:
      return null;
  }
}
