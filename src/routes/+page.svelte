<script lang="ts">
  import { goto } from '$app/navigation';
  import { api } from '$lib/api';
  import { appSettings, config, serviceStatus, notify } from '$lib/stores';

  let busy = $state('');

  async function serviceAction(action: 'start' | 'stop' | 'restart' | 'install' | 'uninstall') {
    busy = action;
    try {
      if (action === 'start')     await api.startService();
      if (action === 'stop')      await api.stopService();
      if (action === 'restart')   await api.restartService();
      if (action === 'install')   await api.installService($appSettings.proxifyrePath);
      if (action === 'uninstall') await api.uninstallService($appSettings.proxifyrePath);
      notify('success', `Service ${action}ed successfully`);
      const s = await api.getServiceStatus();
      serviceStatus.set(s);
    } catch (e) {
      notify('error', String(e));
    } finally {
      busy = '';
    }
  }

  const isRunning   = $derived($serviceStatus.running);
  const isInstalled = $derived($serviceStatus.installed);
  const noExe       = $derived(!$appSettings.proxifyrePath);
  const totalApps   = $derived($config?.proxies.reduce((n, p) => n + p.appNames.length, 0) ?? 0);
</script>

<div class="page-header">
  <div class="page-title">Dashboard</div>
  <div class="page-subtitle">Service management and configuration overview</div>
</div>

<div class="page-body">

  {#if noExe}
    <div class="card" style="border-color:var(--warning); background:var(--warning-bg); margin-bottom:16px;">
      <div class="flex items-center gap-3">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--warning)" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
        <div style="flex:1">
          <div style="font-weight:600; color:var(--warning);">ProxiFyre not configured</div>
          <div style="font-size:12px; color:var(--text-muted); margin-top:2px;">
            Set the path to ProxiFyre.exe and the config file in Settings.
          </div>
        </div>
        <button class="btn btn-secondary btn-sm" onclick={() => goto('/settings')}>Open Settings</button>
      </div>
    </div>
  {/if}

  <div class="card">
    <div class="flex items-center justify-between" style="margin-bottom:20px;">
      <div>
        <div class="card-title" style="margin-bottom:4px;">Windows Service</div>
        <code style="font-size:12px; color:var(--text-subtle);">ProxiFyreService</code>
      </div>
      {#if isRunning}
        <span class="badge badge-success" style="font-size:12.5px; padding:4px 12px; gap:6px;">
          <span class="status-dot running" style="display:inline-block;"></span> Running
        </span>
      {:else if isInstalled}
        <span class="badge badge-error" style="font-size:12.5px; padding:4px 12px; gap:6px;">
          <span class="status-dot stopped" style="display:inline-block;"></span> Stopped
        </span>
      {:else}
        <span class="badge badge-warning" style="font-size:12.5px; padding:4px 12px;">Not Installed</span>
      {/if}
    </div>

    <div class="flex gap-2" style="flex-wrap:wrap;">
      {#if isInstalled}
        {#if !isRunning}
          <button class="btn btn-success" disabled={!!busy} onclick={() => serviceAction('start')}>
            <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>
            {busy === 'start' ? 'Starting…' : 'Start'}
          </button>
        {/if}
        {#if isRunning}
          <button class="btn btn-danger" disabled={!!busy} onclick={() => serviceAction('stop')}>
            <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
            {busy === 'stop' ? 'Stopping…' : 'Stop'}
          </button>
          <button class="btn btn-secondary" disabled={!!busy} onclick={() => serviceAction('restart')}>
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 1 0 .49-3.98"/></svg>
            {busy === 'restart' ? 'Restarting…' : 'Restart'}
          </button>
        {/if}
        <button
          class="btn btn-danger"
          disabled={!!busy || noExe}
          onclick={() => { if (confirm('Uninstall ProxiFyreService?')) serviceAction('uninstall'); }}
        >
          {busy === 'uninstall' ? 'Uninstalling…' : 'Uninstall Service'}
        </button>
      {:else}
        <button class="btn btn-primary" disabled={!!busy || noExe} onclick={() => serviceAction('install')}>
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
          {busy === 'install' ? 'Installing…' : 'Install Service'}
        </button>
      {/if}
    </div>
  </div>

  {#if $config}
    <div class="grid-3 mt-4">
      <div class="card">
        <div class="stat-value">{$config.proxies.length}</div>
        <div class="stat-label">Proxy Rules</div>
      </div>
      <div class="card">
        <div class="stat-value">{totalApps}</div>
        <div class="stat-label">Proxied Applications</div>
      </div>
      <div class="card">
        <div class="stat-value">{$config.excludes.length}</div>
        <div class="stat-label">Excluded Applications</div>
      </div>
    </div>

    <div class="card mt-4">
      <div class="card-title">Configuration Summary</div>
      <div class="grid-2">
        <div>
          <div style="font-size:12px; color:var(--text-muted);">Log Level</div>
          <div style="font-weight:600; margin-top:4px;">{$config.logLevel}</div>
        </div>
        <div>
          <div style="font-size:12px; color:var(--text-muted);">LAN Bypass</div>
          <div style="margin-top:4px;">
            {#if $config.bypassLan}
              <span class="badge badge-success">Enabled</span>
            {:else}
              <span class="badge badge-error">Disabled</span>
            {/if}
          </div>
        </div>
      </div>
      {#if $appSettings.configPath}
        <div class="divider" style="margin:16px 0 12px;"></div>
        <div style="font-size:12px; color:var(--text-muted);">Config File</div>
        <code style="font-size:11.5px; color:var(--text-subtle); word-break:break-all; display:block; margin-top:4px;">
          {$appSettings.configPath}
        </code>
      {/if}
    </div>
  {:else}
    <div class="empty-state mt-4">
      <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>
      <div class="empty-state-title">No configuration loaded</div>
      <div class="empty-state-desc">Configure the path to ProxiFyre's config file in Settings.</div>
    </div>
  {/if}

</div>

