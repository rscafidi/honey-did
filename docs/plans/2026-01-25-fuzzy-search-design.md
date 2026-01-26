# Fuzzy Search with Jump-to-Match Navigation

## Overview

Improve search in the exported HTML document to help users find specific information quickly, even with imperfect recall of names or spellings.

## Features

### Search Bar & Controls

```
┌─────────────────────────────────────────────────────────┐
│ [Search: _______________]  ◀ ▶  Match 3 of 8           │
│                                                         │
│ Show: [Exact ✓] [Contains ✓] [Spelling ✓] [Sounds-like ✓] │
└─────────────────────────────────────────────────────────┘
```

- Search triggers after 2+ characters (debounced 300ms)
- Prev/next buttons (◀ ▶) navigate through visible match types only
- Counter shows "Match X of Y" for filtered results
- Toggles with zero results show disabled: `[Spelling (0)]`
- Current match scrolls into view with distinct highlight
- Default: all toggles ON

### Match Types (Priority Order)

1. **Exact** - Case-insensitive whole-word match
2. **Contains** - Search term is a substring
3. **Spelling** - Levenshtein distance ≤ 2 edits
4. **Sounds-like** - Phonetic match using Metaphone algorithm

### Match Highlighting & Badges

**Badge labels:**
- `[exact]` - Case-insensitive exact match
- `[contains]` - Search term appears within the text
- `[~spelling]` - Within 1-2 character edits
- `[sounds like]` - Phonetic match

**Visual treatment:**
- All matches: pale yellow background
- Current match: orange background + thin border
- Badge: small, gray, lowercase, inline after matched text
- Toggled-off matches: hidden entirely (no highlight, no badge)

### Deduplication

A match only appears in its highest-confidence category:
- "Ally" matching "ally" search → `[exact]` only, not also `[contains]`
- "Sally" contains "ally" AND sounds similar → `[contains]` (higher priority)

### Navigation Order

When pressing ▶, jumps proceed:
1. All exact matches (document order, top to bottom)
2. All contains matches (document order)
3. All spelling matches (document order)
4. All sounds-like matches (document order)
5. Wraps to first match

### Minimum Match Quality

- Contains: search term must be 3+ characters
- Spelling: max distance 2 for short words, 3 for words 8+ characters
- Sounds-like: only matches words 3+ characters

## Technical Implementation

### Phonetic Matching

- Include minimal Metaphone implementation (~50 lines JS)
- Pre-compute phonetic codes for all text nodes on document decrypt
- Store in lookup map for fast searching

### Search Index

Built on decrypt:
```javascript
{
  text: "Ally Bank",
  node: <DOMNode>,
  phonetic: "AL"
}
```

### State Management

```javascript
searchState = {
  term: "ally",
  matches: [
    { node, text, type: "exact", index: 0 },
    { node, text, type: "contains", index: 1 },
    ...
  ],
  filters: { exact: true, contains: true, spelling: true, phonetic: true },
  currentIndex: 2
}
```

### Performance

- Debounce search input by 300ms
- Index builds once on decrypt, searches are fast lookups
- Cache Levenshtein calculations for repeated searches

### File Size Impact

- Metaphone algorithm: ~2KB minified
- Levenshtein function: ~0.5KB
- Search UI controls: ~1KB
- **Total addition: ~4-5KB**
