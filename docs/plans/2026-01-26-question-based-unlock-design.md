# Question-Based Unlock Design

**Goal:** Replace the single passphrase unlock with a personal question-and-answer flow, where creators design a mix of message and question slides, and recipients type answers to unlock. Optional fallback passphrase provides a recovery path.

---

## Data Model

```typescript
MessageSlide {
  id: string
  type: "message" | "question"
  text: string                    // Message text OR question text
  answer?: string                 // Only for questions - expected answer (lowercase)
  transition: { type: "click" } | { type: "auto", seconds: number }
}

WelcomeScreen {
  enabled: boolean
  slides: MessageSlide[]          // Mix of messages and questions
  fallbackPassphrase?: string     // Optional, stored only for encryption
}
```

**Validation rules:**
- When enabled, must have 2-5 question slides (messages don't count toward minimum)
- At least one slide total required
- Fallback passphrase optional, but warn if not set

**Encryption approach:**
- Generate random document key
- Derive "question key" from concatenated lowercase answers
- Derive "passphrase key" from fallback passphrase (if set)
- Encrypt document with document key
- Encrypt document key with question key (always)
- Encrypt document key with passphrase key (if passphrase provided)
- Store both encrypted key blobs in exported HTML

---

## Desktop App - Slide Designer UI

The existing Welcome Screen section supports both message and question slides.

### Main View

```
┌─────────────────────────────────────────────────────────────┐
│ Welcome Screen                                              │
├─────────────────────────────────────────────────────────────┤
│ [Toggle] Enable welcome screen for exports                  │
│                                                             │
│ SLIDES                                                      │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ 💬 1. "Welcome back, sweetheart..."            [↑][↓][×]│ │
│ ├─────────────────────────────────────────────────────────┤ │
│ │ ❓ 2. "What was our first dance song?"         [↑][↓][×]│ │
│ │      Answer: wonderful tonight                          │ │
│ ├─────────────────────────────────────────────────────────┤ │
│ │ 💬 3. "I knew you'd remember..."               [↑][↓][×]│ │
│ ├─────────────────────────────────────────────────────────┤ │
│ │ ❓ 4. "Where was our first date?"              [↑][↓][×]│ │
│ │      Answer: the coffee shop on main street             │ │
│ └─────────────────────────────────────────────────────────┘ │
│                                                             │
│ [+ Add Message]  [+ Add Question]                           │
│                                                             │
│ Questions: 2 of 2-5 required                                │
│                                                             │
│ ─────────────────────────────────────────────────────────── │
│ FALLBACK PASSPHRASE (optional)                              │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ [____________________________________] [Generate]       │ │
│ └─────────────────────────────────────────────────────────┘ │
│ ⚠ Without a passphrase, there's no recovery if the         │
│   recipient forgets the answers.                            │
│                                                             │
│ [Preview]                                                   │
└─────────────────────────────────────────────────────────────┘
```

### Slide Editor (expanded when clicking a slide)

```
┌─────────────────────────────────────────────────────────────┐
│ Edit Question Slide                                         │
├─────────────────────────────────────────────────────────────┤
│ Question:                                                   │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ What was our first dance song?                          │ │
│ └─────────────────────────────────────────────────────────┘ │
│                                                             │
│ Expected Answer:                                            │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ wonderful tonight                                       │ │
│ └─────────────────────────────────────────────────────────┘ │
│ Answers are case-insensitive                                │
│                                                             │
│ Transition:                                                 │
│ (•) Click to continue                                       │
│ ( ) Auto-advance after [3] seconds                          │
│                                                             │
│ [Done]                                                      │
└─────────────────────────────────────────────────────────────┘
```

**Key behaviors:**
- 💬 / ❓ icons distinguish message vs question slides
- Question slides show the expected answer (visible only to creator)
- Counter shows how many questions exist vs required (2-5)
- Warning appears if no fallback passphrase is set
- Preview shows the full recipient experience

---

## Exported HTML - Recipient Experience

### First-Time Flow

**Step 1: Slides play in sequence (messages and questions mixed)**

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                                                             │
│              Welcome back, sweetheart.                      │
│              I made this file just for you.                 │
│                                                             │
│                                                             │
│                      [Continue]                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Step 2: Question slide appears**

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                                                             │
│              What was our first dance song?                 │
│                                                             │
│              [_______________________________]              │
│                                                             │
│                      [Continue]                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Step 3: After final slide, validation happens**

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                        🔓                                   │
│                   Unlocking...                              │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**If answers are correct:** Document decrypts and displays.

**If answers are wrong:**

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│              Some answers weren't quite right.              │
│              Please try again.                              │
│                                                             │
│              Attempt 1 of 5                                 │
│                                                             │
│  ┌────────────────────────────────────────────────────────┐ │
│  │ What was our first dance song?                         │ │
│  │ [wonderful tonite____________________________]         │ │
│  │                                                        │ │
│  │ Where was our first date?                              │ │
│  │ [the coffee shop_____________________________]         │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                             │
│                      [Try Again]                            │
│                                                             │
│              Use passphrase instead                         │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**After 5 failed attempts:**

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│              Having trouble remembering?                    │
│                                                             │
│              You can keep trying, or use the                │
│              passphrase if you have it.                     │
│                                                             │
│                   [Try Again]                               │
│                                                             │
│              Use passphrase instead                         │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Passphrase Fallback Screen

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                        🔒                                   │
│                                                             │
│              Enter passphrase                               │
│                                                             │
│              [_______________________________]              │
│                                                             │
│                      [Unlock]                               │
│                                                             │
│              ← Back to questions                            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Encryption & Export Flow

### Key Derivation

```
Creator enters:
  - Question 1 answer: "Wonderful Tonight"
  - Question 2 answer: "The Coffee Shop on Main Street"

Normalize (lowercase + trim):
  - "wonderful tonight"
  - "the coffee shop on main street"

Concatenate:
  - "wonderful tonightthe coffee shop on main street"

Derive key via PBKDF2:
  - Salt: random 16 bytes
  - Iterations: 600,000
  - Output: 256-bit AES key
```

### Encryption Structure

```javascript
// Stored in exported HTML
ENCRYPTED_DATA = {
  // Document key encrypted with question-derived key (always present)
  questionKey: {
    salt: "...",        // For PBKDF2
    nonce: "...",       // For AES-GCM
    ciphertext: "..."   // Encrypted document key
  },

  // Document key encrypted with passphrase (only if fallback set)
  passphraseKey: {      // Optional
    salt: "...",
    nonce: "...",
    ciphertext: "..."
  },

  // Document encrypted with document key
  document: {
    nonce: "...",
    ciphertext: "..."
  }
}
```

### Unlock Flow (JavaScript in HTML)

```
1. Collect answers from question slides
2. Normalize: lowercase + trim each answer
3. Concatenate in slide order
4. Derive key using PBKDF2 with questionKey.salt
5. Try to decrypt questionKey.ciphertext → document key
6. If success: decrypt document.ciphertext with document key
7. If failure: increment attempt counter, show retry screen
```

### Passphrase Unlock Flow

```
1. User enters passphrase
2. Derive key using PBKDF2 with passphraseKey.salt
3. Try to decrypt passphraseKey.ciphertext → document key
4. If success: decrypt document.ciphertext with document key
5. If failure: show error
```

---

## Export Dialog Changes

### When Welcome Screen is Enabled with Questions

```
┌─────────────────────────────────────────────────────────────┐
│ Create Your Secure File                                     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ ☑ Include welcome screen (2 messages, 3 questions)         │
│                                                             │
│ Your file will be unlocked by answering the questions       │
│ you set up in the Welcome Screen section.                   │
│                                                             │
│ Fallback passphrase: ✓ Set                                  │
│   (or: ⚠ Not set - no recovery if answers forgotten)        │
│                                                             │
│ ☐ Also print a physical copy                                │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                              [Cancel]  [Export File]        │
└─────────────────────────────────────────────────────────────┘
```

**Key changes:**
- No passphrase input in export dialog (comes from Welcome Screen settings)
- Shows summary of what's configured
- Warning if no fallback passphrase set

### When Welcome Screen is Disabled

Falls back to current behavior - standard passphrase input in export dialog.

---

## Edge Cases

| Scenario | Behavior |
|----------|----------|
| Creator has <2 questions | Cannot export - validation error |
| Creator has >5 questions | Cannot add more - button disabled |
| No fallback passphrase | Warning shown, but export allowed |
| Recipient's answer has extra spaces | Trimmed before comparison |
| Recipient uses wrong case | Normalized to lowercase |
| Recipient fails 5+ times | Encouraged to use passphrase, but can keep trying |
| File has no passphraseKey blob | "Use passphrase" link hidden |
| Welcome screen disabled | Export uses standard passphrase flow (current behavior) |

---

## Out of Scope

- Hints for individual questions
- Partial credit / fuzzy matching on answers
- Multiple choice answers
- Images or rich text in slides
- Time-limited lockout after failures
