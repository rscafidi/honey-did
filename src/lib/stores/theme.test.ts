import { describe, it, expect, beforeEach } from 'vitest';
import { get } from 'svelte/store';
import { themePreference, systemTheme, theme, cycleTheme, setTheme } from './theme';

describe('Theme Store', () => {
  beforeEach(() => {
    // Reset to defaults
    themePreference.set('auto');
    systemTheme.set('light');
    localStorage.removeItem('theme-preference');
  });

  describe('themePreference', () => {
    it('defaults to auto', () => {
      expect(get(themePreference)).toBe('auto');
    });

    it('can be set to light', () => {
      themePreference.set('light');
      expect(get(themePreference)).toBe('light');
    });

    it('can be set to dark', () => {
      themePreference.set('dark');
      expect(get(themePreference)).toBe('dark');
    });

    it('persists to localStorage', () => {
      themePreference.set('dark');
      expect(localStorage.getItem('theme-preference')).toBe('dark');
    });
  });

  describe('theme (derived)', () => {
    it('resolves auto to system theme (light)', () => {
      themePreference.set('auto');
      systemTheme.set('light');
      expect(get(theme)).toBe('light');
    });

    it('resolves auto to system theme (dark)', () => {
      themePreference.set('auto');
      systemTheme.set('dark');
      expect(get(theme)).toBe('dark');
    });

    it('overrides system when preference is explicit', () => {
      systemTheme.set('dark');
      themePreference.set('light');
      expect(get(theme)).toBe('light');
    });

    it('applies data-theme attribute to document', () => {
      themePreference.set('dark');
      expect(document.documentElement.getAttribute('data-theme')).toBe('dark');
    });
  });

  describe('cycleTheme', () => {
    it('cycles auto -> light -> dark -> auto', () => {
      themePreference.set('auto');
      cycleTheme();
      expect(get(themePreference)).toBe('light');
      cycleTheme();
      expect(get(themePreference)).toBe('dark');
      cycleTheme();
      expect(get(themePreference)).toBe('auto');
    });
  });

  describe('setTheme', () => {
    it('sets theme preference directly', () => {
      setTheme('dark');
      expect(get(themePreference)).toBe('dark');
      setTheme('light');
      expect(get(themePreference)).toBe('light');
      setTheme('auto');
      expect(get(themePreference)).toBe('auto');
    });
  });
});
