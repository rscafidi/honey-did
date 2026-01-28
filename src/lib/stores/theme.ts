import { writable, derived } from 'svelte/store';

export type ThemePreference = 'auto' | 'light' | 'dark';
export type ResolvedTheme = 'light' | 'dark';

// Get initial preference from localStorage or default to 'auto'
function getInitialPreference(): ThemePreference {
  if (typeof window === 'undefined') return 'auto';
  const stored = localStorage.getItem('theme-preference');
  if (stored === 'light' || stored === 'dark' || stored === 'auto') {
    return stored;
  }
  return 'auto';
}

// Check system preference
function getSystemTheme(): ResolvedTheme {
  if (typeof window === 'undefined') return 'light';
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

// Create the preference store
export const themePreference = writable<ThemePreference>(getInitialPreference());

// System theme store (updates when system preference changes)
export const systemTheme = writable<ResolvedTheme>(getSystemTheme());

// Resolved theme (what's actually applied)
export const theme = derived(
  [themePreference, systemTheme],
  ([$pref, $system]) => $pref === 'auto' ? $system : $pref
);

// Initialize system theme listener
if (typeof window !== 'undefined') {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  mediaQuery.addEventListener('change', (e) => {
    systemTheme.set(e.matches ? 'dark' : 'light');
  });
}

// Save preference to localStorage when it changes
themePreference.subscribe((pref) => {
  if (typeof window !== 'undefined') {
    localStorage.setItem('theme-preference', pref);
  }
});

// Apply theme to document
theme.subscribe((t) => {
  if (typeof document !== 'undefined') {
    document.documentElement.setAttribute('data-theme', t);
  }
});

// Helper to cycle through themes
export function cycleTheme(): void {
  themePreference.update((current) => {
    if (current === 'auto') return 'light';
    if (current === 'light') return 'dark';
    return 'auto';
  });
}

// Set specific theme
export function setTheme(pref: ThemePreference): void {
  themePreference.set(pref);
}
