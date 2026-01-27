# Custom Sections Design

**Goal:** Allow users to create custom subsections within existing preset sections (e.g., "529 Accounts" in Financial) and entirely new top-level sections (e.g., "Recipes"), with user-defined fields.

---

## Data Model

The document gains a `custom_sections` field alongside existing preset sections:

```rust
pub struct LegacyDocument {
    // Existing preset sections unchanged
    pub financial: FinancialSection,
    pub insurance: InsuranceSection,
    pub bills: BillsSection,
    pub property: PropertySection,
    pub legal: LegalSection,
    pub digital: DigitalSection,
    pub household: HouseholdSection,
    pub personal: PersonalSection,
    pub contacts: ContactsSection,
    pub medical: MedicalSection,
    pub pets: PetsSection,
    pub welcome_screen: Option<WelcomeScreen>,

    // New: custom content
    pub custom_sections: Vec<CustomSection>,
}

pub struct CustomSection {
    pub id: String,
    pub name: String,                    // "Recipes" or "529 Accounts"
    pub parent: Option<String>,          // None = top-level, Some("financial") = subsection
    pub subsections: Vec<CustomSubsection>,
}

pub struct CustomSubsection {
    pub id: String,
    pub name: String,                    // "Grandma's Recipes"
    pub field_definitions: Vec<FieldDefinition>,
    pub items: Vec<CustomItem>,
}

pub struct FieldDefinition {
    pub id: String,
    pub name: String,                    // "Prep Time"
    pub field_type: FieldType,           // Text, Number, Date, Boolean
}

pub enum FieldType {
    Text,
    Number,
    Date,
    Boolean,
}

pub struct CustomItem {
    pub id: String,
    pub values: HashMap<String, String>, // field_id -> value
}
```

**Key points:**
- Preset sections remain unchanged
- Custom subsections within Financial are stored with `parent: Some("financial")`
- Custom top-level sections have `parent: None`
- A "Notes" field is auto-included for every custom subsection

---

## UI: Creating Custom Content

### Adding a Custom Top-Level Section

At the bottom of the sidebar navigation:

```
┌─────────────────────────┐
│ Financial               │
│ Insurance               │
│ Bills                   │
│ ...                     │
│ Pets                    │
│                         │
│ + Add Section           │
└─────────────────────────┘
```

Clicking opens an inline form:

```
┌─────────────────────────┐
│ Section name:           │
│ [Recipes____________]   │
│ [Cancel]  [Create]      │
└─────────────────────────┘
```

### Adding a Custom Subsection

At the bottom of any section page:

```
┌─────────────────────────────────────────────┐
│ FINANCIAL                                   │
├─────────────────────────────────────────────┤
│ Bank Accounts        [+ Add Account]        │
│ ┌─────────────────────────────────────────┐ │
│ │ Chase Checking ...                      │ │
│ └─────────────────────────────────────────┘ │
│                                             │
│ Credit Cards         [+ Add Card]           │
│ ...                                         │
│                                             │
│ + Add Custom Subsection                     │
└─────────────────────────────────────────────┘
```

---

## UI: Defining Fields

When creating a custom subsection, user defines fields:

```
┌─────────────────────────────────────────────────────────────┐
│ 529 Accounts                                    [+ Add Item]│
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ ⚙ Define Fields                                             │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ Account Name        [Text     ▼]              [×]       │ │
│ │ Beneficiary         [Text     ▼]              [×]       │ │
│ │ Institution         [Text     ▼]              [×]       │ │
│ │ Balance             [Number   ▼]              [×]       │ │
│ │ Opened Date         [Date     ▼]              [×]       │ │
│ │                                                         │ │
│ │ [+ Add Field]                                           │ │
│ └─────────────────────────────────────────────────────────┘ │
│                                                             │
│ No items yet. Click "+ Add Item" to add your first.         │
└─────────────────────────────────────────────────────────────┘
```

**Field types:**
- Text - free-form text input
- Number - numeric input
- Date - date picker
- Yes/No - boolean toggle

**Behaviors:**
- Field definition panel is collapsible
- Expanded by default when empty, collapsed once items exist
- Fields can be deleted (with confirmation if items have data)
- "Notes" field auto-included

---

## UI: Adding and Displaying Items

Once fields are defined, items display as cards:

```
┌─────────────────────────────────────────────────────────────┐
│ 529 Accounts                                    [+ Add Item]│
├─────────────────────────────────────────────────────────────┤
│ ⚙ Define Fields (5 fields)                           [Edit]│
│                                                             │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ Emma's College Fund                              [Edit] │ │
│ │ Beneficiary: Emma Johnson                               │ │
│ │ Institution: Vanguard                                   │ │
│ │ Balance: 45,000                                         │ │
│ │ Opened: 2019-03-15                                      │ │
│ │ Notes: Target school is State University                │ │
│ └─────────────────────────────────────────────────────────┘ │
│                                                             │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ Jake's College Fund                              [Edit] │ │
│ │ ...                                                     │ │
│ └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

**Item editing form:**

```
┌─────────────────────────────────────────────────────────────┐
│ Account Name:  [Emma's College Fund_______________]        │
│ Beneficiary:   [Emma Johnson______________________]        │
│ Institution:   [Vanguard__________________________]        │
│ Balance:       [45000_____]                                │
│ Opened:        [2019-03-15]                                │
│ Notes:         [Target school is State University_]        │
│                                                             │
│                              [Cancel] [Delete] [Save]       │
└─────────────────────────────────────────────────────────────┘
```

The first field's value becomes the item's display title.

---

## Ordering

- Custom subsections appear after all preset subsections within a section
- Custom top-level sections appear at the bottom of the sidebar
- No reordering UI in initial implementation

---

## Exported HTML

Custom sections render identically to preset sections:

**Sidebar:**
```
│ Financial               │
│ Insurance               │
│ ...                     │
│ Pets                    │
│ Recipes                 │  ← custom, same styling
```

**Within Financial:**
```
│ Bank Accounts                                               │
│ Credit Cards                                                │
│ Investments                                                 │
│ Debts & Loans                                               │
│ 529 Accounts            ← custom, same styling              │
```

Recipients see no distinction between preset and custom content.

---

## Deleting Custom Content

**Deleting a subsection:**

```
┌─────────────────────────────────────────────────────────────┐
│ 529 Accounts                          [+ Add Item] [Delete]│
```

Confirmation dialog:
```
┌─────────────────────────────────────────────────────────────┐
│ Delete "529 Accounts"?                                      │
│                                                             │
│ This will permanently delete the subsection and all 2       │
│ items within it.                                            │
│                                                             │
│                              [Cancel] [Delete]              │
└─────────────────────────────────────────────────────────────┘
```

**Deleting a top-level section:**

Sidebar shows delete icon on hover for custom sections:
```
│ Recipes                    [×] │
```

Confirmation includes count of all subsections and items.

**Renaming:**

Click title or edit icon to rename inline.

---

## Design Decisions Summary

| Decision | Choice |
|----------|--------|
| Custom subsections alongside presets | Yes, presets always visible |
| Field types | Simple: Text, Number, Date, Yes/No |
| Scope | Both subsections AND top-level sections |
| Creation UI | Inline "+ Add" buttons |
| Appearance in export | Blends in seamlessly |
| Ordering | Custom content appears at end |

---

## Out of Scope

- Reordering custom sections/subsections
- Hiding or disabling preset sections
- Complex field types (lists, currency, phone, email, URL)
- Field validation beyond basic type
- Templates/presets for common custom sections
- Import/export of custom section definitions
