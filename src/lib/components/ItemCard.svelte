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
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    margin-bottom: 12px;
  }

  .item-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: #f8f9fa;
    border-bottom: 1px solid #e0e0e0;
    border-radius: 8px 8px 0 0;
  }

  .item-title {
    font-weight: 600;
    color: #333;
  }

  .delete-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: #999;
    cursor: pointer;
    padding: 0;
    line-height: 1;
  }

  .delete-btn:hover {
    color: #dc3545;
  }

  .item-content {
    padding: 16px;
  }
</style>
