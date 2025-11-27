<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import AppPicker from '$lib/AppPicker.svelte';
  import { api } from '$lib/api';
  import { appSettings, config, notify } from '$lib/stores';

  let newEntry = $state('');
  let saving   = $state(false);

  async function addExclusion() {
    const val = newEntry.trim();
    if (!val) return;
    if ($config!.excludes.includes(val)) {
      notify('info', 'Entry already exists');
      return;
    }
    saving = true;
    try {
      const cfg = $config!;
      cfg.excludes = [...cfg.excludes, val];
      config.set(cfg);
      await api.saveProxifyrConfig($appSettings.configPath, cfg);
      notify('success', 'Exclusion added');
      newEntry = '';
    } catch (e) {
      notify('error', String(e));
    } finally {
      saving = false;
    }
  }

  async function removeExclusion(idx: number) {
    try {
      const cfg = $config!;
      cfg.excludes = cfg.excludes.filter((_, i) => i !== idx);
      config.set(cfg);
      await api.saveProxifyrConfig($appSettings.configPath, cfg);
      notify('success', 'Exclusion removed');
    } catch (e) {
      notify('error', String(e));
    }
  }

  function handleKey(e: KeyboardEvent) {
    if (e.key === 'Enter') addExclusion();
  }

  const noConfig = $derived(!$config || !$appSettings.configPath);

  let showPicker = $state(false);

  async function addFromPicker(names: string[]) {
    if (!$config || !$appSettings.configPath) return;
    const toAdd = names.filter(n => !$config!.excludes.includes(n));
    if (toAdd.length === 0) { notify('info', 'All selected entries already exist'); return; }
    saving = true;
    try {
      const cfg = $config!;
      cfg.excludes = [...cfg.excludes, ...toAdd];
      config.set(cfg);
      await api.saveProxifyrConfig($appSettings.configPath, cfg);
      notify('success', `Added ${toAdd.length} exclusion${toAdd.length !== 1 ? 's' : ''}`);
    } catch (e) {
      notify('error', String(e));
    } finally {
      saving = false;
    }
  }

  async function browseExeForExclude() {
    const result = await open({
      multiple: false,
      filters: [{ name: 'Executable', extensions: ['exe'] }],
      title: 'Select Executable',
    });
    if (result) {
      newEntry = result as string;
      await addExclusion();
    }
  }
</script>

<div class="page-header">
  <div class="page-title">Exclusions</div>
  <div class="page-subtitle">Applications that bypass all proxy rules</div>
</div>

<div class="page-body">
  <div class="card" style="margin-bottom:16px;">
    <div class="card-title">Add Exclusion</div>
    <div class="input-row">
      <input
        class="form-input"
        placeholder="App name or full path (e.g. edge, C:\Program Files\App\app.exe)"
        bind:value={newEntry}
        onkeydown={handleKey}
        disabled={noConfig}
      />
      <button class="btn btn-primary" disabled={noConfig || saving || !newEntry.trim()} onclick={addExclusion}>
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Add
      </button>
    </div>
    <div class="picker-trigger-row" style="margin-top:8px;">
      <button class="btn btn-secondary" disabled={noConfig} onclick={browseExeForExclude}>
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
        Browse EXE…
      </button>
      <button class="btn btn-secondary" disabled={noConfig} onclick={() => showPicker = true}>
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="8" y1="12" x2="16" y2="12"/><line x1="8" y1="8" x2="16" y2="8"/><line x1="8" y1="16" x2="13" y2="16"/></svg>
        From Running…
      </button>
    </div>
    <div class="form-hint mt-1">
      Partial name match is supported — "firefox" will exclude any process containing that string.
      Use a path prefix to exclude an entire folder (e.g. UWP apps).
    </div>
  </div>

  {#if noConfig}
    <div class="empty-state">
      <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><line x1="4.93" y1="4.93" x2="19.07" y2="19.07"/></svg>
      <div class="empty-state-title">No configuration loaded</div>
      <div class="empty-state-desc">Go to Settings to set the config file path first.</div>
    </div>
  {:else if $config!.excludes.length === 0}
    <div class="empty-state">
      <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><line x1="4.93" y1="4.93" x2="19.07" y2="19.07"/></svg>
      <div class="empty-state-title">No exclusions defined</div>
      <div class="empty-state-desc">All applications will be subject to proxy rules.</div>
    </div>
  {:else}
    {#each $config!.excludes as entry, idx}
      <div class="exclude-item">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--text-subtle)" stroke-width="2" style="flex-shrink:0;"><circle cx="12" cy="12" r="10"/><line x1="4.93" y1="4.93" x2="19.07" y2="19.07"/></svg>
        <span class="exclude-name">{entry}</span>
        <button class="btn-icon danger" title="Remove" onclick={() => removeExclusion(idx)}>
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6"/><path d="M10 11v6M14 11v6"/><path d="M9 6V4h6v2"/></svg>
        </button>
      </div>
    {/each}
  {/if}
</div>

{#if showPicker}
  <AppPicker onAdd={addFromPicker} onClose={() => showPicker = false} />
{/if}
