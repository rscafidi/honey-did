<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let title: string = '';
  export let editable: boolean = true;

  const dispatch = createEventDispatcher();
</script>

<div class="item-card">
  <div class="item-header">
    <span class="item-title">{title || 'Untitled'}</span>
    {#if editable}
      <button class="delete-btn" on:click={() => dispatch('delete')} title="Delete">×</button>
    {/if}
  </div>
  <div class="item-content">
    <slot />
  </div>
</div>

<style>
  .item-card {
    background: white;
    border: 1px solid #D4D4D4;
    border-radius: 8px;
    margin-bottom: 12px;
  }

  .item-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: #F0EFEB;
    border-bottom: 1px solid #D4D4D4;
    border-radius: 8px 8px 0 0;
  }

  .item-title {
    font-weight: 600;
    color: #283618;
  }

  .delete-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: #B7B7A4;
    cursor: pointer;
    padding: 0;
    line-height: 1;
    transition: color 0.15s ease;
  }

  .delete-btn:hover {
    color: #9B2C2C;
  }

  .item-content {
    padding: 16px;
  }
</style>
