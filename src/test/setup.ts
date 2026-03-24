import '@testing-library/jest-dom/vitest';

// Mock window.matchMedia (not implemented in jsdom)
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

// Mock Tauri APIs
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/plugin-dialog', () => ({
  save: vi.fn(),
  open: vi.fn(),
}));

vi.mock('@tauri-apps/plugin-fs', () => ({
  writeTextFile: vi.fn(),
  readTextFile: vi.fn(),
}));

// Mock CSS variables for jsdom
const style = window.document.createElement('style');
style.textContent = `
  :root {
    --text-primary: #1a1a2e;
    --text-secondary: #555;
    --bg-primary: #fff;
    --bg-secondary: #f5f5f7;
    --bg-tertiary: #fafafa;
    --border-color: #e0e0e0;
    --accent-primary: #4a6fa5;
    --accent-light: #eef2f7;
    --accent-hover: #dde5ef;
    --error-color: #9b2c2c;
  }
`;
window.document.head.appendChild(style);
