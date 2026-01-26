# Welcome Screen Design

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Allow users to create a personalized series of message slides that play before the passphrase prompt in exported HTML documents.

**Architecture:** New "Welcome Screen" section in desktop app sidebar for designing slides. Slides are embedded in exported HTML and play sequentially before showing the passphrase input.

**Tech Stack:** Svelte components for designer UI, embedded JavaScript in exported HTML for slide playback.

---

## Data Model

```typescript
WelcomeScreen {
  enabled: boolean
  slides: MessageSlide[]
}

MessageSlide {
  id: string
  text: string
  transition: { type: "click" } | { type: "auto", seconds: number }
}
```

Stored as part of the document state, exported into the HTML when enabled.

---

## Desktop App: Welcome Screen Section

New sidebar item "Welcome Screen" added after Pets section.

### Main View

```
┌─────────────────────────────────────────────────────┐
│ Welcome Screen                                      │
├─────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────┐ │
│ │ [Toggle] Enable welcome screen for exports      │ │
│ └─────────────────────────────────────────────────┘ │
│                                                     │
│ SLIDES                                              │
│ ┌─────────────────────────────────────────────────┐ │
│ │ 1. "Welcome back, sweetheart..."       [↑][↓][×]│ │
│ ├─────────────────────────────────────────────────┤ │
│ │ 2. "I made this for you..."            [↑][↓][×]│ │
│ ├─────────────────────────────────────────────────┤ │
│ │ 3. "Everything you need is here..."    [↑][↓][×]│ │
│ └─────────────────────────────────────────────────┘ │
│                                                     │
│ [+ Add Slide]                                       │
│                                                     │
│ [Preview]                                           │
└─────────────────────────────────────────────────────┘
```

### Slide Editor (expanded view when clicking a slide)

```
┌─────────────────────────────────────────────────────┐
│ Edit Slide                                          │
├─────────────────────────────────────────────────────┤
│ Message:                                            │
│ ┌─────────────────────────────────────────────────┐ │
│ │ Welcome back, sweetheart. I made this file      │ │
│ │ just for you...                                 │ │
│ └─────────────────────────────────────────────────┘ │
│                                                     │
│ Transition:                                         │
│ (•) Click to continue                               │
│ ( ) Auto-advance after [3] seconds                  │
│                                                     │
│ [Done]                                              │
└─────────────────────────────────────────────────────┘
```

### Interactions

- Click slide row to expand/edit inline
- [↑][↓] buttons to reorder (or drag-and-drop)
- [×] button to delete with confirmation
- Preview opens modal simulating the full slide experience
- Auto-save on changes (like other sections)

### Validation

- Message text required (non-empty)
- Auto-advance duration: 1-10 seconds
- At least 1 slide required when enabled

---

## Exported HTML: Recipient Experience

### Slide Display

Slides appear centered, one at a time, with smooth fade transitions.

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│                                                     │
│         Welcome back, sweetheart.                   │
│         I made this file just for you...            │
│                                                     │
│                                                     │
│                   [Continue]                        │
│           (or auto-advances if timed)               │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Flow

1. File opens → welcome slides begin
2. Each slide displays according to its transition setting
3. After final slide fades out → passphrase input appears
4. User enters passphrase → document decrypts and displays

### Styling

- Match existing HTML export theme (forest color scheme)
- Centered text, comfortable font size
- Subtle fade transitions between slides
- Continue button styled consistently with existing UI

---

## Export Flow Integration

### Export Dialog Changes

When welcome screen is enabled with slides:

```
┌─────────────────────────────────────────────────────┐
│ Create Your Secure File                             │
├─────────────────────────────────────────────────────┤
│                                                     │
│ ☑ Include welcome screen (3 slides)                 │
│                                                     │
│ Choose a passphrase:                                │
│ [____________________________________] [Generate]   │
│                                                     │
│ Confirm passphrase:                                 │
│ [____________________________________]              │
│                                                     │
│ ☐ Also print a physical copy                        │
│                                                     │
├─────────────────────────────────────────────────────┤
│                        [Cancel]  [Export File]      │
└─────────────────────────────────────────────────────┘
```

- Checkbox only appears if welcome screen is enabled with ≥1 slide
- Checked by default when available
- User can uncheck to export without slides for this export

---

## Technical Implementation Notes

### Rust Backend

- Add `WelcomeScreen` to `LegacyDocument` model
- Serialize slides into HTML during export
- No encryption changes needed (passphrase flow unchanged)

### Svelte Frontend

- New `WelcomeScreenSection.svelte` component
- Add to sidebar navigation in `App.svelte`
- Reuse existing `ItemCard`, `AddButton`, `FormField` patterns

### HTML Export

- Embed slides as JSON in a script tag
- Add slide playback JavaScript before existing decrypt logic
- CSS for slide presentation (centered, fade transitions)

---

## Out of Scope

- Question slides / answer buttons (removed for security simplicity)
- Using answers as encryption key
- Rich text or images in slides
- Multiple welcome screen configurations
- Slide templates or presets
