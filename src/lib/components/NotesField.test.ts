import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import NotesField from './NotesField.svelte';

describe('NotesField', () => {
  it('renders with label', () => {
    render(NotesField, { props: { value: '' } });
    expect(screen.getByText('Section Notes')).toBeInTheDocument();
  });

  it('renders textarea with value', () => {
    render(NotesField, { props: { value: 'My notes here' } });
    const textarea = document.querySelector('textarea') as HTMLTextAreaElement;
    expect(textarea).toBeInTheDocument();
    expect(textarea.value).toBe('My notes here');
  });

  it('renders placeholder text', () => {
    render(NotesField, { props: { value: '' } });
    const textarea = document.querySelector('textarea') as HTMLTextAreaElement;
    expect(textarea.placeholder).toContain('notes');
  });

  it('textarea accepts user input', async () => {
    render(NotesField, { props: { value: '' } });
    const textarea = document.querySelector('textarea') as HTMLTextAreaElement;
    await fireEvent.input(textarea, { target: { value: 'Updated' } });
    expect(textarea.value).toBe('Updated');
  });
});
