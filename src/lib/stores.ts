// ProxiFyre UI — Svelte stores
// Author: Dmitry Osin <d@osin.pro>

import { writable } from 'svelte/store';
import type { AppSettings, Notification, ProxiFyreConfig, ServiceStatus, Theme } from './types';

function persistedTheme(): Theme {
  if (typeof window === 'undefined') return 'dark';
  return (localStorage.getItem('theme') as Theme) ?? 'dark';
}

export const theme = writable<Theme>(persistedTheme());

export const config = writable<ProxiFyreConfig | null>(null);

export const appSettings = writable<AppSettings>({
  proxifyrePath: '',
  configPath: '',
  startWithWindows: false,
  startMinimized: false,
});

export const serviceStatus = writable<ServiceStatus>({
  installed: false,
  running: false,
  status: 'Unknown',
});

export const isAdmin = writable<boolean>(false);

export const notification = writable<Notification | null>(null);

export function notify(type: Notification['type'], message: string) {
  notification.set({ type, message });
}

export const LOG_LEVELS = ['Error', 'Warning', 'Info', 'Debug', 'All'] as const;
