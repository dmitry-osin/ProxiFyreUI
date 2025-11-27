// ProxiFyre UI — Tauri invoke wrappers
// Author: Dmitry Osin <d@osin.pro>

import { invoke } from '@tauri-apps/api/core';
import type { AppSettings, ProxiFyreConfig, ServiceStatus } from './types';

export const api = {
  loadAppSettings: () =>
    invoke<AppSettings>('load_app_settings'),

  saveAppSettings: (settings: AppSettings) =>
    invoke<void>('save_app_settings', { settings }),

  loadProxifyrConfig: (path: string) =>
    invoke<ProxiFyreConfig>('load_proxifyre_config', { path }),

  saveProxifyrConfig: (path: string, config: ProxiFyreConfig) =>
    invoke<void>('save_proxifyre_config', { path, config }),

  getServiceStatus: () =>
    invoke<ServiceStatus>('get_service_status'),

  startService: () =>
    invoke<void>('start_service'),

  stopService: () =>
    invoke<void>('stop_service'),

  restartService: () =>
    invoke<void>('restart_service'),

  installService: (proxifyrePath: string) =>
    invoke<void>('install_service', { proxifyrePath }),

  uninstallService: (proxifyrePath: string) =>
    invoke<void>('uninstall_service', { proxifyrePath }),

  checkAdmin: () =>
    invoke<boolean>('check_admin'),

  relaunchAsAdmin: () =>
    invoke<void>('relaunch_as_admin'),

  listProcesses: () =>
    invoke<string[]>('list_processes'),
};
