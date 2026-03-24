import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import AddButton from './AddButton.svelte';

describe('AddButton', () => {
  it('renders with default label', () => {
    render(AddButton);
    expect(screen.getByText('Add Item')).toBeInTheDocument();
    expect(screen.getByText('+')).toBeInTheDocument();
  });

  it('renders with custom label', () => {
    render(AddButton, { props: { label: 'Add Bank Account' } });
    expect(screen.getByText('Add Bank Account')).toBeInTheDocument();
  });

  it('button is clickable', async () => {
    render(AddButton, { props: { label: 'Add' } });
    const button = document.querySelector('.add-btn') as HTMLButtonElement;
    // Verify button exists and is enabled
    expect(button).toBeInTheDocument();
    expect(button).not.toBeDisabled();
    // Click fires without error
    await fireEvent.click(button);
  });

  it('renders as a button element', () => {
    render(AddButton);
    const button = document.querySelector('.add-btn');
    expect(button).toBeInTheDocument();
    expect(button?.tagName).toBe('BUTTON');
  });
});
