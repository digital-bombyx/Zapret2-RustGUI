<script lang="ts">
  import { onMount } from "svelte";
  import { api, on, type Status } from "$lib/api";
  import { app, refresh, toast, act } from "$lib/state.svelte";
  import { t } from "$lib/i18n.svelte";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import Home from "$lib/components/Home.svelte";
  import Tests from "$lib/components/Tests.svelte";
  import Lists from "$lib/components/Lists.svelte";
  import Hosts from "$lib/components/Hosts.svelte";
  import Settings from "$lib/components/Settings.svelte";
  import { House, FlaskConical, ListChecks, Globe, Settings2, ShieldAlert } from "@lucide/svelte";

  type Tab = "home" | "tests" | "lists" | "hosts" | "settings";
  let tab = $state<Tab>("home");
  let dirInput = $state("");

  const tabs: { id: Tab; label: () => string; icon: typeof House }[] = [
    { id: "home", label: () => t("главная", "home"), icon: House },
    { id: "tests", label: () => t("тесты", "tests"), icon: FlaskConical },
    { id: "lists", label: () => t("списки", "lists"), icon: ListChecks },
    { id: "hosts", label: () => "hosts", icon: Globe },
    { id: "settings", label: () => t("настройки", "settings"), icon: Settings2 },
  ];
  const paths: Record<Tab, string> = {
    home: "",
    tests: "tests",
    lists: "lists",
    hosts: "hosts",
    settings: "settings",
  };

  onMount(() => {
    refresh();
    const un = [
      on<Status>("status", (s) => {
        if (app.ov) app.ov.status = s;
      }),
      on<string>("error", (e) => toast(e)),
      on("overview-changed", () => refresh()),
    ];
    // Окно снова показали из трея — подтянуть актуальное состояние.
    const vis = () => document.visibilityState === "visible" && refresh();
    document.addEventListener("visibilitychange", vis);
    return () => {
      un.forEach((p) => p.then((f) => f()));
      document.removeEventListener("visibilitychange", vis);
    };
  });
</script>

<div class="shell">
  <TitleBar path={paths[tab]} running={!!app.ov?.status.running} />

  {#if app.ov}
    {#if !app.ov.admin}
      <div class="banner mono"><ShieldAlert size={14} /> {t("нет прав администратора — только просмотр", "no administrator rights — view only")}</div>
    {/if}

    {#if !app.ov.dir}
      <main class="content">
        <section class="card setup fade-in">
          <div class="label">{t("первый запуск", "first run")}</div>
          <p class="muted">{t("Укажите папку со сборкой zapret-discord-youtube (в ней bin\\winws2.exe, lua\\ и lists\\).", "Set the zapret-discord-youtube build folder (it contains bin\\winws2.exe, lua\\ and lists\\).")}</p>
          <input class="input" placeholder="D:\zapret-discord-youtube" bind:value={dirInput} />
          <button class="btn primary" disabled={!dirInput.trim()} onclick={() => act(() => api.setZapretDir(dirInput))}
            >{t("сохранить", "save")}</button>
        </section>
      </main>
    {:else}
      <nav class="tabs">
        {#each tabs as tb (tb.id)}
          <button class:active={tab === tb.id} onclick={() => (tab = tb.id)} title={tb.label()}>
            <tb.icon size={15} />
            <span class="mono">{tb.label()}</span>
          </button>
        {/each}
      </nav>
      <main class="content">
        {#if tab === "home"}<Home />
        {:else if tab === "tests"}<Tests />
        {:else if tab === "lists"}<Lists />
        {:else if tab === "hosts"}<Hosts />
        {:else}<Settings />{/if}
      </main>
    {/if}
  {/if}

  {#if app.toast}
    <div class="toast mono {app.toast.kind}" role="status">{app.toast.text}</div>
  {/if}
</div>

<style>
  .shell {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: radial-gradient(140% 60% at 50% -10%, #232919 0%, var(--bg) 55%);
  }
  .banner {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 7px 14px;
    font-size: 11px;
    color: var(--yellow);
    background: rgba(217, 194, 94, 0.08);
    border-bottom: 1px solid rgba(217, 194, 94, 0.18);
  }
  .tabs {
    display: flex;
    gap: 4px;
    padding: 10px 14px 2px;
  }
  .tabs button {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 3px;
    padding: 7px 0 6px;
    border: 1px solid transparent;
    border-radius: 12px;
    background: transparent;
    color: var(--faint);
    cursor: pointer;
    transition: background 0.18s, color 0.18s, border-color 0.18s;
  }
  .tabs button span {
    font-size: 10px;
  }
  .tabs button:hover {
    color: var(--muted);
  }
  .tabs button.active {
    background: var(--panel);
    border-color: var(--border-soft);
    color: var(--amber-2);
  }
  .content {
    flex: 1;
    overflow-y: auto;
    padding: 10px 14px 16px;
    min-height: 0;
  }
  .setup {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-top: 20px;
  }
  .setup p {
    margin: 0;
    font-size: 12.5px;
  }
  .toast {
    position: fixed;
    left: 50%;
    bottom: 16px;
    transform: translateX(-50%);
    max-width: calc(100% - 40px);
    padding: 9px 14px;
    border-radius: 12px;
    font-size: 11.5px;
    background: var(--panel-3);
    border: 1px solid var(--border);
    box-shadow: 0 12px 30px -10px rgba(0, 0, 0, 0.8);
    animation: up 0.2s ease-out;
    white-space: pre-wrap;
    user-select: text;
  }
  .toast.error {
    color: #f0b3a4;
    border-color: rgba(208, 112, 92, 0.5);
  }
  .toast.ok {
    color: var(--sage);
  }
  @keyframes up {
    from {
      opacity: 0;
      transform: translate(-50%, 6px);
    }
  }
</style>
