<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { X, Minus, Maximize2 } from "@lucide/svelte";
  import { isTauri } from "$lib/api";
  import { t } from "$lib/i18n.svelte";

  let { path, running }: { path: string; running: boolean } = $props();
  const noop = async () => {};
  const win = isTauri ? getCurrentWindow() : { close: noop, minimize: noop, toggleMaximize: noop };
</script>

<header class="bar" data-tauri-drag-region>
  <div class="state mono" class:on={running} data-tauri-drag-region>
    <span class="dot"></span>{running ? "spinning" : "idle"}
  </div>
  <div class="title mono" data-tauri-drag-region>digital-bombyx/Zapret2-RustGUI: ~/{path}</div>
  <div class="lights">
    <button class="l min" title={t("Свернуть", "Minimize")} onclick={() => win.minimize()}
      ><span class="v"><Minus size={8} strokeWidth={3.5} /></span></button>
    <button class="l max" title={t("Развернуть", "Maximize")} onclick={() => win.toggleMaximize()}
      ><span class="v"><Maximize2 size={7} strokeWidth={3.5} /></span></button>
    <button class="l close" title={t("Закрыть", "Close")} onclick={() => win.close()}
      ><span class="v"><X size={8} strokeWidth={3.5} /></span></button>
  </div>
</header>

<style>
  .bar {
    height: 40px;
    flex: none;
    display: grid;
    grid-template-columns: auto 1fr auto;
    column-gap: 8px;
    align-items: center;
    padding: 0 14px;
    border-bottom: 1px solid var(--border-soft);
    background: var(--bg-2);
  }
  /* Хитбоксы как у кнопок Windows: прямоугольник на всю высоту шапки,
     видимая часть — прежний кружок по центру. */
  .lights {
    display: flex;
    justify-self: end;
    align-self: stretch;
    margin-right: -14px;
  }
  .l {
    width: 34px;
    height: 100%;
    border: 0;
    padding: 0;
    display: grid;
    place-items: center;
    background: transparent;
    cursor: pointer;
    transition: background 0.15s;
  }
  .l:hover {
    background: rgba(243, 227, 195, 0.06);
  }
  .l.close:hover {
    background: rgba(217, 162, 91, 0.14);
  }
  .v {
    width: 13px;
    height: 13px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    color: transparent;
    transition: filter 0.15s, transform 0.15s;
  }
  .lights:hover .v {
    color: rgba(22, 26, 19, 0.75);
  }
  .l:hover .v {
    filter: brightness(1.12);
    transform: scale(1.08);
  }
  .close .v {
    background: #d9a25b;
  }
  .min .v {
    background: #f3e3c3;
  }
  .max .v {
    background: #9db07f;
  }
  .title {
    text-align: center;
    font-size: 11px;
    color: var(--muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    pointer-events: none;
  }
  .state {
    justify-self: start;
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: var(--faint);
    pointer-events: none;
  }
  .dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--faint);
  }
  .state.on {
    color: var(--sage);
  }
  .state.on .dot {
    background: var(--sage);
    box-shadow: 0 0 0 3px var(--sage-soft);
    animation: pulse 2s ease-in-out infinite;
  }
  @keyframes pulse {
    50% {
      box-shadow: 0 0 0 5px rgba(157, 176, 127, 0.05);
    }
  }
</style>
