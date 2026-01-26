<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import WizardStep from './WizardStep.svelte';
  import { prioritySteps, secondarySteps } from './wizardContent';
  import FinancialSection from '../sections/FinancialSection.svelte';
  import InsuranceSection from '../sections/InsuranceSection.svelte';
  import LegalSection from '../sections/LegalSection.svelte';
  import MedicalSection from '../sections/MedicalSection.svelte';
  import ContactsSection from '../sections/ContactsSection.svelte';
  import BillsSection from '../sections/BillsSection.svelte';
  import PropertySection from '../sections/PropertySection.svelte';
  import DigitalSection from '../sections/DigitalSection.svelte';
  import HouseholdSection from '../sections/HouseholdSection.svelte';
  import PersonalSection from '../sections/PersonalSection.svelte';
  import PetsSection from '../sections/PetsSection.svelte';

  const dispatch = createEventDispatcher();

  let currentStepIndex = 0;
  let phase: 'priority' | 'askContinue' | 'secondary' | 'complete' = 'priority';

  $: steps = phase === 'secondary' ? secondarySteps : prioritySteps;
  $: currentStep = steps[currentStepIndex];
  $: totalSteps = steps.length;

  function next() {
    if (currentStepIndex < steps.length - 1) {
      currentStepIndex++;
    } else if (phase === 'priority') {
      phase = 'askContinue';
    } else if (phase === 'secondary') {
      phase = 'complete';
    }
  }

  function back() {
    if (currentStepIndex > 0) {
      currentStepIndex--;
    }
  }

  function skip() {
    next();
  }

  function continueWithMore() {
    phase = 'secondary';
    currentStepIndex = 0;
  }

  function finishSetup() {
    phase = 'complete';
  }

  function exitWizard() {
    dispatch('exit');
  }

  function startOver() {
    phase = 'priority';
    currentStepIndex = 0;
  }
</script>

<div class="wizard">
  <header class="wizard-header">
    <h1>Guided Setup</h1>
    <button class="exit-link" on:click={exitWizard}>Exit to Full View</button>
  </header>

  <main class="wizard-content">
    {#if phase === 'askContinue'}
      <div class="continue-prompt">
        <div class="prompt-icon">🎉</div>
        <h2>Great progress!</h2>
        <p>You've completed the 5 most important categories. Would you like to continue with additional categories?</p>
        <div class="continue-actions">
          <button class="btn btn-secondary" on:click={finishSetup}>Finish & Review</button>
          <button class="btn btn-primary" on:click={continueWithMore}>Continue with More Categories</button>
        </div>
      </div>
    {:else if phase === 'complete'}
      <div class="complete-prompt">
        <div class="prompt-icon">✅</div>
        <h2>Setup Complete!</h2>
        <p>You've finished the guided setup. Your information has been saved automatically.</p>
        <p>You can always come back to add more details or update information.</p>
        <div class="complete-actions">
          <button class="btn btn-secondary" on:click={startOver}>Start Over</button>
          <button class="btn btn-primary" on:click={exitWizard}>Go to Full View</button>
        </div>
      </div>
    {:else}
      <WizardStep step={currentStep} currentStep={currentStepIndex + 1} totalSteps={totalSteps}>
        {#if currentStep.id === 'financial'}
          <FinancialSection />
        {:else if currentStep.id === 'insurance'}
          <InsuranceSection />
        {:else if currentStep.id === 'legal'}
          <LegalSection />
        {:else if currentStep.id === 'medical'}
          <MedicalSection />
        {:else if currentStep.id === 'contacts'}
          <ContactsSection />
        {:else if currentStep.id === 'bills'}
          <BillsSection />
        {:else if currentStep.id === 'property'}
          <PropertySection />
        {:else if currentStep.id === 'digital'}
          <DigitalSection />
        {:else if currentStep.id === 'household'}
          <HouseholdSection />
        {:else if currentStep.id === 'personal'}
          <PersonalSection />
        {:else if currentStep.id === 'pets'}
          <PetsSection />
        {/if}
      </WizardStep>
    {/if}
  </main>

  {#if phase !== 'askContinue' && phase !== 'complete'}
    <footer class="wizard-footer">
      <div class="footer-left">
        {#if currentStepIndex > 0}
          <button class="btn btn-secondary" on:click={back}>Back</button>
        {/if}
      </div>
      <div class="footer-right">
        <button class="skip-link" on:click={skip}>Skip this section</button>
        <button class="btn btn-primary" on:click={next}>
          {currentStepIndex === steps.length - 1 ? (phase === 'priority' ? 'Continue' : 'Finish') : 'Next'}
        </button>
      </div>
    </footer>
  {/if}
</div>

<style>
  .wizard {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #f5f5f5;
  }

  .wizard-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    background: white;
    border-bottom: 1px solid #e0e0e0;
  }

  .wizard-header h1 {
    margin: 0;
    font-size: 1.25rem;
    color: #333;
  }

  .exit-link {
    background: none;
    border: none;
    color: #1976d2;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .exit-link:hover {
    text-decoration: underline;
  }

  .wizard-content {
    flex: 1;
    overflow-y: auto;
    padding: 24px;
  }

  .wizard-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    background: white;
    border-top: 1px solid #e0e0e0;
  }

  .footer-left, .footer-right {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .skip-link {
    background: none;
    border: none;
    color: #666;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .skip-link:hover {
    color: #333;
  }

  .btn {
    padding: 10px 20px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.95rem;
  }

  .btn-primary {
    background: #1976d2;
    color: white;
  }

  .btn-primary:hover {
    background: #1565c0;
  }

  .btn-secondary {
    background: #e0e0e0;
    color: #333;
  }

  .btn-secondary:hover {
    background: #d0d0d0;
  }

  .continue-prompt, .complete-prompt {
    text-align: center;
    max-width: 500px;
    margin: 60px auto;
    padding: 40px;
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }

  .prompt-icon {
    font-size: 4rem;
    margin-bottom: 16px;
  }

  .continue-prompt h2, .complete-prompt h2 {
    margin: 0 0 16px 0;
    color: #333;
  }

  .continue-prompt p, .complete-prompt p {
    margin: 0 0 24px 0;
    color: #666;
    line-height: 1.5;
  }

  .continue-actions, .complete-actions {
    display: flex;
    justify-content: center;
    gap: 12px;
  }
</style>
