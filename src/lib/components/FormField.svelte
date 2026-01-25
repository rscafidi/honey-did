<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let label: string;
  export let value: string = '';
  export let type: 'text' | 'textarea' | 'checkbox' = 'text';
  export let placeholder: string = '';
  export let checked: boolean = false;

  const dispatch = createEventDispatcher<{
    change: { value: string; checked?: boolean };
  }>();

  function handleChange(e: Event) {
    const target = e.target as HTMLInputElement | HTMLTextAreaElement;
    if (type === 'checkbox') {
      dispatch('change', { value: '', checked: (target as HTMLInputElement).checked });
    } else {
      dispatch('change', { value: target.value });
    }
  }
</script>

<div class="form-field" class:checkbox={type === 'checkbox'}>
  <label>
    {#if type === 'checkbox'}
      <input type="checkbox" bind:checked on:change={handleChange} />
      <span>{label}</span>
    {:else}
      <span class="label-text">{label}</span>
      {#if type === 'textarea'}
        <textarea bind:value {placeholder} rows="2" on:change={handleChange}></textarea>
      {:else}
        <input type="text" bind:value {placeholder} on:change={handleChange} />
      {/if}
    {/if}
  </label>
</div>

<style>
  .form-field {
    margin-bottom: 12px;
  }

  .label-text {
    display: block;
    font-size: 0.85rem;
    color: #666;
    margin-bottom: 4px;
  }

  input[type="text"], textarea {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.95rem;
    box-sizing: border-box;
  }

  input:focus, textarea:focus {
    outline: none;
    border-color: #1976d2;
  }

  .checkbox label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
  }

  .checkbox input {
    width: 18px;
    height: 18px;
  }
</style>
