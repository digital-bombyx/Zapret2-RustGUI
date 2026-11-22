<script lang="ts">
  import { api } from "$lib/api";
  import { app, act } from "$lib/state.svelte";
  import Segmented from "./Segmented.svelte";
  import { t } from "$lib/i18n.svelte";
  import { Save, RotateCcw } from "@lucide/svelte";

  type Kind = "general" | "exclude" | "ipset-exclude";
  const info: Record<Kind, { title: string; hint: () => string }> = {
    general: { title: "list-general-user.txt", hint: () => t("домены, которые нужно обходить — по одному в строке", "domains to bypass — one per line") },
    exclude: { title: "list-exclude-user.txt", hint: () => t("домены, которые обходить НЕ нужно", "domains NOT to bypass") },
    "ipset-exclude": { title: "ipset-exclude-user.txt", hint: () => t("IP и подсети (CIDR), исключённые из обхода", "IPs and subnets (CIDR) excluded from bypass") },
  };

  let kind = $state<Kind>("general");
  let text = $state("");
  let saved = $state("");
  let warnings = $state<string[]>([]);
  const dirty = $derived(text !== saved);

  async function load(k: Kind) {
    warnings = [];
    try {
      text = saved = await api.listRead(k);
    } catch (e) {
      text = saved = "";
    }
  }
  $effect(() => {
    load(kind);
  });

  async function save() {
    const r = await act(() => api.listWrite(kind, text), t("Сохранено", "Saved"));
    if (r) {
      warnings = r.warnings;
      await load(kind);
      warnings = r.warnings;
    }
  }
  const lines = $derived(text.split("\n").filter((l) => l.trim() && !l.trim().startsWith("#")).length);
</script>

<div class="lists fade-in">
  <Segmented
    value={kind}
    onchange={(v) => {
      if (!dirty || confirm(t("Есть несохранённые изменения. Переключиться?", "You have unsaved changes. Switch anyway?"))) kind = v;
    }}
    options={[
      { value: "general", label: t("обходить", "bypass") },
      { value: "exclude", label: t("исключить", "exclude") },
      { value: "ipset-exclude", label: t("ip-исключения", "ip exclude") },
    ]} />

  <section class="card ed">
    <div class="head">
      <div>
        <div class="label">{info[kind].title}</div>
        <div class="hint faint mono">{info[kind].hint()}</div>
      </div>
      <span class="chip">{lines} {t("строк", "lines")}</span>
    </div>
    <textarea class="textarea" spellcheck="false" bind:value={text}></textarea>
    {#if warnings.length}
      <div class="warn mono">
        {t("не похоже на", "doesn't look like")} {kind === "ipset-exclude" ? "IP/CIDR" : t("домен", "a domain")}: {warnings.slice(0, 6).join(", ")}{warnings.length > 6
          ? ` ${t("и ещё", "and")} ${warnings.length - 6}`
          : ""}
      </div>
    {/if}
    <div class="actions">
      <span class="faint mono note">{t("пустой файл заменится заглушкой; работающий обход перезапустится", "an empty file gets a placeholder; a running bypass restarts")}</span>
      <button class="btn ghost sm" disabled={!dirty} onclick={() => (text = saved)}><RotateCcw size={12} /></button>
      <button class="btn primary" disabled={!dirty || app.busy || app.ov?.testing} onclick={save}
        ><Save size={13} /> {t("сохранить", "save")}</button>
    </div>
  </section>
</div>

<style>
  .lists {
    display: flex;
    flex-direction: column;
    gap: 12px;
    height: 100%;
  }
  .ed {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 12px 14px;
    min-height: 0;
  }
  .head {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }
  .hint {
    font-size: 10.5px;
    margin-top: 3px;
  }
  textarea {
    flex: 1;
    min-height: 300px;
  }
  .warn {
    font-size: 10.5px;
    color: var(--yellow);
  }
  .actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .note {
    flex: 1;
    font-size: 10px;
  }
</style>
