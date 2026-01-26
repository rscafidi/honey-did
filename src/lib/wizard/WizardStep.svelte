<script lang="ts">
  import type { WizardStepContent } from './wizardContent';

  export let step: WizardStepContent;
  export let currentStep: number;
  export let totalSteps: number;
</script>

<div class="wizard-step">
  <div class="step-header">
    <div class="progress">Step {currentStep} of {totalSteps} - {step.title}</div>
    <div class="step-icon">{step.icon}</div>
    <h2>{step.title}</h2>
  </div>

  <div class="why-it-matters">
    <h3>Why this matters</h3>
    <p>{step.whyItMatters}</p>
  </div>

  <div class="form-area">
    <slot />
  </div>

  <div class="prompts">
    <h3>Things to consider</h3>
    <ul>
      {#each step.prompts as prompt}
        <li>{prompt}</li>
      {/each}
    </ul>
  </div>

  <div class="example">
    <h3>Example entry</h3>
    <p class="example-text">"{step.example}"</p>
  </div>
</div>

<style>
  .wizard-step {
    max-width: 800px;
    margin: 0 auto;
  }

  .step-header {
    text-align: center;
    margin-bottom: 24px;
  }

  .progress {
    font-size: 0.9rem;
    color: #606060;
    margin-bottom: 8px;
  }

  .step-icon {
    font-size: 3rem;
    margin-bottom: 8px;
  }

  h2 {
    margin: 0;
    color: #283618;
    font-weight: 600;
  }

  .why-it-matters {
    background: #DDE5B6;
    padding: 16px 20px;
    border-radius: 8px;
    margin-bottom: 24px;
  }

  .why-it-matters h3 {
    margin: 0 0 8px 0;
    font-size: 0.95rem;
    color: #283618;
    font-weight: 600;
  }

  .why-it-matters p {
    margin: 0;
    color: #283618;
    line-height: 1.5;
  }

  .form-area {
    background: white;
    border: 1px solid #D4D4D4;
    border-radius: 8px;
    padding: 20px;
    margin-bottom: 24px;
  }

  .prompts {
    background: #FDF6E3;
    padding: 16px 20px;
    border-radius: 8px;
    margin-bottom: 24px;
  }

  .prompts h3 {
    margin: 0 0 12px 0;
    font-size: 0.95rem;
    color: #856404;
    font-weight: 600;
  }

  .prompts ul {
    margin: 0;
    padding-left: 20px;
  }

  .prompts li {
    margin: 8px 0;
    color: #283618;
  }

  .example {
    background: #F0EFEB;
    padding: 16px 20px;
    border-radius: 8px;
  }

  .example h3 {
    margin: 0 0 8px 0;
    font-size: 0.95rem;
    color: #606060;
    font-weight: 600;
  }

  .example-text {
    margin: 0;
    font-style: italic;
    color: #283618;
  }
</style>
