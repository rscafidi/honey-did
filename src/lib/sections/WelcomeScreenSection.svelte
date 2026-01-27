<script lang="ts">
  import { document, type MessageSlide, type WelcomeScreen, type SlideType } from '../stores/document';
  import { invoke } from '@tauri-apps/api/core';

  $: welcomeScreen = ($document?.welcome_screen || { enabled: false, slides: [], fallback_passphrase: undefined }) as WelcomeScreen;
  $: slides = welcomeScreen.slides || [];
  $: questionCount = slides.filter(s => s.type === 'question').length;
  $: hasMinQuestions = questionCount >= 2;
  $: hasMaxQuestions = questionCount >= 5;

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

  function addSlide(slideType: SlideType) {
    const newSlide: MessageSlide = {
      id: generateId(),
      type: slideType,
      text: '',
      answer: slideType === 'question' ? '' : undefined,
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
    return text.length > 35 ? text.substring(0, 35) + '...' : text;
  }

  function updateFallbackPassphrase(value: string) {
    saveWelcomeScreen({
      ...welcomeScreen,
      fallback_passphrase: value || undefined,
    });
  }

  async function generatePassphrase() {
    try {
      const passphrase = await invoke<string>('generate_passphrase');
      updateFallbackPassphrase(passphrase);
    } catch (e) {
      console.error('Failed to generate passphrase:', e);
    }
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
        <p class="empty-message">No slides yet. Add messages and questions to create your welcome experience.</p>
      {/if}

      {#each slides as slide, index (slide.id)}
        <div class="slide-item" class:question={slide.type === 'question'}>
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <div class="slide-header" on:click={() => (editingSlideId = editingSlideId === slide.id ? null : slide.id)}>
            <span class="slide-icon">{slide.type === 'question' ? '?' : '#'}</span>
            <span class="slide-number">{index + 1}.</span>
            <span class="slide-preview">"{getSlidePreview(slide)}"</span>
            {#if slide.type === 'question' && slide.answer}
              <span class="answer-preview">Answer: {slide.answer}</span>
            {/if}
            <div class="slide-actions">
              <button
                class="action-btn"
                on:click|stopPropagation={() => moveSlide(slide.id, 'up')}
                disabled={index === 0}
                title="Move up"
              >^</button>
              <button
                class="action-btn"
                on:click|stopPropagation={() => moveSlide(slide.id, 'down')}
                disabled={index === slides.length - 1}
                title="Move down"
              >v</button>
              <button
                class="action-btn delete"
                on:click|stopPropagation={() => deleteSlide(slide.id)}
                title="Delete"
              >x</button>
            </div>
          </div>

          {#if editingSlideId === slide.id}
            <div class="slide-editor">
              <div class="field">
                <label for="slide-text-{slide.id}">{slide.type === 'question' ? 'Question' : 'Message'}</label>
                <textarea
                  id="slide-text-{slide.id}"
                  value={slide.text}
                  on:input={(e) => updateSlide(slide.id, { text: e.currentTarget.value })}
                  placeholder={slide.type === 'question' ? 'Enter your question...' : 'Enter your message...'}
                  rows="3"
                ></textarea>
              </div>

              {#if slide.type === 'question'}
                <div class="field">
                  <label for="slide-answer-{slide.id}">Expected Answer</label>
                  <input
                    type="text"
                    id="slide-answer-{slide.id}"
                    value={slide.answer || ''}
                    on:input={(e) => updateSlide(slide.id, { answer: e.currentTarget.value.toLowerCase().trim() })}
                    placeholder="Enter the expected answer..."
                  />
                  <span class="field-hint">Answers are case-insensitive</span>
                </div>
              {/if}

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

      <div class="add-buttons">
        <button class="btn btn-add" on:click={() => addSlide('message')}>+ Add Message</button>
        <button class="btn btn-add" on:click={() => addSlide('question')} disabled={hasMaxQuestions}>+ Add Question</button>
      </div>

      <div class="question-counter" class:warning={!hasMinQuestions} class:ok={hasMinQuestions}>
        Questions: {questionCount} of 2-5 required
      </div>
    </div>

    <div class="fallback-section">
      <h3>Fallback Passphrase (optional)</h3>
      <div class="passphrase-input-row">
        <input
          type="text"
          value={welcomeScreen.fallback_passphrase || ''}
          on:input={(e) => updateFallbackPassphrase(e.currentTarget.value)}
          placeholder="Enter a fallback passphrase..."
        />
        <button class="btn btn-secondary" on:click={generatePassphrase}>Generate</button>
      </div>
      {#if !welcomeScreen.fallback_passphrase}
        <p class="warning-message">Without a passphrase, there's no recovery if the recipient forgets the answers.</p>
      {:else}
        <p class="success-message">Fallback passphrase set</p>
      {/if}
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

  .slides-section h3,
  .fallback-section h3 {
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

  .slide-item.question {
    border-left: 4px solid #606C38;
  }

  .slide-header {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    cursor: pointer;
    gap: 8px;
  }

  .slide-header:hover {
    background: #F0EFEB;
  }

  .slide-icon {
    font-weight: 700;
    color: #606C38;
    font-size: 1.1rem;
    min-width: 20px;
    text-align: center;
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

  .answer-preview {
    color: #606C38;
    font-size: 0.85rem;
    margin-left: 8px;
    padding: 2px 8px;
    background: #E8EDE0;
    border-radius: 4px;
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

  .field textarea,
  .field input[type="text"] {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid #D4D4D4;
    border-radius: 6px;
    font-family: inherit;
    font-size: 0.95rem;
    box-sizing: border-box;
  }

  .field textarea {
    resize: vertical;
  }

  .field textarea:focus,
  .field input[type="text"]:focus {
    outline: none;
    border-color: #283618;
  }

  .field-hint {
    display: block;
    margin-top: 4px;
    font-size: 0.85rem;
    color: #606060;
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
  }

  .btn-secondary {
    background: #D4D4D4;
    color: #283618;
  }

  .btn-secondary:hover {
    background: #B7B7A4;
  }

  .btn-add {
    background: white;
    border: 1px dashed #D4D4D4;
    color: #606060;
  }

  .btn-add:hover:not(:disabled) {
    border-color: #283618;
    color: #283618;
  }

  .btn-add:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .add-buttons {
    display: flex;
    gap: 12px;
    margin-top: 8px;
  }

  .question-counter {
    margin-top: 12px;
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 0.9rem;
  }

  .question-counter.warning {
    background: #FEF3C7;
    color: #92400E;
  }

  .question-counter.ok {
    background: #E8EDE0;
    color: #606C38;
  }

  .fallback-section {
    background: white;
    border: 1px solid #D4D4D4;
    border-radius: 8px;
    padding: 16px;
  }

  .passphrase-input-row {
    display: flex;
    gap: 12px;
  }

  .passphrase-input-row input {
    flex: 1;
    padding: 10px 12px;
    border: 1px solid #D4D4D4;
    border-radius: 6px;
    font-family: inherit;
    font-size: 0.95rem;
  }

  .passphrase-input-row input:focus {
    outline: none;
    border-color: #283618;
  }

  .warning-message {
    margin: 12px 0 0 0;
    padding: 8px 12px;
    background: #FEF3C7;
    border-radius: 6px;
    color: #92400E;
    font-size: 0.9rem;
  }

  .success-message {
    margin: 12px 0 0 0;
    padding: 8px 12px;
    background: #E8EDE0;
    border-radius: 6px;
    color: #606C38;
    font-size: 0.9rem;
  }
</style>
