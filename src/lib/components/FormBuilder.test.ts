import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import FormBuilder from './FormBuilder.svelte';
import type { FormElement } from '../stores/document';

// Mock svelte-dnd-action since it relies on browser DnD APIs not available in jsdom
vi.mock('svelte-dnd-action', () => ({
  dndzone: () => ({ destroy: () => {} }),
  TRIGGERS: { DRAG_STARTED: 'dragStarted' },
}));

describe('FormBuilder', () => {
  it('renders toolbar buttons', () => {
    render(FormBuilder, { props: { elements: [] } });
    expect(screen.getByText('+ Field')).toBeInTheDocument();
    expect(screen.getByText('+ Divider')).toBeInTheDocument();
    expect(screen.getByText('+ Header')).toBeInTheDocument();
  });

  it('shows empty hint when no elements', () => {
    render(FormBuilder, { props: { elements: [] } });
    expect(screen.getByText('Add fields, dividers, or headers to build your form.')).toBeInTheDocument();
  });

  it('adds a field element on "+ Field" click', async () => {
    render(FormBuilder, { props: { elements: [] } });
    await fireEvent.click(screen.getByText('+ Field'));
    // A field row should appear with a name input and type selector
    expect(document.querySelector('.el-name')).toBeInTheDocument();
    expect(document.querySelector('.el-field-type')).toBeInTheDocument();
  });

  it('adds a divider element on "+ Divider" click', async () => {
    render(FormBuilder, { props: { elements: [] } });
    await fireEvent.click(screen.getByText('+ Divider'));
    expect(document.querySelector('.divider-indicator')).toBeInTheDocument();
  });

  it('adds a header element on "+ Header" click', async () => {
    render(FormBuilder, { props: { elements: [] } });
    await fireEvent.click(screen.getByText('+ Header'));
    expect(document.querySelector('.el-header-text')).toBeInTheDocument();
  });

  it('renders existing field elements with inputs', () => {
    const elements: FormElement[] = [
      { type: 'field', id: 'f1', name: 'Email', field_type: 'text' },
    ];
    render(FormBuilder, { props: { elements } });
    const input = document.querySelector('.el-name') as HTMLInputElement;
    expect(input).toBeInTheDocument();
    expect(input.value).toBe('Email');
  });

  it('renders field type selector', () => {
    const elements: FormElement[] = [
      { type: 'field', id: 'f1', name: 'Age', field_type: 'number' },
    ];
    render(FormBuilder, { props: { elements } });
    const select = document.querySelector('.el-field-type') as HTMLSelectElement;
    expect(select).toBeInTheDocument();
    expect(select.value).toBe('number');
  });

  it('renders divider element', () => {
    const elements: FormElement[] = [
      { type: 'divider', id: 'd1' },
    ];
    render(FormBuilder, { props: { elements } });
    expect(document.querySelector('.divider-indicator')).toBeInTheDocument();
  });

  it('renders header element with input', () => {
    const elements: FormElement[] = [
      { type: 'header', id: 'h1', text: 'Contact Info' },
    ];
    render(FormBuilder, { props: { elements } });
    const input = document.querySelector('.el-header-text') as HTMLInputElement;
    expect(input).toBeInTheDocument();
    expect(input.value).toBe('Contact Info');
  });

  it('allows editing field name', async () => {
    const elements: FormElement[] = [
      { type: 'field', id: 'f1', name: 'Old Name', field_type: 'text' },
    ];
    render(FormBuilder, { props: { elements } });
    const input = document.querySelector('.el-name') as HTMLInputElement;
    await fireEvent.input(input, { target: { value: 'New Name' } });
    expect(input.value).toBe('New Name');
  });

  it('allows changing field type via select', async () => {
    const elements: FormElement[] = [
      { type: 'field', id: 'f1', name: 'Value', field_type: 'text' },
    ];
    render(FormBuilder, { props: { elements } });
    const select = document.querySelector('.el-field-type') as HTMLSelectElement;
    await fireEvent.change(select, { target: { value: 'number' } });
    expect(select.value).toBe('number');
  });

  it('removes element on delete click', async () => {
    const elements: FormElement[] = [
      { type: 'field', id: 'f1', name: 'Name', field_type: 'text' },
      { type: 'field', id: 'f2', name: 'Age', field_type: 'number' },
    ];
    render(FormBuilder, { props: { elements } });
    expect(document.querySelectorAll('.element-row')).toHaveLength(2);
    const deleteButtons = document.querySelectorAll('.btn-icon.delete');
    await fireEvent.click(deleteButtons[0]);
    // After delete, only one element should remain
    expect(document.querySelectorAll('.element-row')).toHaveLength(1);
  });

  it('appends new elements to existing list', async () => {
    const elements: FormElement[] = [
      { type: 'field', id: 'f1', name: 'Existing', field_type: 'text' },
    ];
    render(FormBuilder, { props: { elements } });
    expect(document.querySelectorAll('.element-row')).toHaveLength(1);
    await fireEvent.click(screen.getByText('+ Field'));
    expect(document.querySelectorAll('.element-row')).toHaveLength(2);
  });

  it('renders all four field type options', () => {
    const elements: FormElement[] = [
      { type: 'field', id: 'f1', name: 'Test', field_type: 'text' },
    ];
    render(FormBuilder, { props: { elements } });
    const options = document.querySelectorAll('.el-field-type option');
    const values = Array.from(options).map((o) => (o as HTMLOptionElement).value);
    expect(values).toEqual(['text', 'number', 'date', 'boolean']);
  });
});
