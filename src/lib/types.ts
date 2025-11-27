// ProxiFyre UI — TypeScript type definitions
// Author: Dmitry Osin <d@osin.pro>

export interface ProxyEntry {
  name?: string;
  appNames: string[];
  socks5ProxyEndpoint: string;
  username?: string;
  password?: string;
  supportedProtocols: string[];
}

export interface ProxiFyreConfig {
  logLevel: string;
  bypassLan: boolean;
  proxies: ProxyEntry[];
  excludes: string[];
}

export interface AppSettings {
  proxifyrePath: string;
  configPath: string;
  startWithWindows: boolean;
  startMinimized: boolean;
}

export interface ServiceStatus {
  installed: boolean;
  running: boolean;
  status: string;
}

export type LogLevel = 'Error' | 'Warning' | 'Info' | 'Debug' | 'All';
export type Theme = 'dark' | 'light';
export type NotificationType = 'success' | 'error' | 'info';

export interface Notification {
  type: NotificationType;
  message: string;
}
