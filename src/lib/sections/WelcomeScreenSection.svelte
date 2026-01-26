<script lang="ts">
  import { document } from '../stores/document';
  import AddButton from '../components/AddButton.svelte';

  interface MessageSlide {
    id: string;
    text: string;
    transition: { type: 'click' } | { type: 'auto'; seconds: number };
  }

  interface WelcomeScreen {
    enabled: boolean;
    slides: MessageSlide[];
  }

  $: welcomeScreen = ($document?.welcome_screen || { enabled: false, slides: [] }) as WelcomeScreen;
  $: slides = welcomeScreen.slides || [];

  let editingSlideId: string | null = null;

  function generateId(): string {
    return Math.random().toString(36).substring(2, 9);
  }

  function saveWelcomeScreen(newWelcomeScreen: WelcomeScreen) {
    document.updateSection('welcome_screen', newWelcomeScreen);
  }

  function toggleEnabled() {
    saveWelcomeScreen({
      ...welcomeScreen,
      enabled: !welcomeScreen.enabled,
    });
  }

  function addSlide() {
    const newSlide: MessageSlide = {
      id: generateId(),
      text: '',
      transition: { type: 'click' },
    };
    saveWelcomeScreen({
      ...welcomeScreen,
      slides: [...slides, newSlide],
    });
    editingSlideId = newSlide.id;
  }

  function updateSlide(id: string, updates: Partial<MessageSlide>) {
    saveWelcomeScreen({
      ...welcomeScreen,
      slides: slides.map((s) => (s.id === id ? { ...s, ...updates } : s)),
    });
  }

  function deleteSlide(id: string) {
    saveWelcomeScreen({
      ...welcomeScreen,
      slides: slides.filter((s) => s.id !== id),
    });
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

    saveWelcomeScreen({
      ...welcomeScreen,
      slides: newSlides,
    });
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
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <!-- svelte-ignore a11y-no-static-element-interactions -->
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
