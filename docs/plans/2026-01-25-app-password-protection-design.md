# App Password Protection Design

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add app-level password protection to prevent unauthorized access to local data, plus options to clear data.

**Architecture:** Lock screen on launch when data exists, password setup prompt on first data entry, settings menu with clear data options.

**Tech Stack:** Svelte 4, TypeScript, Tauri 2.0, Argon2id for password hashing

---

## App Password Flow

### States

- **No data, no password** - App opens directly to guided wizard or full view. Fresh state.
- **Has data, has password** - App opens to lock screen. User must enter password to access.
- **Data cleared** - Returns to "no data, no password" state.

### Setting Password (First Data Entry)

1. User starts typing in any field or clicks "Add" button
2. Modal appears: "Set App Password"
3. User enters password (8+ chars) and confirms it
4. On success: password hash stored, data entry continues
5. On cancel: input is reverted, no data saved

### Unlocking App

1. App launches, detects existing data
2. Lock screen shown with password field
3. Correct password → app unlocks, shows last state
4. Wrong password → error message, stays locked
5. No "forgot password" - only option is "Clear All Data" (destructive)

### Password Storage

- Hash password with Argon2id
- Store hash in `password.hash` file in app data directory
- Never store plaintext password

---

## Settings Menu & Clear Data Options

### Settings Access

- Gear icon button in sidebar footer (between "Guided Setup" and "Import File")
- Opens a Settings modal dialog

### Settings Modal Contents

**Security section:**
- "Change Password" button - requires current password, then set new one
- "Clear on Exit" toggle - when ON, wipes all data when app closes
- "Clear All Data" button - immediate wipe after password confirmation

### Clear on Exit Behavior

- Toggle is OFF by default
- When ON, shows warning: "Data will be deleted when you close the app"
- On app close: delete local encrypted file + remove password hash
- Next launch returns to fresh state

### Clear All Data Behavior

1. User clicks "Clear All Data"
2. Confirmation modal: "Enter your password to confirm deletion"
3. Correct password → wipe data + password hash, close settings, show fresh app
4. Wrong password → error, no action

### Clear All Data from Lock Screen

- Small "Clear All Data" link below password field on lock screen
- Same confirmation flow (requires password)
- Allows recovery if user forgets password but accepts data loss

---

## Technical Implementation

### New Components

- `src/lib/components/LockScreen.svelte` - Password entry on app launch
- `src/lib/components/SetPasswordModal.svelte` - Initial password setup
- `src/lib/components/SettingsModal.svelte` - Settings with clear options
- `src/lib/components/ChangePasswordModal.svelte` - Change existing password

### Rust Backend Commands

Add to `main.rs`:
- `set_app_password(password: String)` - Hash and store password
- `verify_app_password(password: String) -> bool` - Check password
- `has_app_password() -> bool` - Check if password exists
- `clear_all_data()` - Delete document + password hash
- `change_password(old: String, new: String)` - Verify old, set new
- `get_clear_on_exit() -> bool` - Get setting
- `set_clear_on_exit(enabled: bool)` - Set setting

### Storage Changes

- Store password hash in `password.hash` file alongside `document.encrypted`
- Store settings in `settings.json` file
- Use Argon2id for password hashing (already a dependency)

### App.svelte Changes

- New state: `isLocked: boolean`
- On mount: check `has_app_password()` → if true, show LockScreen
- Pass `onUnlock` callback to LockScreen
- Add Settings button (gear icon) to sidebar footer
- Track `hasPassword: boolean` in app state

### Data Entry Interception

- Create wrapper/hook that checks if password is set before allowing data changes
- If no password exists when user tries to enter data, show SetPasswordModal
- On successful password creation, allow the data change to proceed
- On cancel, revert the attempted change

### Clear on Exit Implementation

- Store `clearOnExit` flag in settings
- On app window close event (Tauri), check flag
- If true, call `clear_all_data()` before closing

---

## Success Criteria

1. Fresh app (no data) opens directly without password prompt
2. First data entry triggers password setup modal
3. App with data shows lock screen on launch
4. Correct password unlocks app
5. Wrong password shows error, stays locked
6. Settings accessible via gear icon in sidebar
7. "Change Password" works with old password verification
8. "Clear on Exit" toggle wipes data on app close when enabled
9. "Clear All Data" requires password and wipes everything
10. "Clear All Data" available on lock screen for forgotten password recovery
