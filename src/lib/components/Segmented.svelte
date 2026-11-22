<script lang="ts" generics="T extends string">
  type Props = {
    options: { value: T; label: string }[];
    value: T;
    onchange: (v: T) => void;
    disabled?: boolean;
    small?: boolean;
  };
  let { options, value, onchange, disabled = false, small = false }: Props = $props();
</script>

<div class="seg" class:small class:disabled>
  {#each options as o (o.value)}
    <button
      class:active={o.value === value}
      {disabled}
      onclick={() => o.value !== value && onchange(o.value)}>{o.label}</button>
  {/each}
</div>

<style>
  .seg {
    display: flex;
    gap: 2px;
    padding: 3px;
    border-radius: 999px;
    background: var(--bg);
    border: 1px solid var(--border-soft);
  }
  .seg button {
    flex: 1;
    height: 26px;
    padding: 0 10px;
    border: 0;
    border-radius: 999px;
    background: transparent;
    color: var(--muted);
    font-family: var(--mono);
    font-size: 11.5px;
    cursor: pointer;
    transition: background 0.18s, color 0.18s, box-shadow 0.18s;
    white-space: nowrap;
  }
  .seg button:hover:not(:disabled):not(.active) {
    color: var(--text);
  }
  .seg button.active {
    background: var(--panel-3);
    color: var(--text);
    box-shadow: 0 1px 0 rgba(255, 245, 220, 0.06) inset, 0 2px 8px -2px rgba(0, 0, 0, 0.5);
  }
  .small button {
    height: 22px;
    font-size: 10.5px;
    padding: 0 8px;
  }
  .disabled {
    opacity: 0.5;
  }
</style>
