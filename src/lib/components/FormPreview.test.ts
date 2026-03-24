import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import FormPreview from './FormPreview.svelte';
import type { FormElement, CustomItem } from '../stores/document';

describe('FormPreview', () => {
  const fieldElements: FormElement[] = [
    { type: 'field', id: 'name', name: 'Name', field_type: 'text' },
    { type: 'field', id: 'amount', name: 'Amount', field_type: 'number' },
    { type: 'field', id: 'date', name: 'Due Date', field_type: 'date' },
    { type: 'field', id: 'active', name: 'Active', field_type: 'boolean' },
  ];

  const fieldWithDividerAndHeader: FormElement[] = [
    { type: 'header', id: 'h1', text: 'Basic Info' },
    { type: 'field', id: 'name', name: 'Name', field_type: 'text' },
    { type: 'divider', id: 'd1' },
    { type: 'field', id: 'amount', name: 'Amount', field_type: 'number' },
  ];

  // --- Empty states ---
  describe('empty state', () => {
    it('shows prompt to add fields when no fields defined', () => {
      render(FormPreview, { props: { formElements: [], items: [] } });
      expect(screen.getByText('Add fields in the form builder, then add items.')).toBeInTheDocument();
    });

    it('shows prompt to add items when fields exist but no items', () => {
      render(FormPreview, { props: { formElements: fieldElements, items: [] } });
      expect(screen.getByText('No items yet. Click "+ Add Item" to add your first.')).toBeInTheDocument();
    });
  });

  // --- Display mode ---
  describe('display mode', () => {
    it('renders item with field values', () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Test Item', amount: '100', date: '2024-01-15', active: 'true' } },
      ];
      render(FormPreview, { props: { formElements: fieldElements, items } });
      expect(screen.getByText('Test Item')).toBeInTheDocument();
      expect(screen.getByText('100')).toBeInTheDocument();
      expect(screen.getByText('2024-01-15')).toBeInTheDocument();
      expect(screen.getByText('Yes')).toBeInTheDocument();
    });

    it('renders boolean false as No', () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Item', active: 'false' } },
      ];
      render(FormPreview, { props: { formElements: fieldElements, items } });
      expect(screen.getByText('No')).toBeInTheDocument();
    });

    it('shows Edit button for each item', () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Item 1' } },
        { id: 'i2', values: { name: 'Item 2' } },
      ];
      render(FormPreview, { props: { formElements: fieldElements, items } });
      expect(screen.getAllByText('Edit')).toHaveLength(2);
    });

    it('shows notes when present', () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Item', _notes: 'Important note here' } },
      ];
      render(FormPreview, { props: { formElements: fieldElements, items } });
      expect(screen.getByText('Important note here')).toBeInTheDocument();
    });

    it('skips empty field values in display', () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Item', amount: '' } },
      ];
      render(FormPreview, { props: { formElements: fieldElements, items } });
      expect(screen.getByText('Item')).toBeInTheDocument();
      expect(screen.queryByText('Amount:')).not.toBeInTheDocument();
    });

    it('renders headers and dividers in display', () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Test', amount: '50' } },
      ];
      render(FormPreview, { props: { formElements: fieldWithDividerAndHeader, items } });
      expect(screen.getByText('Basic Info')).toBeInTheDocument();
    });
  });

  // --- Edit mode ---
  describe('edit mode', () => {
    it('enters edit mode on Edit click', async () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Item' } },
      ];
      render(FormPreview, { props: { formElements: fieldElements, items } });
      await fireEvent.click(screen.getByText('Edit'));
      expect(screen.getByText('Save')).toBeInTheDocument();
      expect(screen.getByText('Cancel')).toBeInTheDocument();
    });

    it('shows form fields for all element types in edit mode', async () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Test', amount: '50', date: '2024-06-01', active: 'true' } },
      ];
      render(FormPreview, { props: { formElements: fieldElements, items } });
      await fireEvent.click(screen.getByText('Edit'));

      const nameInput = document.querySelector('#field-name') as HTMLInputElement;
      expect(nameInput).toBeInTheDocument();
      expect(nameInput.type).toBe('text');

      const amountInput = document.querySelector('#field-amount') as HTMLInputElement;
      expect(amountInput).toBeInTheDocument();
      expect(amountInput.type).toBe('number');

      const dateInput = document.querySelector('#field-date') as HTMLInputElement;
      expect(dateInput).toBeInTheDocument();
      expect(dateInput.type).toBe('date');

      const checkbox = screen.getByRole('checkbox');
      expect(checkbox).toBeInTheDocument();
      expect(checkbox).toBeChecked();
    });

    it('renders Notes textarea in edit mode', async () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Test', _notes: 'My notes' } },
      ];
      render(FormPreview, { props: { formElements: fieldElements, items } });
      await fireEvent.click(screen.getByText('Edit'));

      const notesArea = document.querySelector('#notes-i1') as HTMLTextAreaElement;
      expect(notesArea).toBeInTheDocument();
      expect(notesArea.value).toBe('My notes');
    });

    it('allows editing field values in edit mode', async () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Old Name', amount: '' } },
      ];
      render(FormPreview, { props: { formElements: fieldElements, items } });

      await fireEvent.click(screen.getByText('Edit'));
      const nameInput = document.querySelector('#field-name') as HTMLInputElement;
      await fireEvent.input(nameInput, { target: { value: 'New Name' } });
      expect(nameInput.value).toBe('New Name');

      // Save returns to display mode
      await fireEvent.click(screen.getByText('Save'));
      expect(screen.queryByText('Save')).not.toBeInTheDocument();
    });

    it('reverts changes on Cancel', async () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Original' } },
      ];
      render(FormPreview, { props: { formElements: fieldElements, items } });
      await fireEvent.click(screen.getByText('Edit'));

      const nameInput = document.querySelector('#field-name') as HTMLInputElement;
      await fireEvent.input(nameInput, { target: { value: 'Changed' } });
      await fireEvent.click(screen.getByText('Cancel'));

      expect(screen.getByText('Original')).toBeInTheDocument();
      expect(screen.queryByText('Save')).not.toBeInTheDocument();
    });

    it('renders headers and dividers in edit form', async () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Test', amount: '50' } },
      ];
      render(FormPreview, { props: { formElements: fieldWithDividerAndHeader, items } });
      await fireEvent.click(screen.getByText('Edit'));
      expect(screen.getByText('Basic Info')).toBeInTheDocument();
    });
  });

  // --- Delete flow ---
  describe('delete', () => {
    it('shows delete confirmation when Delete clicked in edit mode', async () => {
      const textOnlyElements: FormElement[] = [
        { type: 'field', id: 'name', name: 'Name', field_type: 'text' },
      ];
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Item' } },
      ];
      render(FormPreview, { props: { formElements: textOnlyElements, items } });
      await fireEvent.click(screen.getByText('Edit'));
      await fireEvent.click(screen.getByText('Delete'));

      expect(screen.getByText('Delete?')).toBeInTheDocument();
      expect(screen.getByText('Yes')).toBeInTheDocument();
      expect(screen.getByText('No')).toBeInTheDocument();
    });

    it('confirm delete flow completes without error', async () => {
      const textOnlyElements: FormElement[] = [
        { type: 'field', id: 'name', name: 'Name', field_type: 'text' },
      ];
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Item' } },
      ];
      render(FormPreview, { props: { formElements: textOnlyElements, items } });

      await fireEvent.click(screen.getByText('Edit'));
      await fireEvent.click(screen.getByText('Delete'));
      expect(screen.getByText('Delete?')).toBeInTheDocument();
      await fireEvent.click(screen.getByText('Yes'));
      // Delete dispatches updateItems to parent; in isolation the edit mode exits
      expect(screen.queryByText('Delete?')).not.toBeInTheDocument();
    });

    it('cancels delete on No', async () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'Item' } },
      ];
      render(FormPreview, { props: { formElements: fieldElements, items } });
      await fireEvent.click(screen.getByText('Edit'));
      await fireEvent.click(screen.getByText('Delete'));
      await fireEvent.click(screen.getByText('No'));

      expect(screen.queryByText('Delete?')).not.toBeInTheDocument();
      expect(screen.getByText('Save')).toBeInTheDocument();
    });
  });

  // --- addItem method ---
  describe('addItem', () => {
    it('addItem is callable on the component', () => {
      const { component } = render(FormPreview, { props: { formElements: fieldElements, items: [] } });
      // addItem dispatches to parent; we verify it's callable and doesn't throw
      expect(() => component.addItem()).not.toThrow();
    });

    it('addItem does nothing when no fields defined', () => {
      const { component } = render(FormPreview, { props: { formElements: [], items: [] } });
      component.addItem();
      // Should still show empty state
      expect(screen.getByText('Add fields in the form builder, then add items.')).toBeInTheDocument();
    });

    it('renders items with all field types when items are provided', () => {
      const items: CustomItem[] = [{
        id: 'i1',
        values: { name: 'Test', amount: '42', date: '2024-01-01', active: 'false' },
      }];
      render(FormPreview, { props: { formElements: fieldElements, items } });
      expect(screen.getByText('Test')).toBeInTheDocument();
      expect(screen.getByText('42')).toBeInTheDocument();
      expect(screen.getByText('No')).toBeInTheDocument(); // boolean false
    });

    it('renders boolean true as Yes', () => {
      const boolElements: FormElement[] = [
        { type: 'field', id: 'active', name: 'Active', field_type: 'boolean' },
      ];
      const items: CustomItem[] = [{ id: 'i1', values: { active: 'true' } }];
      render(FormPreview, { props: { formElements: boolElements, items } });
      expect(screen.getByText('Yes')).toBeInTheDocument();
    });
  });

  // --- Multiple items ---
  describe('multiple items', () => {
    it('renders all items', () => {
      const items: CustomItem[] = [
        { id: 'i1', values: { name: 'First' } },
        { id: 'i2', values: { name: 'Second' } },
        { id: 'i3', values: { name: 'Third' } },
      ];
      render(FormPreview, { props: { formElements: fieldElements, items } });
      expect(screen.getByText('First')).toBeInTheDocument();
      expect(screen.getByText('Second')).toBeInTheDocument();
      expect(screen.getByText('Third')).toBeInTheDocument();
    });
  });
});
