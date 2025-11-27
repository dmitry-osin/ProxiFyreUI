<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { api } from '$lib/api';
  import { appSettings, config, notify, LOG_LEVELS } from '$lib/stores';
  import type { AppSettings, ProxiFyreConfig } from '$lib/types';

  let localSettings = $state<AppSettings>({ proxifyrePath: '', configPath: '', startWithWindows: false, startMinimized: false });
  let logLevel      = $state('Error');
  let bypassLan     = $state(false);
  let savingSettings = $state(false);
  let savingConfig   = $state(false);

  $effect(() => {
    localSettings = { ...$appSettings };
  });

  $effect(() => {
    if ($config) {
      logLevel  = $config.logLevel;
      bypassLan = $config.bypassLan;
    }
  });

  async function browseExe() {
    const result = await open({
      multiple:  false,
      filters:   [{ name: 'Executable', extensions: ['exe'] }],
      title:     'Select ProxiFyre.exe',
    });
    if (result) localSettings = { ...localSettings, proxifyrePath: result as string };
  }

  async function browseJson() {
    const result = await open({
      multiple:  false,
      filters:   [{ name: 'JSON Config', extensions: ['json'] }],
      title:     'Select app-config.json',
    });
    if (result) localSettings = { ...localSettings, configPath: result as string };
  }

  async function saveSettings() {
    savingSettings = true;
    try {
      await api.saveAppSettings(localSettings);
      appSettings.set(localSettings);

      if (localSettings.configPath) {
        const cfg = await api.loadProxifyrConfig(localSettings.configPath);
        config.set(cfg);
        logLevel  = cfg.logLevel;
        bypassLan = cfg.bypassLan;
      }
      notify('success', 'Settings saved');
    } catch (e) {
      notify('error', String(e));
    } finally {
      savingSettings = false;
    }
  }

  async function createDefaultConfig() {
    if (!localSettings.configPath) {
      notify('error', 'Set the config file path first');
      return;
    }
    const defaultCfg: ProxiFyreConfig = {
      logLevel:  'Error',
      bypassLan: false,
      proxies:   [],
      excludes:  [],
    };
    try {
      await api.saveProxifyrConfig(localSettings.configPath, defaultCfg);
      config.set(defaultCfg);
      logLevel  = defaultCfg.logLevel;
      bypassLan = defaultCfg.bypassLan;
      notify('success', 'Default config created');
    } catch (e) {
      notify('error', String(e));
    }
  }

  async function saveConfigOptions() {
    if (!$config || !$appSettings.configPath) return;
    savingConfig = true;
    try {
      const updated = { ...$config, logLevel, bypassLan };
      await api.saveProxifyrConfig($appSettings.configPath, updated);
      config.set(updated);
      notify('success', 'Configuration saved');
    } catch (e) {
      notify('error', String(e));
    } finally {
      savingConfig = false;
    }
  }

  const hasConfig = $derived(!!$config && !!$appSettings.configPath);
</script>

<div class="page-header">
  <div class="page-title">Settings</div>
  <div class="page-subtitle">ProxiFyre path, configuration file and global options</div>
</div>

<div class="page-body">

  <div class="settings-section">
    <div class="section-header">Application Behavior</div>
    <div class="section-body">

      <div class="setting-row" style="border-top:none; padding-top:0;">
        <div>
          <div class="setting-label">Start with Windows</div>
          <div class="setting-desc">
            Launch ProxiFyre UI automatically when Windows starts (stored in the current-user Run registry key).
          </div>
        </div>
        <label class="toggle" style="margin-left:16px;">
          <input type="checkbox" bind:checked={localSettings.startWithWindows} />
          <span class="toggle-slider"></span>
        </label>
      </div>

      <div class="setting-row">
        <div>
          <div class="setting-label">Start minimized</div>
          <div class="setting-desc">
            Start hidden in the system tray instead of showing the main window on launch.
          </div>
        </div>
        <label class="toggle" style="margin-left:16px;">
          <input type="checkbox" bind:checked={localSettings.startMinimized} />
          <span class="toggle-slider"></span>
        </label>
      </div>

      <div style="margin-top:20px;">
        <button class="btn btn-primary" disabled={savingSettings} onclick={saveSettings}>
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
          {savingSettings ? 'Saving…' : 'Save Behavior Settings'}
        </button>
      </div>
    </div>
  </div>

  <div class="settings-section">
    <div class="section-header">ProxiFyre Paths</div>
    <div class="section-body">
      <div class="form-group">
        <label class="form-label" for="exe-path">ProxiFyre Executable</label>
        <div class="input-row">
          <input
            id="exe-path"
            class="form-input"
            placeholder="C:\...\ProxiFyre.exe"
            bind:value={localSettings.proxifyrePath}
          />
          <button class="btn btn-secondary" onclick={browseExe}>Browse…</button>
        </div>
        <div class="form-hint">Path to <code>ProxiFyre.exe</code> — used for service install / uninstall commands.</div>
      </div>

      <div class="form-group">
        <label class="form-label" for="cfg-path">Configuration File</label>
        <div class="input-row">
          <input
            id="cfg-path"
            class="form-input"
            placeholder="C:\...\app-config.json"
            bind:value={localSettings.configPath}
          />
          <button class="btn btn-secondary" onclick={browseJson}>Browse…</button>
        </div>
        <div class="form-hint">Path to <code>app-config.json</code> — the ProxiFyre configuration file.</div>
      </div>

      <div class="flex gap-2" style="flex-wrap:wrap; margin-top:4px;">
        <button class="btn btn-primary" disabled={savingSettings} onclick={saveSettings}>
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
          {savingSettings ? 'Saving…' : 'Save Paths'}
        </button>
        {#if localSettings.configPath && !$config}
          <button class="btn btn-secondary" onclick={createDefaultConfig}>
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="12" y1="18" x2="12" y2="12"/><line x1="9" y1="15" x2="15" y2="15"/></svg>
            Create Default Config
          </button>
        {/if}
      </div>
    </div>
  </div>

  <div class="settings-section">
    <div class="section-header">ProxiFyre Global Options</div>
    <div class="section-body">
      {#if !hasConfig}
        <div style="color:var(--text-muted); font-size:13px; padding:8px 0;">
          Load a configuration file first to edit these options.
        </div>
      {:else}
        <div class="form-group">
          <label class="form-label" for="loglevel">Log Level</label>
          <select id="loglevel" class="form-select" bind:value={logLevel}>
            {#each LOG_LEVELS as level}
              <option value={level}>{level}</option>
            {/each}
          </select>
          <div class="form-hint">Controls the verbosity of ProxiFyre's log output in the <code>/logs</code> directory.</div>
        </div>

        <div class="setting-row" style="border-top:none; padding-top:0;">
          <div>
            <div class="setting-label">Bypass LAN</div>
            <div class="setting-desc">
              Local network traffic (10.x, 172.16–31.x, 192.168.x, 169.254.x, multicast) bypasses the proxy.
            </div>
          </div>
          <label class="toggle" style="margin-left:16px;">
            <input type="checkbox" bind:checked={bypassLan} />
            <span class="toggle-slider"></span>
          </label>
        </div>

        <div style="margin-top:20px;">
          <button class="btn btn-primary" disabled={savingConfig} onclick={saveConfigOptions}>
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
            {savingConfig ? 'Saving…' : 'Save Options'}
          </button>
        </div>
      {/if}
    </div>
  </div>

  <div class="settings-section">
    <div class="section-header">About</div>
    <div class="section-body">
      <div class="setting-row" style="border-top:none; padding-top:0;">
        <div>
          <div class="setting-label">ProxiFyre UI</div>
          <div class="setting-desc">Configuration manager and service controller for ProxiFyre</div>
        </div>
        <span class="badge badge-accent">v0.1.0</span>
      </div>
      <div class="setting-row">
        <div>
          <div class="setting-label">Author</div>
          <div class="setting-desc">Dmitry Osin &lt;d@osin.pro&gt;</div>
        </div>
        <a
          href="https://github.com/dmitry-osin/ProxiFyreUI"
          class="btn btn-secondary btn-sm"
          target="_blank"
          rel="noreferrer noopener"
        >GitHub</a>
      </div>
      <div class="setting-row">
        <div>
          <div class="setting-label">ProxiFyre</div>
          <div class="setting-desc">SOCKS5 proxifier for Windows with UDP support by wiresock</div>
        </div>
        <a
          href="https://github.com/wiresock/proxifyre"
          class="btn btn-secondary btn-sm"
          target="_blank"
          rel="noreferrer noopener"
        >GitHub</a>
      </div>
    </div>
  </div>

</div>
