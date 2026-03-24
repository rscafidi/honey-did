import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import ItemCard from './ItemCard.svelte';

describe('ItemCard', () => {
  it('renders with title', () => {
    render(ItemCard, { props: { title: 'Bank Account #1' } });
    expect(screen.getByText('Bank Account #1')).toBeInTheDocument();
  });

  it('shows delete button by default', () => {
    render(ItemCard, { props: { title: 'Test' } });
    const deleteBtn = document.querySelector('.delete-btn');
    expect(deleteBtn).toBeInTheDocument();
  });

  it('delete button is clickable', async () => {
    render(ItemCard, { props: { title: 'Test Item' } });
    const deleteBtn = document.querySelector('.delete-btn') as HTMLElement;
    expect(deleteBtn).toBeInTheDocument();
    // Click fires without error
    await fireEvent.click(deleteBtn);
  });

  it('renders card structure', () => {
    render(ItemCard, { props: { title: 'Card Title' } });
    expect(document.querySelector('.item-card')).toBeInTheDocument();
    expect(document.querySelector('.item-header')).toBeInTheDocument();
    expect(document.querySelector('.item-content')).toBeInTheDocument();
  });

  it('shows Untitled when title is empty', () => {
    render(ItemCard, { props: { title: '' } });
    expect(screen.getByText('Untitled')).toBeInTheDocument();
  });

  it('hides delete button when editable is false', () => {
    render(ItemCard, { props: { title: 'Test', editable: false } });
    expect(document.querySelector('.delete-btn')).not.toBeInTheDocument();
  });
});
