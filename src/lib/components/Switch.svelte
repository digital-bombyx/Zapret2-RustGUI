<script lang="ts">
  let { checked, onchange, disabled = false }: { checked: boolean; onchange: (v: boolean) => void; disabled?: boolean } =
    $props();
</script>

<button class="sw" class:on={checked} {disabled} role="switch" aria-checked={checked} aria-label="toggle" onclick={() => onchange(!checked)}>
  <span></span>
</button>

<style>
  .sw {
    position: relative;
    width: 38px;
    height: 22px;
    flex: none;
    border-radius: 999px;
    border: 1px solid var(--border);
    background: var(--bg);
    cursor: pointer;
    transition: background 0.2s, border-color 0.2s;
  }
  .sw span {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: var(--faint);
    transition: transform 0.22s cubic-bezier(0.3, 1.4, 0.5, 1), background 0.2s;
  }
  .sw.on {
    background: var(--amber-soft);
    border-color: rgba(217, 162, 91, 0.55);
  }
  .sw.on span {
    transform: translateX(16px);
    background: var(--amber);
  }
  .sw:disabled {
    opacity: 0.45;
    cursor: default;
  }
</style>
