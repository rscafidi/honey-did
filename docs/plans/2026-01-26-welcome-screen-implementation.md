# Welcome Screen Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a Welcome Screen section to the desktop app where users can create personalized message slides that play before the passphrase prompt in exported HTML documents.

**Architecture:** New Svelte section component for slide management, updated Rust data model to include welcome screen data, modified HTML export to embed and play slides before showing passphrase input.

**Tech Stack:** Svelte 4, TypeScript, Rust/Tauri, embedded JavaScript in HTML export

---

### Task 1: Add WelcomeScreen to Rust Data Model

**Files:**
- Modify: `src-tauri/src/models.rs`

**Step 1: Add WelcomeScreen types to models.rs**

Add after the existing model definitions:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageSlide {
    pub id: String,
    pub text: String,
    pub transition: SlideTransition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlideTransition {
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "auto")]
    Auto { seconds: u32 },
}

impl Default for SlideTransition {
    fn default() -> Self {
        SlideTransition::Click
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WelcomeScreen {
    pub enabled: bool,
    pub slides: Vec<MessageSlide>,
}
```

**Step 2: Add welcome_screen field to LegacyDocument**

Find the `LegacyDocument` struct and add:

```rust
pub welcome_screen: Option<WelcomeScreen>,
```

**Step 3: Build to verify**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

**Step 4: Commit**

```bash
git add src-tauri/src/models.rs
git commit -m "feat: add WelcomeScreen to data model"
```

---

### Task 2: Create WelcomeScreenSection Svelte Component

**Files:**
- Create: `src/lib/sections/WelcomeScreenSection.svelte`

**Step 1: Create the component**

```svelte
<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';

  interface MessageSlide {
    id: string;
    text: string;
    transition: { type: 'click' } | { type: 'auto'; seconds: number };
  }

  $: welcomeScreen = $document.welcome_screen || { enabled: false, slides: [] };
  $: slides = welcomeScreen.slides || [];

  let editingSlideId: string | null = null;

  function generateId(): string {
    return Math.random().toString(36).substring(2, 9);
  }

  function toggleEnabled() {
    document.update((doc) => ({
      ...doc,
      welcome_screen: {
        ...welcomeScreen,
        enabled: !welcomeScreen.enabled,
      },
    }));
  }

  function addSlide() {
    const newSlide: MessageSlide = {
      id: generateId(),
      text: '',
      transition: { type: 'click' },
    };
    document.update((doc) => ({
      ...doc,
      welcome_screen: {
        ...welcomeScreen,
        slides: [...slides, newSlide],
      },
    }));
    editingSlideId = newSlide.id;
  }

  function updateSlide(id: string, updates: Partial<MessageSlide>) {
    document.update((doc) => ({
      ...doc,
      welcome_screen: {
        ...welcomeScreen,
        slides: slides.map((s) => (s.id === id ? { ...s, ...updates } : s)),
      },
    }));
  }

  function deleteSlide(id: string) {
    document.update((doc) => ({
      ...doc,
      welcome_screen: {
        ...welcomeScreen,
        slides: slides.filter((s) => s.id !== id),
      },
    }));
    if (editingSlideId === id) {
      editingSlideId = null;
    }
  }

  function moveSlide(id: string, direction: 'up' | 'down') {
    const index = slides.findIndex((s) => s.id === id);
    if (index === -1) return;
    const newIndex = direction === 'up' ? index - 1 : index + 1;
    if (newIndex < 0 || newIndex >= slides.length) return;

    const newSlides = [...slides];
    [newSlides[index], newSlides[newIndex]] = [newSlides[newIndex], newSlides[index]];

    document.update((doc) => ({
      ...doc,
      welcome_screen: {
        ...welcomeScreen,
        slides: newSlides,
      },
    }));
  }

  function getSlidePreview(slide: MessageSlide): string {
    const text = slide.text || '(empty)';
    return text.length > 40 ? text.substring(0, 40) + '...' : text;
  }
</script>

<div class="welcome-screen-section">
  <div class="enable-toggle">
    <label class="toggle-label">
      <input type="checkbox" checked={welcomeScreen.enabled} on:change={toggleEnabled} />
      <span>Enable welcome screen for exports</span>
    </label>
  </div>

  {#if welcomeScreen.enabled}
    <div class="slides-section">
      <h3>Slides</h3>

      {#if slides.length === 0}
        <p class="empty-message">No slides yet. Add a slide to create your welcome message.</p>
      {/if}

      {#each slides as slide, index (slide.id)}
        <div class="slide-item">
          <div class="slide-header" on:click={() => (editingSlideId = editingSlideId === slide.id ? null : slide.id)}>
            <span class="slide-number">{index + 1}.</span>
            <span class="slide-preview">"{getSlidePreview(slide)}"</span>
            <div class="slide-actions">
              <button
                class="action-btn"
                on:click|stopPropagation={() => moveSlide(slide.id, 'up')}
                disabled={index === 0}
                title="Move up"
              >↑</button>
              <button
                class="action-btn"
                on:click|stopPropagation={() => moveSlide(slide.id, 'down')}
                disabled={index === slides.length - 1}
                title="Move down"
              >↓</button>
              <button
                class="action-btn delete"
                on:click|stopPropagation={() => deleteSlide(slide.id)}
                title="Delete"
              >×</button>
            </div>
          </div>

          {#if editingSlideId === slide.id}
            <div class="slide-editor">
              <div class="field">
                <label for="slide-text-{slide.id}">Message</label>
                <textarea
                  id="slide-text-{slide.id}"
                  value={slide.text}
                  on:input={(e) => updateSlide(slide.id, { text: e.currentTarget.value })}
                  placeholder="Enter your message..."
                  rows="3"
                ></textarea>
              </div>

              <div class="field">
                <label>Transition</label>
                <div class="radio-group">
                  <label class="radio-label">
                    <input
                      type="radio"
                      name="transition-{slide.id}"
                      checked={slide.transition.type === 'click'}
                      on:change={() => updateSlide(slide.id, { transition: { type: 'click' } })}
                    />
                    <span>Click to continue</span>
                  </label>
                  <label class="radio-label">
                    <input
                      type="radio"
                      name="transition-{slide.id}"
                      checked={slide.transition.type === 'auto'}
                      on:change={() =>
                        updateSlide(slide.id, {
                          transition: { type: 'auto', seconds: slide.transition.type === 'auto' ? slide.transition.seconds : 3 },
                        })}
                    />
                    <span>Auto-advance after</span>
                    {#if slide.transition.type === 'auto'}
                      <input
                        type="number"
                        min="1"
                        max="10"
                        value={slide.transition.seconds}
                        on:input={(e) =>
                          updateSlide(slide.id, {
                            transition: { type: 'auto', seconds: parseInt(e.currentTarget.value) || 3 },
                          })}
                        class="seconds-input"
                      />
                      <span>seconds</span>
                    {/if}
                  </label>
                </div>
              </div>

              <button class="btn btn-secondary" on:click={() => (editingSlideId = null)}>Done</button>
            </div>
          {/if}
        </div>
      {/each}

      <AddButton label="Add Slide" on:click={addSlide} />
    </div>
  {/if}
</div>

<style>
  .welcome-screen-section {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .enable-toggle {
    background: white;
    border: 1px solid #D4D4D4;
    border-radius: 8px;
    padding: 16px;
  }

  .toggle-label {
    display: flex;
    align-items: center;
    gap: 12px;
    cursor: pointer;
    font-weight: 500;
    color: #283618;
  }

  .toggle-label input {
    width: 20px;
    height: 20px;
  }

  .slides-section h3 {
    margin: 0 0 16px 0;
    color: #283618;
    font-size: 1rem;
  }

  .empty-message {
    color: #606060;
    font-style: italic;
    margin: 0 0 16px 0;
  }

  .slide-item {
    background: white;
    border: 1px solid #D4D4D4;
    border-radius: 8px;
    margin-bottom: 12px;
    overflow: hidden;
  }

  .slide-header {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    cursor: pointer;
    gap: 12px;
  }

  .slide-header:hover {
    background: #F0EFEB;
  }

  .slide-number {
    font-weight: 600;
    color: #283618;
    min-width: 24px;
  }

  .slide-preview {
    flex: 1;
    color: #606060;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .slide-actions {
    display: flex;
    gap: 4px;
  }

  .action-btn {
    background: none;
    border: 1px solid #D4D4D4;
    border-radius: 4px;
    width: 28px;
    height: 28px;
    cursor: pointer;
    color: #606060;
    font-size: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .action-btn:hover:not(:disabled) {
    background: #D4D4D4;
    color: #283618;
  }

  .action-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .action-btn.delete:hover {
    background: #FED7D7;
    border-color: #9B2C2C;
    color: #9B2C2C;
  }

  .slide-editor {
    padding: 16px;
    border-top: 1px solid #D4D4D4;
    background: #F0EFEB;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .field label {
    display: block;
    margin-bottom: 6px;
    font-weight: 500;
    color: #283618;
  }

  .field textarea {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid #D4D4D4;
    border-radius: 6px;
    font-family: inherit;
    font-size: 0.95rem;
    resize: vertical;
    box-sizing: border-box;
  }

  .field textarea:focus {
    outline: none;
    border-color: #283618;
  }

  .radio-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    color: #283618;
  }

  .radio-label input[type="radio"] {
    width: 18px;
    height: 18px;
  }

  .seconds-input {
    width: 60px;
    padding: 4px 8px;
    border: 1px solid #D4D4D4;
    border-radius: 4px;
    font-size: 0.9rem;
    text-align: center;
  }

  .seconds-input:focus {
    outline: none;
    border-color: #283618;
  }

  .btn {
    padding: 8px 16px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 500;
    align-self: flex-start;
  }

  .btn-secondary {
    background: #D4D4D4;
    color: #283618;
  }

  .btn-secondary:hover {
    background: #B7B7A4;
  }
</style>
```

**Step 2: Build to verify**

Run: `npm run build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add src/lib/sections/WelcomeScreenSection.svelte
git commit -m "feat: add WelcomeScreenSection component"
```

---

### Task 3: Add Welcome Screen to Sidebar Navigation

**Files:**
- Modify: `src/App.svelte`

**Step 1: Add to sections array**

Find the `sections` array and add after pets:

```typescript
{ id: 'welcome', label: 'Welcome Screen', icon: '👋' },
```

**Step 2: Update Section type**

Find the `Section` type and add `'welcome'`:

```typescript
type Section =
  | 'financial' | 'insurance' | 'bills' | 'property' | 'legal'
  | 'digital' | 'household' | 'personal' | 'contacts' | 'medical' | 'pets' | 'welcome';
```

**Step 3: Import WelcomeScreenSection**

Add to imports:

```typescript
import WelcomeScreenSection from './lib/sections/WelcomeScreenSection.svelte';
```

**Step 4: Add to content-body render**

Find the content-body section and add after the pets condition:

```svelte
{:else if currentSection === 'welcome'}
  <WelcomeScreenSection />
```

**Step 5: Build to verify**

Run: `npm run build`
Expected: Build succeeds

**Step 6: Commit**

```bash
git add src/App.svelte
git commit -m "feat: add Welcome Screen to sidebar navigation"
```

---

### Task 4: Update TypeScript Store Types

**Files:**
- Modify: `src/lib/stores/document.ts`

**Step 1: Add WelcomeScreen types to the store**

Find the document store file and add the TypeScript interfaces if not already present. Look for existing type definitions and add:

```typescript
interface MessageSlide {
  id: string;
  text: string;
  transition: { type: 'click' } | { type: 'auto'; seconds: number };
}

interface WelcomeScreen {
  enabled: boolean;
  slides: MessageSlide[];
}
```

Then ensure the document type includes `welcome_screen?: WelcomeScreen`.

**Step 2: Build to verify**

Run: `npm run build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add src/lib/stores/document.ts
git commit -m "feat: add WelcomeScreen types to document store"
```

---

### Task 5: Update HTML Export - Embed Slides Data

**Files:**
- Modify: `src-tauri/src/export.rs`

**Step 1: Add slides data to HTML template**

In the `generate_encrypted_html` function, after the encrypted data script tag, add the welcome screen data. Find where the HTML template is built and add:

```rust
// Serialize welcome screen if enabled
let welcome_screen_json = if let Some(ref ws) = doc.welcome_screen {
    if ws.enabled && !ws.slides.is_empty() {
        serde_json::to_string(&ws.slides).unwrap_or_else(|_| "[]".to_string())
    } else {
        "[]".to_string()
    }
} else {
    "[]".to_string()
};
```

Then embed this in the HTML as a script tag:

```html
<script>
const welcomeSlides = {welcome_screen_json};
</script>
```

**Step 2: Build to verify**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add src-tauri/src/export.rs
git commit -m "feat: embed welcome slides data in HTML export"
```

---

### Task 6: Update HTML Export - Slide Playback CSS

**Files:**
- Modify: `src-tauri/src/export.rs`

**Step 1: Add slide presentation CSS**

Find the CSS section in the HTML template and add:

```css
.welcome-overlay {{
    position: fixed;
    inset: 0;
    background: #F0EFEB;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
    opacity: 1;
    transition: opacity 0.5s ease;
}}

.welcome-overlay.hidden {{
    opacity: 0;
    pointer-events: none;
}}

.welcome-slide {{
    text-align: center;
    max-width: 600px;
    padding: 40px;
    opacity: 0;
    transition: opacity 0.5s ease;
}}

.welcome-slide.visible {{
    opacity: 1;
}}

.welcome-slide .message {{
    font-size: 1.5rem;
    line-height: 1.6;
    color: #283618;
    white-space: pre-wrap;
}}

.welcome-slide .continue-btn {{
    margin-top: 40px;
    padding: 12px 32px;
    background: #283618;
    color: #F0EFEB;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s ease;
}}

.welcome-slide .continue-btn:hover {{
    background: #1f2a12;
}}
```

**Step 2: Build to verify**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add src-tauri/src/export.rs
git commit -m "feat: add welcome slide CSS to HTML export"
```

---

### Task 7: Update HTML Export - Slide Playback HTML Structure

**Files:**
- Modify: `src-tauri/src/export.rs`

**Step 1: Add welcome overlay HTML**

Find where the body content starts and add before the lockScreen div:

```html
<div id="welcomeOverlay" class="welcome-overlay" style="display: none;">
    <div id="welcomeSlide" class="welcome-slide">
        <div class="message" id="slideMessage"></div>
        <button class="continue-btn" id="continueBtn" style="display: none;">Continue</button>
    </div>
</div>
```

**Step 2: Build to verify**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add src-tauri/src/export.rs
git commit -m "feat: add welcome overlay HTML structure"
```

---

### Task 8: Update HTML Export - Slide Playback JavaScript

**Files:**
- Modify: `src-tauri/src/export.rs`

**Step 1: Add slide playback JavaScript**

Find the JavaScript section and add the welcome screen logic:

```javascript
let currentSlideIndex = 0;
let slideTimer = null;

function showWelcomeScreen() {{
    if (!welcomeSlides || welcomeSlides.length === 0) {{
        showLockScreen();
        return;
    }}

    document.getElementById('welcomeOverlay').style.display = 'flex';
    document.getElementById('lockScreen').style.display = 'none';
    showSlide(0);
}}

function showSlide(index) {{
    if (index >= welcomeSlides.length) {{
        hideWelcomeScreen();
        return;
    }}

    currentSlideIndex = index;
    const slide = welcomeSlides[index];
    const slideEl = document.getElementById('welcomeSlide');
    const messageEl = document.getElementById('slideMessage');
    const continueBtn = document.getElementById('continueBtn');

    // Fade out
    slideEl.classList.remove('visible');

    setTimeout(() => {{
        messageEl.textContent = slide.text;

        if (slide.transition.type === 'click') {{
            continueBtn.style.display = 'inline-block';
            continueBtn.onclick = () => showSlide(index + 1);
        }} else {{
            continueBtn.style.display = 'none';
            if (slideTimer) clearTimeout(slideTimer);
            slideTimer = setTimeout(() => showSlide(index + 1), slide.transition.seconds * 1000);
        }}

        // Fade in
        slideEl.classList.add('visible');
    }}, 300);
}}

function hideWelcomeScreen() {{
    const overlay = document.getElementById('welcomeOverlay');
    overlay.classList.add('hidden');
    setTimeout(() => {{
        overlay.style.display = 'none';
        showLockScreen();
    }}, 500);
}}

function showLockScreen() {{
    document.getElementById('lockScreen').style.display = 'flex';
    document.getElementById('passphrase').focus();
}}
```

**Step 2: Update the initialization**

Find where the page initializes (likely `window.onload` or similar) and change it to call `showWelcomeScreen()` instead of directly showing the lock screen.

**Step 3: Build to verify**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

**Step 4: Commit**

```bash
git add src-tauri/src/export.rs
git commit -m "feat: add welcome slide playback JavaScript"
```

---

### Task 9: Update Export Dialog - Show Welcome Screen Option

**Files:**
- Modify: `src/lib/components/ExportDialog.svelte`

**Step 1: Add welcome screen checkbox**

Import the document store and add reactive variable:

```typescript
import { document } from '../stores/document';

$: welcomeScreenAvailable = $document.welcome_screen?.enabled &&
                            ($document.welcome_screen?.slides?.length || 0) > 0;
let includeWelcomeScreen = true;
```

**Step 2: Add checkbox to form**

Add before the passphrase field:

```svelte
{#if welcomeScreenAvailable}
  <label class="checkbox-field">
    <input type="checkbox" bind:checked={includeWelcomeScreen} />
    <span>Include welcome screen ({$document.welcome_screen?.slides?.length} slides)</span>
  </label>
{/if}
```

**Step 3: Build to verify**

Run: `npm run build`
Expected: Build succeeds

**Step 4: Commit**

```bash
git add src/lib/components/ExportDialog.svelte
git commit -m "feat: add welcome screen option to export dialog"
```

---

### Task 10: Pass Welcome Screen Flag to Export

**Files:**
- Modify: `src/lib/components/ExportDialog.svelte`
- Modify: `src-tauri/src/main.rs`
- Modify: `src-tauri/src/export.rs`

**Step 1: Update Tauri command to accept flag**

In `main.rs`, update the `save_export_with_dialog` function signature:

```rust
#[tauri::command]
async fn save_export_with_dialog(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    passphrase: String,
    include_welcome_screen: bool,
) -> Result<Option<String>, String> {
```

And pass the flag to the export function.

**Step 2: Update export function**

In `export.rs`, update `generate_encrypted_html` to accept the flag:

```rust
pub fn generate_encrypted_html(doc: &LegacyDocument, passphrase: &str, include_welcome_screen: bool) -> Result<String, ExportError> {
```

And conditionally include the welcome slides based on the flag.

**Step 3: Update ExportDialog to pass flag**

Update the invoke call:

```typescript
const filePath = await invoke<string | null>('save_export_with_dialog', {
  passphrase,
  includeWelcomeScreen: welcomeScreenAvailable && includeWelcomeScreen
});
```

**Step 4: Build to verify**

Run: `npm run build && cd src-tauri && cargo build`
Expected: Build succeeds

**Step 5: Commit**

```bash
git add src/lib/components/ExportDialog.svelte src-tauri/src/main.rs src-tauri/src/export.rs
git commit -m "feat: pass welcome screen flag through export flow"
```

---

### Task 11: Final Integration Test

**Step 1: Manual testing checklist**

1. Open app, navigate to Welcome Screen section
2. Toggle enable on
3. Add 3 slides with different transitions
4. Reorder slides using arrows
5. Delete a slide
6. Click Export, verify checkbox shows slide count
7. Export with welcome screen enabled
8. Open exported HTML in browser
9. Verify slides play in order
10. Verify passphrase prompt appears after slides
11. Verify document unlocks with correct passphrase

**Step 2: Final commit**

```bash
git add -A
git commit -m "feat: complete welcome screen feature implementation"
```
