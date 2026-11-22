import { api, type Overview } from "./api";
import { i18n } from "./i18n.svelte";

export const app = $state({
  ov: null as Overview | null,
  busy: false,
  toast: null as { text: string; kind: "error" | "ok" } | null,
});

let toastTimer: ReturnType<typeof setTimeout> | undefined;

export function toast(text: string, kind: "error" | "ok" = "error") {
  app.toast = { text, kind };
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => (app.toast = null), kind === "error" ? 6000 : 2500);
}

export async function refresh() {
  try {
    app.ov = await api.overview();
    i18n.lang = app.ov.language;
    document.documentElement.lang = app.ov.language;
  } catch (e) {
    toast(String(e));
  }
}

/** Выполнить действие с индикатором занятости, ошибкой в тост и обновлением состояния. */
export async function act<T>(fn: () => Promise<T>, okText?: string): Promise<T | undefined> {
  app.busy = true;
  try {
    const r = await fn();
    if (okText) toast(okText, "ok");
    return r;
  } catch (e) {
    toast(String(e));
  } finally {
    app.busy = false;
    await refresh();
  }
}
