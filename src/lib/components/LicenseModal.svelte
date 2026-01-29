<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let isOpen = false;

  const dispatch = createEventDispatcher();

  function close() {
    dispatch('close');
  }
</script>

{#if isOpen}
  <div class="overlay" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()} role="presentation">
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <div class="dialog" role="dialog" aria-modal="true" on:click|stopPropagation on:keydown|stopPropagation>
      <h2>MIT License</h2>
      <div class="license-text">
        <p>Copyright (c) 2025 scafidi.dev</p>
        <p>
          Permission is hereby granted, free of charge, to any person obtaining a copy
          of this software and associated documentation files (the "Software"), to deal
          in the Software without restriction, including without limitation the rights
          to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
          copies of the Software, and to permit persons to whom the Software is
          furnished to do so, subject to the following conditions:
        </p>
        <p>
          The above copyright notice and this permission notice shall be included in all
          copies or substantial portions of the Software.
        </p>
        <p>
          THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
          IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
          FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
          AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
          LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
          OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
          SOFTWARE.
        </p>
      </div>
      <div class="actions">
        <button class="btn" on:click={close}>Close</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1100;
  }

  .dialog {
    background: var(--bg-secondary);
    border-radius: 12px;
    padding: 24px;
    width: 100%;
    max-width: 520px;
    max-height: 80vh;
    overflow-y: auto;
    box-shadow: var(--card-shadow);
  }

  h2 {
    margin: 0 0 16px 0;
    color: var(--text-primary);
    font-weight: 600;
  }

  .license-text {
    font-size: 0.85rem;
    line-height: 1.6;
    color: var(--text-secondary);
  }

  .license-text p {
    margin: 0 0 12px 0;
  }

  .license-text p:first-child {
    font-weight: 600;
    color: var(--text-primary);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    margin-top: 20px;
  }

  .btn {
    padding: 10px 20px;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    cursor: pointer;
    font-weight: 500;
    background: var(--bg-tertiary);
    color: var(--text-primary);
    transition: background 0.15s ease;
  }

  .btn:hover {
    background: var(--border-color);
  }
</style>
