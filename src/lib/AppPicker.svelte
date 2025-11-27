<!--
  AppPicker — shared modal for selecting applications from the running process
  list or by browsing for an .exe file on disk.
  Author: Dmitry Osin <d@osin.pro>
-->
<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { api } from '$lib/api';
  import { notify } from '$lib/stores';

  interface Props {
    /** Called with the names/paths the user chose. */
    onAdd:   (names: string[]) => void;
    onClose: () => void;
  }

  let { onAdd, onClose }: Props = $props();

  let processes = $state<string[]>([]);
  let search    = $state('');
  let selected  = $state(new Set<string>());
  let loading   = $state(true);

  $effect(() => {
    api.listProcesses()
      .then(list => { processes = list; loading = false; })
      .catch(e  => { notify('error', String(e)); loading = false; });
  });

  const filtered = $derived(
    search.trim()
      ? processes.filter(p => p.toLowerCase().includes(search.trim().toLowerCase()))
      : processes
  );

  function toggle(name: string) {
    const next = new Set(selected);
    if (next.has(name)) next.delete(name); else next.add(name);
    selected = next;
  }

  async function browseExe() {
    const result = await open({
      multiple: false,
      filters:  [{ name: 'Executable', extensions: ['exe'] }],
      title:    'Select Executable',
    });
    if (result) {
      onAdd([result as string]);
      onClose();
    }
  }

  function addSelected() {
    if (selected.size === 0) return;
    onAdd([...selected]);
    onClose();
  }

  function handleOverlayKey(e: KeyboardEvent) {
    if (e.key === 'Escape') onClose();
  }

  function toggleAll() {
    if (filtered.every(p => selected.has(p))) {
      const next = new Set(selected);
      filtered.forEach(p => next.delete(p));
      selected = next;
    } else {
      const next = new Set(selected);
      filtered.forEach(p => next.add(p));
      selected = next;
    }
  }

  const allVisibleSelected = $derived(filtered.length > 0 && filtered.every(p => selected.has(p)));
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  class="overlay"
  role="dialog"
  aria-modal="true"
  aria-label="Application Picker"
  tabindex="-1"
  onkeydown={handleOverlayKey}
>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal picker-modal" onclick={e => e.stopPropagation()}>

    <div class="modal-header">
      <span class="modal-title">Select Applications</span>
      <button class="btn-icon" onclick={onClose} title="Close">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
      </button>
    </div>

    <div class="picker-browse-row">
      <button class="btn btn-secondary" style="width:100%;" onclick={browseExe}>
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
        </svg>
        Browse for .exe file…
      </button>
    </div>

    <div class="picker-divider">
      <span>or pick from running processes</span>
    </div>

    <div class="picker-search-wrap">
      <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="picker-search-icon">
        <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
      <input
        class="picker-search"
        placeholder="Search processes…"
        bind:value={search}
        autocomplete="off"
        spellcheck="false"
      />
      {#if search}
        <button class="picker-search-clear" onclick={() => search = ''} title="Clear">
          <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      {/if}
    </div>

    {#if filtered.length > 0}
      <div class="picker-select-all">
        <label class="picker-check-row" style="cursor:pointer;">
          <input type="checkbox" checked={allVisibleSelected} onchange={toggleAll} />
          <span style="font-size:11.5px; color:var(--text-muted);">
            {allVisibleSelected ? 'Deselect all' : 'Select all'} ({filtered.length})
          </span>
        </label>
        {#if selected.size > 0}
          <span class="badge badge-accent">{selected.size} selected</span>
        {/if}
      </div>
    {/if}

    <div class="picker-list">
      {#if loading}
        <div class="picker-loading">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="spin">
            <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
          </svg>
          Loading processes…
        </div>
      {:else if filtered.length === 0}
        <div class="picker-loading" style="color:var(--text-subtle);">No matching processes</div>
      {:else}
        {#each filtered as name}
          <label class="picker-item" class:selected={selected.has(name)}>
            <input
              type="checkbox"
              checked={selected.has(name)}
              onchange={() => toggle(name)}
            />
            <span class="picker-item-name">{name}</span>
          </label>
        {/each}
      {/if}
    </div>

    <div class="modal-footer">
      <button class="btn btn-secondary" onclick={onClose}>Cancel</button>
      <button
        class="btn btn-primary"
        disabled={selected.size === 0}
        onclick={addSelected}
      >
        Add Selected ({selected.size})
      </button>
    </div>
  </div>
</div>
