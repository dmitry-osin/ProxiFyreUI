<script lang="ts">
  import { page } from '$app/stores';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { getVersion } from '@tauri-apps/api/app';
  import { theme, serviceStatus, isAdmin, notification, appSettings, config, notify } from '$lib/stores';
  import { api } from '$lib/api';
  import '../app.css';

  const { children } = $props();

  let appVersion = $state('...');

  $effect(() => {
    const t = $theme;
    document.documentElement.classList.toggle('light', t === 'light');
    localStorage.setItem('theme', t);
  });

  $effect(() => {
    const win = getCurrentWindow();
    const unlisten = win.onResized(async () => {
      if (await win.isMinimized()) await win.hide();
    });
    return () => { unlisten.then(fn => fn()); };
  });

  $effect(() => {
    bootstrap();
    const interval = setInterval(pollStatus, 5000);
    return () => clearInterval(interval);
  });

  $effect(() => {
    const unlisten = listen<{ action: string }>('tray-action', async ({ payload }) => {
      try {
        if (payload.action === 'install')   await api.installService($appSettings.proxifyrePath);
        if (payload.action === 'uninstall') await api.uninstallService($appSettings.proxifyrePath);
        if (payload.action === 'restart')   await api.restartService();
        const s = await api.getServiceStatus();
        serviceStatus.set(s);
        notify('success', `Service ${payload.action}ed`);
      } catch (e) {
        notify('error', String(e));
      }
    });
    return () => { unlisten.then(fn => fn()); };
  });

  async function bootstrap() {
    try {
      const settings = await api.loadAppSettings();
      appSettings.set(settings);
      if (settings.configPath) {
        const cfg = await api.loadProxifyrConfig(settings.configPath);
        config.set(cfg);
      }
    } catch { /* first run */ }

    appVersion = await getVersion().catch(() => '?');
    const admin = await api.checkAdmin();
    isAdmin.set(admin);
    await pollStatus();
  }

  async function pollStatus() {
    try {
      const s = await api.getServiceStatus();
      serviceStatus.set(s);
    } catch { /* ignore polling errors */ }
  }

  function toggleTheme() {
    theme.update(t => (t === 'dark' ? 'light' : 'dark'));
  }

  $effect(() => {
    if ($notification) {
      const t = setTimeout(() => notification.set(null), 3500);
      return () => clearTimeout(t);
    }
  });

  const navItems = [
    { href: '/',         label: 'Dashboard',  icon: 'home'     },
    { href: '/proxies',  label: 'Proxies',    icon: 'server'   },
    { href: '/excludes', label: 'Exclusions', icon: 'ban'      },
    { href: '/settings', label: 'Settings',   icon: 'settings' },
  ] as const;

  function isActive(href: string) {
    return href === '/'
      ? $page.url.pathname === '/'
      : $page.url.pathname.startsWith(href);
  }

  function dotClass(status: string) {
    if (status === 'Running') return 'running';
    if (status === 'Stopped' || status === 'Not Installed') return 'stopped';
    return 'unknown';
  }
</script>

<div class="app-layout">
  <aside class="sidebar">
    <div class="sidebar-brand">
      <div class="sidebar-brand-icon" aria-hidden="true">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2">
          <circle cx="12" cy="12" r="3"/>
          <path d="M12 1v4M12 19v4M4.22 4.22l2.83 2.83M16.95 16.95l2.83 2.83M1 12h4M19 12h4M4.22 19.78l2.83-2.83M16.95 7.05l2.83-2.83"/>
        </svg>
      </div>
      <div>
        <div class="sidebar-brand-name">ProxiFyre UI</div>
        <div class="sidebar-brand-ver">v{appVersion} by Dmitry Osin</div>
      </div>
    </div>

    <nav class="sidebar-nav">
      {#each navItems as item}
        <a href={item.href} class="nav-item" class:active={isActive(item.href)}>
          {#if item.icon === 'home'}
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
          {:else if item.icon === 'server'}
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="2" width="20" height="8" rx="2"/><rect x="2" y="14" width="20" height="8" rx="2"/><circle cx="6" cy="6" r="1" fill="currentColor"/><circle cx="6" cy="18" r="1" fill="currentColor"/></svg>
          {:else if item.icon === 'ban'}
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="4.93" y1="4.93" x2="19.07" y2="19.07"/></svg>
          {:else if item.icon === 'settings'}
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M12 1v4M12 19v4M4.22 4.22l2.83 2.83M16.95 16.95l2.83 2.83M1 12h4M19 12h4M4.22 19.78l2.83-2.83M16.95 7.05l2.83-2.83"/></svg>
          {/if}
          {item.label}
        </a>
      {/each}
    </nav>

    <div class="sidebar-footer">
      <div class="service-pill">
        <span class="status-dot {dotClass($serviceStatus.status)}"></span>
        <span style="flex:1; color:var(--text-muted); font-size:12px;">{$serviceStatus.status}</span>
        <span style="font-size:11px; color:var(--text-subtle);">ProxiFyre</span>
      </div>
      <button class="theme-btn" onclick={toggleTheme}>
        {#if $theme === 'dark'}
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>
          Light Mode
        {:else}
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>
          Dark Mode
        {/if}
      </button>
    </div>
  </aside>

  <div class="main-area">
    {#if !$isAdmin}
      <div class="admin-banner">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
        Not running as Administrator — service management requires elevation.
        <button
          class="btn btn-sm btn-secondary"
          style="margin-left:auto"
          onclick={() => api.relaunchAsAdmin().catch(e => notify('error', String(e)))}
        >
          Restart as Admin
        </button>
      </div>
    {/if}

    {@render children()}
  </div>
</div>

{#if $notification}
  <div class="toast {$notification.type}">
    {#if $notification.type === 'success'}
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polyline points="20 6 9 17 4 12"/></svg>
    {:else if $notification.type === 'error'}
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>
    {:else}
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
    {/if}
    {$notification.message}
  </div>
{/if}
