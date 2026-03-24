import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import FormField from './FormField.svelte';

describe('FormField', () => {
  // --- Text input ---
  describe('text input', () => {
    it('renders a text input with label', () => {
      render(FormField, { props: { label: 'Account Name', value: '' } });
      expect(screen.getByText('Account Name')).toBeInTheDocument();
      expect(screen.getByRole('textbox')).toBeInTheDocument();
    });

    it('displays the initial value', () => {
      render(FormField, { props: { label: 'Name', value: 'John Doe' } });
      expect(screen.getByRole('textbox')).toHaveValue('John Doe');
    });

    it('renders with placeholder', () => {
      render(FormField, { props: { label: 'Type', value: '', placeholder: '401k, IRA, etc.' } });
      expect(screen.getByPlaceholderText('401k, IRA, etc.')).toBeInTheDocument();
    });

    it('updates value on user input', async () => {
      render(FormField, { props: { label: 'Name', value: '' } });
      const input = screen.getByRole('textbox');
      await fireEvent.input(input, { target: { value: 'New Value' } });
      expect(input).toHaveValue('New Value');
    });

    it('responds to change events', async () => {
      render(FormField, { props: { label: 'Name', value: '' } });
      const input = screen.getByRole('textbox');
      // Simulate full user interaction
      await fireEvent.input(input, { target: { value: 'Test' } });
      await fireEvent.change(input);
      expect(input).toHaveValue('Test');
    });
  });

  // --- Textarea ---
  describe('textarea', () => {
    it('renders a textarea when type is textarea', () => {
      render(FormField, { props: { label: 'Notes', value: '', type: 'textarea' } });
      expect(screen.getByText('Notes')).toBeInTheDocument();
      const textarea = document.querySelector('textarea');
      expect(textarea).toBeInTheDocument();
    });

    it('displays textarea value', () => {
      render(FormField, { props: { label: 'Notes', value: 'Some notes', type: 'textarea' } });
      const textarea = document.querySelector('textarea') as HTMLTextAreaElement;
      expect(textarea.value).toBe('Some notes');
    });

    it('updates textarea on user input', async () => {
      render(FormField, { props: { label: 'Notes', value: '', type: 'textarea' } });
      const textarea = document.querySelector('textarea') as HTMLTextAreaElement;
      await fireEvent.input(textarea, { target: { value: 'Updated notes' } });
      expect(textarea.value).toBe('Updated notes');
    });
  });

  // --- Checkbox ---
  describe('checkbox', () => {
    it('renders a checkbox when type is checkbox', () => {
      render(FormField, { props: { label: 'Autopay', type: 'checkbox', checked: false } });
      expect(screen.getByText('Autopay')).toBeInTheDocument();
      expect(screen.getByRole('checkbox')).toBeInTheDocument();
    });

    it('renders checked state', () => {
      render(FormField, { props: { label: 'Autopay', type: 'checkbox', checked: true } });
      expect(screen.getByRole('checkbox')).toBeChecked();
    });

    it('renders unchecked state', () => {
      render(FormField, { props: { label: 'Autopay', type: 'checkbox', checked: false } });
      expect(screen.getByRole('checkbox')).not.toBeChecked();
    });

    it('toggles on click', async () => {
      render(FormField, { props: { label: 'Autopay', type: 'checkbox', checked: false } });
      const checkbox = screen.getByRole('checkbox');
      await fireEvent.click(checkbox);
      expect(checkbox).toBeChecked();
    });
  });

  // --- CSS classes ---
  describe('styling', () => {
    it('applies checkbox class for checkbox type', () => {
      render(FormField, { props: { label: 'Toggle', type: 'checkbox', checked: false } });
      const wrapper = document.querySelector('.form-field');
      expect(wrapper).toHaveClass('checkbox');
    });

    it('does not apply checkbox class for text type', () => {
      render(FormField, { props: { label: 'Name', value: '' } });
      const wrapper = document.querySelector('.form-field');
      expect(wrapper).not.toHaveClass('checkbox');
    });
  });
});
