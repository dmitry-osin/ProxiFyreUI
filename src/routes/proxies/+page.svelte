<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import AppPicker from '$lib/AppPicker.svelte';
  import { api } from '$lib/api';
  import { appSettings, config, notify } from '$lib/stores';
  import type { ProxyEntry } from '$lib/types';

  let showModal   = $state(false);
  let editIndex   = $state<number | null>(null);
  let saving      = $state(false);

  let proxyName = $state('');
  let endpoint  = $state('');
  let username  = $state('');
  let password  = $state('');
  let tcpOn     = $state(true);
  let udpOn     = $state(false);
  let appNames  = $state<string[]>([]);
  let tagInput  = $state('');

  function openAdd() {
    editIndex = null;
    proxyName = 'New';
    endpoint  = '';
    username  = '';
    password  = '';
    tcpOn     = true;
    udpOn     = false;
    appNames  = [];
    tagInput  = '';
    showModal = true;
  }

  function openEdit(idx: number) {
    const p = $config!.proxies[idx];
    editIndex = idx;
    proxyName = p.name ?? '';
    endpoint  = p.socks5ProxyEndpoint;
    username  = p.username ?? '';
    password  = p.password ?? '';
    tcpOn     = p.supportedProtocols.includes('TCP');
    udpOn     = p.supportedProtocols.includes('UDP');
    appNames  = [...p.appNames];
    tagInput  = '';
    showModal = true;
  }

  function closeModal() { showModal = false; }

  function addTag() {
    const val = tagInput.trim().replace(/,+$/, '');
    if (val && !appNames.includes(val)) appNames = [...appNames, val];
    tagInput = '';
  }

  function handleTagKey(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ',') { e.preventDefault(); addTag(); }
    if (e.key === 'Backspace' && tagInput === '' && appNames.length > 0) {
      appNames = appNames.slice(0, -1);
    }
  }

  function removeTag(i: number) { appNames = appNames.filter((_, idx) => idx !== i); }

  async function saveProxy() {
    if (!endpoint.trim()) { notify('error', 'Endpoint is required'); return; }
    if (!tcpOn && !udpOn)  { notify('error', 'Select at least one protocol'); return; }

    const protocols: string[] = [];
    if (tcpOn) protocols.push('TCP');
    if (udpOn) protocols.push('UDP');

    if (tagInput.trim()) addTag();

    const entry: ProxyEntry = {
      name: proxyName.trim() || undefined,
      appNames,
      socks5ProxyEndpoint: endpoint.trim(),
      username: username.trim() || undefined,
      password: password.trim() || undefined,
      supportedProtocols: protocols,
    };

    saving = true;
    try {
      const cfg = $config!;
      if (editIndex === null) {
        cfg.proxies = [...cfg.proxies, entry];
      } else {
        cfg.proxies = cfg.proxies.map((p, i) => (i === editIndex ? entry : p));
      }
      config.set(cfg);
      await api.saveProxifyrConfig($appSettings.configPath, cfg);
      notify('success', editIndex === null ? 'Proxy added' : 'Proxy updated');
      closeModal();
    } catch (e) {
      notify('error', String(e));
    } finally {
      saving = false;
    }
  }

  async function deleteProxy(idx: number) {
    if (!confirm('Delete this proxy rule?')) return;
    try {
      const cfg = $config!;
      cfg.proxies = cfg.proxies.filter((_, i) => i !== idx);
      config.set(cfg);
      await api.saveProxifyrConfig($appSettings.configPath, cfg);
      notify('success', 'Proxy deleted');
    } catch (e) {
      notify('error', String(e));
    }
  }

  function handleOverlayKey(e: KeyboardEvent) {
    if (e.key === 'Escape') closeModal();
  }

  const noConfig = $derived(!$config || !$appSettings.configPath);

  let showPicker = $state(false);

  function addFromPicker(names: string[]) {
    names.forEach(n => {
      if (!appNames.includes(n)) appNames = [...appNames, n];
    });
  }

  async function browseExeForProxy() {
    const result = await open({
      multiple: false,
      filters: [{ name: 'Executable', extensions: ['exe'] }],
      title: 'Select Executable',
    });
    if (result) {
      const name = result as string;
      if (!appNames.includes(name)) appNames = [...appNames, name];
    }
  }
</script>

<div class="page-header">
  <div class="page-header-actions">
    <div>
      <div class="page-title">Proxies</div>
      <div class="page-subtitle">SOCKS5 proxy rules and application associations</div>
    </div>
    <button class="btn btn-primary" disabled={noConfig} onclick={openAdd}>
      <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
      Add Proxy
    </button>
  </div>
</div>

<div class="page-body">
  {#if noConfig}
    <div class="empty-state">
      <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="2" y="2" width="20" height="8" rx="2"/><rect x="2" y="14" width="20" height="8" rx="2"/><circle cx="6" cy="6" r="1" fill="currentColor"/><circle cx="6" cy="18" r="1" fill="currentColor"/></svg>
      <div class="empty-state-title">No configuration loaded</div>
      <div class="empty-state-desc">Go to Settings to set the config file path first.</div>
    </div>
  {:else if $config!.proxies.length === 0}
    <div class="empty-state">
      <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="2" y="2" width="20" height="8" rx="2"/><rect x="2" y="14" width="20" height="8" rx="2"/><circle cx="6" cy="6" r="1" fill="currentColor"/><circle cx="6" cy="18" r="1" fill="currentColor"/></svg>
      <div class="empty-state-title">No proxy rules yet</div>
      <div class="empty-state-desc">Click "Add Proxy" to create your first SOCKS5 proxy rule.</div>
    </div>
  {:else}
    {#each $config!.proxies as proxy, idx}
      <div class="proxy-card">
        <div class="flex items-center justify-between">
          <div style="flex:1; min-width:0;">
            {#if proxy.name}
              <div class="proxy-name">{proxy.name}</div>
            {/if}
            <div class="proxy-endpoint">{proxy.socks5ProxyEndpoint}</div>
            <div class="proxy-meta">
              {#if proxy.username}
                <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="vertical-align:-1px; margin-right:3px;"><circle cx="12" cy="8" r="4"/><path d="M6 20v-2a6 6 0 0 1 12 0v2"/></svg>
                {proxy.username}
              {:else}
                No authentication
              {/if}
            </div>
          </div>
          <div class="flex items-center gap-2">
            {#each proxy.supportedProtocols as proto}
              <span class="badge badge-{proto.toLowerCase()}">{proto}</span>
            {/each}
            <button class="btn-icon" title="Edit" onclick={() => openEdit(idx)}>
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
            </button>
            <button class="btn-icon danger" title="Delete" onclick={() => deleteProxy(idx)}>
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6"/><path d="M10 11v6M14 11v6"/><path d="M9 6V4h6v2"/></svg>
            </button>
          </div>
        </div>

        {#if proxy.appNames.length > 0}
          <div class="proxy-chips">
            {#each proxy.appNames.slice(0, 6) as name}
              <span class="app-chip">{name}</span>
            {/each}
            {#if proxy.appNames.length > 6}
              <span class="app-chip" style="color:var(--accent);">+{proxy.appNames.length - 6} more</span>
            {/if}
          </div>
        {:else}
          <div style="margin-top:8px; font-size:12px; color:var(--text-subtle);">
            No specific apps — applies to all unassociated processes
          </div>
        {/if}
      </div>
    {/each}
  {/if}
</div>

{#if showModal}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="overlay" role="dialog" aria-modal="true" tabindex="-1" onkeydown={handleOverlayKey}>
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="modal" onclick={e => e.stopPropagation()}>
      <div class="modal-header">
        <span class="modal-title">{editIndex === null ? 'Add Proxy Rule' : 'Edit Proxy Rule'}</span>
        <button class="btn-icon" onclick={closeModal} title="Close">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>

      <div class="modal-body">
        <div class="form-group">
          <label class="form-label" for="pname">Name</label>
          <input id="pname" class="form-input" placeholder="e.g. Work Proxy" bind:value={proxyName} />
        </div>

        <div class="form-group">
          <label class="form-label" for="ep">SOCKS5 Endpoint <span style="color:var(--error)">*</span></label>
          <input id="ep" class="form-input" placeholder="host:port (e.g. 127.0.0.1:1080)" bind:value={endpoint} />
        </div>

        <div class="grid-2">
          <div class="form-group">
            <label class="form-label" for="usr">Username <span style="color:var(--text-subtle)">(optional)</span></label>
            <input id="usr" class="form-input" placeholder="username" bind:value={username} autocomplete="off" />
          </div>
          <div class="form-group">
            <label class="form-label" for="pwd">Password <span style="color:var(--text-subtle)">(optional)</span></label>
            <input id="pwd" class="form-input" type="password" placeholder="••••••••" bind:value={password} autocomplete="new-password" />
          </div>
        </div>

        <div class="form-group">
          <div class="form-label">Protocols <span style="color:var(--error)">*</span></div>
          <div class="protocol-group">
            <button
              class="proto-btn"
              class:on-tcp={tcpOn}
              onclick={() => tcpOn = !tcpOn}
            >TCP</button>
            <button
              class="proto-btn"
              class:on-udp={udpOn}
              onclick={() => udpOn = !udpOn}
            >UDP</button>
          </div>
          <div class="form-hint">Enable UDP for QUIC-based traffic (e.g. HTTP/3).</div>
        </div>

        <div class="form-group">
          <div class="form-label">Application Names</div>
          <div class="picker-trigger-row">
            <button class="btn btn-secondary" type="button" onclick={browseExeForProxy}>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
              Browse EXE…
            </button>
            <button class="btn btn-secondary" type="button" onclick={() => showPicker = true}>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="8" y1="12" x2="16" y2="12"/><line x1="8" y1="8" x2="16" y2="8"/><line x1="8" y1="16" x2="13" y2="16"/></svg>
              From Running…
            </button>
          </div>
          <div class="tag-wrap">
            {#each appNames as name, i}
              <span class="chip">
                {name}
                <button class="chip-remove" onclick={() => removeTag(i)} title="Remove">×</button>
              </span>
            {/each}
            <input
              class="tag-input"
              placeholder={appNames.length === 0 ? 'chrome, firefox… (Enter to add)' : 'Add more…'}
              bind:value={tagInput}
              onkeydown={handleTagKey}
              onblur={addTag}
            />
          </div>
          <div class="form-hint">Leave empty to proxy all applications. Partial name match supported (e.g. "firefox" matches "firefox.exe").</div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn btn-secondary" onclick={closeModal}>Cancel</button>
        <button class="btn btn-primary" disabled={saving} onclick={saveProxy}>
          {saving ? 'Saving…' : editIndex === null ? 'Add Proxy' : 'Save Changes'}
        </button>
      </div>
    </div>
  </div>
{/if}

{#if showPicker}
  <AppPicker onAdd={addFromPicker} onClose={() => showPicker = false} />
{/if}
