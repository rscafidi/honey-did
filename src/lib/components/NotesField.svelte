<script lang="ts">
  export let value: string = '';
  export let placeholder: string = 'Add any additional notes...';
</script>

<div class="notes-field">
  <label>
    <span class="label-text">Section Notes</span>
    <textarea
      bind:value
      {placeholder}
      rows="3"
      on:change
    ></textarea>
  </label>
</div>

<style>
  .notes-field {
    margin-top: 20px;
    padding-top: 20px;
    border-top: 1px solid #D4D4D4;
  }

  .label-text {
    display: block;
    font-weight: 600;
    margin-bottom: 8px;
    color: #606060;
  }

  textarea {
    width: 100%;
    padding: 12px;
    border: 1px solid #D4D4D4;
    border-radius: 6px;
    font-family: inherit;
    font-size: 0.95rem;
    resize: vertical;
    box-sizing: border-box;
  }

  textarea:focus {
    outline: none;
    border-color: #283618;
  }
</style>
