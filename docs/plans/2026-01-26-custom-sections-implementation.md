# Custom Sections Implementation Plan

## Tasks

### 1. Update Rust Data Models
- Add `CustomSection`, `CustomSubsection`, `FieldDefinition`, `FieldType`, `CustomItem` structs to `models.rs`
- Add `custom_sections: Vec<CustomSection>` to `LegacyDocument`
- Ensure all new types derive necessary traits (Serialize, Deserialize, Clone, Default, Debug)

### 2. Update TypeScript Types
- Add corresponding TypeScript interfaces to `document.ts`
- Add `custom_sections` to `LegacyDocument` interface

### 3. Create CustomSection Svelte Component
- Create `CustomSectionEditor.svelte` for editing a custom subsection
- Field definition UI with add/remove/type selection
- Item list with add/edit/delete
- Item editing form that renders based on field definitions

### 4. Update Sidebar for Custom Top-Level Sections
- Add "+ Add Section" button at bottom of sidebar
- Display custom top-level sections in sidebar navigation
- Add delete/rename functionality for custom sections

### 5. Update Section Pages for Custom Subsections
- Add "+ Add Custom Subsection" button to each preset section
- Render custom subsections at the bottom of each section
- Integrate `CustomSectionEditor` component

### 6. Update Export HTML Template
- Render custom top-level sections in sidebar
- Render custom subsections within their parent sections
- Render custom items with their field values

### 7. Update isDocumentEmpty Check
- Include custom sections in the empty document check
