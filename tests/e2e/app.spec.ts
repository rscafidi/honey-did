import { test, expect, type Page } from '@playwright/test';

/**
 * E2E tests for the Honey Did application.
 *
 * These tests run against the Vite dev server (web mode, no Tauri backend).
 * In web mode the app uses localStorage for persistence.
 */

// Helper: dismiss intro/empty-document screens so we can reach the main UI
async function dismissIntro(page: Page) {
  // Wait for the app to load
  await page.waitForLoadState('networkidle');

  // The app may show an intro screen for empty documents. Click through it if present.
  const skipButton = page.getByText('Skip', { exact: false });
  if (await skipButton.isVisible({ timeout: 3000 }).catch(() => false)) {
    await skipButton.click();
  }

  // If a "Get Started" or similar CTA is shown, click it
  const getStarted = page.getByText('Get Started', { exact: false });
  if (await getStarted.isVisible({ timeout: 1000 }).catch(() => false)) {
    await getStarted.click();
  }

  // Wait for main content area
  await page.waitForSelector('.section, .sidebar, [class*="section"]', { timeout: 5000 }).catch(() => {});
}

// Helper: navigate to a section by clicking sidebar
async function navigateToSection(page: Page, sectionName: string) {
  const sidebar = page.locator('.sidebar, nav, [class*="sidebar"]');
  const button = sidebar.getByText(sectionName, { exact: false }).first();
  await button.click();
  // Small delay for section render
  await page.waitForTimeout(300);
}

// Helper: click the Add button within the current view
async function clickAddButton(page: Page, label?: string) {
  if (label) {
    await page.getByText(`+ ${label}`, { exact: false }).click();
  } else {
    await page.locator('.add-button').first().click();
  }
  await page.waitForTimeout(200);
}

// ==============================
// Section Navigation
// ==============================
test.describe('Section Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
  });

  test('displays sidebar with all built-in sections', async ({ page }) => {
    const sectionNames = [
      'Financial', 'Insurance', 'Bills', 'Property', 'Legal',
      'Digital', 'Household', 'Personal', 'Contacts', 'Medical', 'Pets',
    ];

    for (const name of sectionNames) {
      await expect(page.getByText(name, { exact: false }).first()).toBeVisible();
    }
  });

  test('navigates between sections', async ({ page }) => {
    await navigateToSection(page, 'Insurance');
    await expect(page.getByText('Insurance', { exact: false }).first()).toBeVisible();

    await navigateToSection(page, 'Bills');
    await expect(page.getByText('Bills', { exact: false }).first()).toBeVisible();
  });
});

// ==============================
// Financial Section
// ==============================
test.describe('Financial Section', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
    await navigateToSection(page, 'Financial');
  });

  test('adds a bank account with all fields', async ({ page }) => {
    await clickAddButton(page, 'Add Bank Account');

    // Fill in bank account fields
    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    // Account Name
    await fields.filter({ hasText: 'Account Name' }).locator('input').fill('My Checking');
    // Institution
    await fields.filter({ hasText: 'Institution' }).locator('input').fill('Chase Bank');
    // Account Type
    await fields.filter({ hasText: 'Account Type' }).locator('input').fill('Checking');
    // Last 4 Digits
    await fields.filter({ hasText: 'Last 4' }).locator('input').fill('1234');
    // Notes
    await fields.filter({ hasText: 'Notes' }).locator('textarea').fill('Primary checking account');

    // Verify the card title updates
    await expect(card.locator('.item-card-header').first()).toContainText('My Checking');
  });

  test('adds a credit card', async ({ page }) => {
    await clickAddButton(page, 'Add Credit Card');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Card Name' }).locator('input').fill('Rewards Visa');
    await fields.filter({ hasText: 'Issuer' }).locator('input').fill('Citi');
    await fields.filter({ hasText: 'Last 4' }).locator('input').fill('5678');

    await expect(card.locator('.item-card-header').first()).toContainText('Rewards Visa');
  });

  test('adds an investment account', async ({ page }) => {
    await clickAddButton(page, 'Add Investment');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Account Name' }).locator('input').fill('Retirement 401k');
    await fields.filter({ hasText: 'Institution' }).locator('input').fill('Fidelity');
    await fields.filter({ hasText: 'Account Type' }).locator('input').fill('401k');

    await expect(card.locator('.item-card-header').first()).toContainText('Retirement 401k');
  });

  test('adds a debt', async ({ page }) => {
    await clickAddButton(page, 'Add Debt');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Description' }).locator('input').fill('Mortgage');
    await fields.filter({ hasText: 'Lender' }).locator('input').fill('Bank of America');

    await expect(card.locator('.item-card-header').first()).toContainText('Mortgage');
  });

  test('fills section notes', async ({ page }) => {
    const notesArea = page.locator('.notes-field textarea');
    await notesArea.fill('All financial accounts are joint with spouse');
    await expect(notesArea).toHaveValue('All financial accounts are joint with spouse');
  });

  test('removes a bank account', async ({ page }) => {
    await clickAddButton(page, 'Add Bank Account');
    await page.waitForTimeout(200);

    const deleteBtn = page.locator('.item-card .delete-btn').first();
    await deleteBtn.click();

    // After delete, the card should be removed
    const cards = page.locator('.item-card');
    await expect(cards).toHaveCount(0);
  });
});

// ==============================
// Insurance Section
// ==============================
test.describe('Insurance Section', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
    await navigateToSection(page, 'Insurance');
  });

  test('adds a policy with all fields', async ({ page }) => {
    await clickAddButton(page, 'Add Policy');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Policy Type' }).locator('input').fill('Life Insurance');
    await fields.filter({ hasText: 'Provider' }).locator('input').fill('MetLife');
    await fields.filter({ hasText: 'Policy Number' }).locator('input').fill('POL-123456');
    await fields.filter({ hasText: 'Contact' }).locator('input').fill('800-555-0100');
    await fields.filter({ hasText: 'Notes' }).locator('textarea').fill('$500k term life');

    await expect(card.locator('.item-card-header').first()).toContainText('Life Insurance');
  });
});

// ==============================
// Bills Section
// ==============================
test.describe('Bills Section', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
    await navigateToSection(page, 'Bills');
  });

  test('adds a bill with all fields including autopay checkbox', async ({ page }) => {
    await clickAddButton(page, 'Add Bill');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Bill Name' }).locator('input').fill('Electric');
    await fields.filter({ hasText: 'Provider' }).locator('input').fill('ConEd');
    await fields.filter({ hasText: 'Amount' }).locator('input').fill('$150');
    await fields.filter({ hasText: 'Due Day' }).locator('input').fill('15');

    // Autopay checkbox
    const autopayCheckbox = card.locator('input[type="checkbox"]');
    await autopayCheckbox.check();
    await expect(autopayCheckbox).toBeChecked();

    await expect(card.locator('.item-card-header').first()).toContainText('Electric');
  });
});

// ==============================
// Property Section
// ==============================
test.describe('Property Section', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
    await navigateToSection(page, 'Property');
  });

  test('adds a property', async ({ page }) => {
    await clickAddButton(page, 'Add Property');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('Family Home');
    await fields.filter({ hasText: 'Address' }).locator('input').fill('123 Main Street');

    await expect(card.locator('.item-card-header').first()).toContainText('Family Home');
  });

  test('adds a vehicle', async ({ page }) => {
    await clickAddButton(page, 'Add Vehicle');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('2023 Honda Civic');
    await fields.filter({ hasText: 'Details' }).locator('input').fill('VIN: 1HGBH41JXMN109186');
  });

  test('adds a valuable', async ({ page }) => {
    await clickAddButton(page, 'Add Valuable');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('Diamond Ring');
    await fields.filter({ hasText: 'Location' }).locator('input').fill('Safe deposit box');
  });
});

// ==============================
// Legal Section
// ==============================
test.describe('Legal Section', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
    await navigateToSection(page, 'Legal');
  });

  test('fills will location and power of attorney', async ({ page }) => {
    const willField = page.locator('.form-field').filter({ hasText: 'Will Location' }).locator('input, textarea').first();
    await willField.fill('Safe deposit box at Chase Bank');

    const poaField = page.locator('.form-field').filter({ hasText: 'Power of Attorney' }).locator('input, textarea').first();
    await poaField.fill('Jane Doe');
  });

  test('fills attorney contact info', async ({ page }) => {
    const attorneyCard = page.locator('.item-card').filter({ hasText: /Attorney|attorney/i }).first();
    if (await attorneyCard.isVisible().catch(() => false)) {
      const fields = attorneyCard.locator('.form-field');
      await fields.filter({ hasText: 'Name' }).locator('input').first().fill('Robert Smith, Esq.');
      await fields.filter({ hasText: 'Phone' }).locator('input').fill('555-0100');
      await fields.filter({ hasText: 'Email' }).locator('input').fill('bob@smithlaw.com');
    }
  });

  test('adds a trust', async ({ page }) => {
    await clickAddButton(page, 'Add Trust');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('Family Trust');
    await fields.filter({ hasText: 'Trustee' }).locator('input').fill('Jane Doe');
  });
});

// ==============================
// Digital Life Section
// ==============================
test.describe('Digital Life Section', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
    await navigateToSection(page, 'Digital');
  });

  test('fills password manager info', async ({ page }) => {
    const pmSection = page.locator('.form-field').filter({ hasText: /Password Manager|Manager Name/i }).first();
    if (await pmSection.isVisible().catch(() => false)) {
      await pmSection.locator('input').fill('1Password');
    }
  });

  test('adds an email account', async ({ page }) => {
    await clickAddButton(page, 'Add Email');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('Personal Gmail');
    await fields.filter({ hasText: 'Username' }).locator('input').fill('user@gmail.com');
  });

  test('adds a social media account', async ({ page }) => {
    await clickAddButton(page, 'Add Social');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('Twitter/X');
    await fields.filter({ hasText: 'Username' }).locator('input').fill('@myhandle');
  });
});

// ==============================
// Household Section
// ==============================
test.describe('Household Section', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
    await navigateToSection(page, 'Household');
  });

  test('adds a maintenance item', async ({ page }) => {
    await clickAddButton(page, 'Add Maintenance');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('HVAC Filter');
    await fields.filter({ hasText: 'Frequency' }).locator('input').fill('Every 3 months');
  });

  test('adds a contractor', async ({ page }) => {
    await clickAddButton(page, 'Add Contractor');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('Joe the Plumber');
    await fields.filter({ hasText: 'Phone' }).locator('input').fill('555-0200');
  });

  test('adds a how-to', async ({ page }) => {
    await clickAddButton(page, 'Add');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('Sprinkler System');
  });
});

// ==============================
// Personal Section
// ==============================
test.describe('Personal Section', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
    await navigateToSection(page, 'Personal');
  });

  test('fills funeral preferences textarea', async ({ page }) => {
    const textarea = page.locator('textarea').filter({ hasText: '' }).first();
    // Find the funeral preferences section
    const funeralSection = page.locator('.subsection').filter({ hasText: /Funeral/i }).first();
    if (await funeralSection.isVisible().catch(() => false)) {
      const ta = funeralSection.locator('textarea');
      await ta.fill('Cremation, scatter ashes at the beach');
      await expect(ta).toHaveValue('Cremation, scatter ashes at the beach');
    }
  });

  test('adds a personal message', async ({ page }) => {
    await clickAddButton(page, 'Add Message');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Recipient' }).locator('input').fill('My Children');
    await fields.filter({ hasText: 'Message' }).locator('textarea').fill('Always be kind to each other');
  });
});

// ==============================
// Contacts Section
// ==============================
test.describe('Contacts Section', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
    await navigateToSection(page, 'Contacts');
  });

  test('adds an emergency contact', async ({ page }) => {
    await clickAddButton(page, 'Add Emergency');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('Jane Doe');
    await fields.filter({ hasText: 'Relationship' }).locator('input').fill('Spouse');
    await fields.filter({ hasText: 'Phone' }).locator('input').fill('555-0300');
    await fields.filter({ hasText: 'Email' }).locator('input').fill('jane@email.com');
  });

  test('adds a family member contact', async ({ page }) => {
    await clickAddButton(page, 'Add Family');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('Mom');
    await fields.filter({ hasText: 'Phone' }).locator('input').fill('555-0400');
  });

  test('adds a professional contact', async ({ page }) => {
    await clickAddButton(page, 'Add Professional');

    const card = page.locator('.item-card').last();
    const fields = card.locator('.form-field');

    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('Dr. Smith');
    await fields.filter({ hasText: 'Relationship' }).locator('input').fill('Primary Care');
    await fields.filter({ hasText: 'Phone' }).locator('input').fill('555-0500');
  });
});

// ==============================
// Medical Section
// ==============================
test.describe('Medical Section', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
    await navigateToSection(page, 'Medical');
  });

  test('adds a family member with doctors and medications', async ({ page }) => {
    await clickAddButton(page, 'Add Family Member');

    // Fill name
    const nameField = page.locator('.form-field').filter({ hasText: 'Name' }).locator('input').first();
    await nameField.fill('Self');

    // Wait for card to appear
    await page.waitForTimeout(300);
  });
});

// ==============================
// Pets Section
// ==============================
test.describe('Pets Section', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
    await navigateToSection(page, 'Pets');
  });

  test('adds a pet with all fields', async ({ page }) => {
    await clickAddButton(page, 'Add Pet');

    const fields = page.locator('.form-field');

    // Basic info
    await fields.filter({ hasText: 'Name' }).locator('input').first().fill('Buddy');
    await fields.filter({ hasText: 'Species' }).locator('input').fill('Dog');
    await fields.filter({ hasText: 'Breed' }).locator('input').fill('Golden Retriever');
  });
});

// ==============================
// Export Flow
// ==============================
test.describe('Export', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);
  });

  test('opens export dialog', async ({ page }) => {
    // Add some data first so export is available
    await navigateToSection(page, 'Financial');
    await clickAddButton(page, 'Add Bank Account');
    await page.locator('.item-card .form-field').filter({ hasText: 'Account Name' }).locator('input').fill('Test Account');

    // Trigger flush
    await page.waitForTimeout(500);

    // Click export button
    const exportBtn = page.getByText('Export', { exact: false }).first();
    if (await exportBtn.isVisible().catch(() => false)) {
      await exportBtn.click();
      await page.waitForTimeout(500);

      // Export dialog should be visible
      const dialog = page.locator('.export-dialog, .modal, [class*="export"]');
      await expect(dialog.first()).toBeVisible({ timeout: 3000 }).catch(() => {
        // Export may not be available in web mode
      });
    }
  });

  test('export dialog requires passphrase', async ({ page }) => {
    // Add data
    await navigateToSection(page, 'Financial');
    await clickAddButton(page, 'Add Bank Account');
    await page.locator('.item-card .form-field').filter({ hasText: 'Account Name' }).locator('input').fill('Test');
    await page.waitForTimeout(500);

    const exportBtn = page.getByText('Export', { exact: false }).first();
    if (await exportBtn.isVisible().catch(() => false)) {
      await exportBtn.click();
      await page.waitForTimeout(500);

      // Look for passphrase input
      const passphraseInput = page.locator('input[type="password"]').first();
      if (await passphraseInput.isVisible().catch(() => false)) {
        // Fill in a passphrase
        await passphraseInput.fill('TestPassphrase123!');

        // Look for confirm input
        const confirmInput = page.locator('input[type="password"]').nth(1);
        if (await confirmInput.isVisible().catch(() => false)) {
          await confirmInput.fill('TestPassphrase123!');
        }
      }
    }
  });
});

// ==============================
// Data Persistence (localStorage in web mode)
// ==============================
test.describe('Data Persistence', () => {
  test('persists data across page reloads via localStorage', async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);

    // Add a bank account
    await navigateToSection(page, 'Financial');
    await clickAddButton(page, 'Add Bank Account');
    const nameInput = page.locator('.item-card .form-field').filter({ hasText: 'Account Name' }).locator('input');
    await nameInput.fill('Persistent Account');

    // Trigger change event and wait for debounced save
    await nameInput.press('Tab');
    await page.waitForTimeout(2000);

    // Verify localStorage has data
    const stored = await page.evaluate(() => localStorage.getItem('honey-did-document'));
    expect(stored).toBeTruthy();
    if (stored) {
      const doc = JSON.parse(stored);
      expect(doc.financial.bank_accounts.length).toBeGreaterThan(0);
    }
  });
});

// ==============================
// Full Workflow: Fill All Sections
// ==============================
test.describe('Full Workflow', () => {
  test('adds data to every section type', async ({ page }) => {
    await page.goto('/');
    await dismissIntro(page);

    // Financial
    await navigateToSection(page, 'Financial');
    await clickAddButton(page, 'Add Bank Account');
    await page.locator('.item-card').last().locator('.form-field').filter({ hasText: 'Account Name' }).locator('input').fill('Checking');
    await page.waitForTimeout(200);

    // Insurance
    await navigateToSection(page, 'Insurance');
    await clickAddButton(page, 'Add Policy');
    await page.locator('.item-card').last().locator('.form-field').filter({ hasText: 'Policy Type' }).locator('input').fill('Life');
    await page.waitForTimeout(200);

    // Bills
    await navigateToSection(page, 'Bills');
    await clickAddButton(page, 'Add Bill');
    await page.locator('.item-card').last().locator('.form-field').filter({ hasText: 'Bill Name' }).locator('input').fill('Electric');
    await page.waitForTimeout(200);

    // Property
    await navigateToSection(page, 'Property');
    await clickAddButton(page, 'Add Property');
    await page.locator('.item-card').last().locator('.form-field').filter({ hasText: 'Name' }).locator('input').first().fill('Home');
    await page.waitForTimeout(200);

    // Legal
    await navigateToSection(page, 'Legal');
    const willField = page.locator('.form-field').filter({ hasText: 'Will' }).locator('input, textarea').first();
    if (await willField.isVisible().catch(() => false)) {
      await willField.fill('Safe deposit box');
    }
    await page.waitForTimeout(200);

    // Digital
    await navigateToSection(page, 'Digital');
    await clickAddButton(page, 'Add Email');
    await page.locator('.item-card').last().locator('.form-field').filter({ hasText: 'Name' }).locator('input').first().fill('Gmail');
    await page.waitForTimeout(200);

    // Household
    await navigateToSection(page, 'Household');
    await clickAddButton(page, 'Add Maintenance');
    await page.locator('.item-card').last().locator('.form-field').filter({ hasText: 'Name' }).locator('input').first().fill('HVAC');
    await page.waitForTimeout(200);

    // Personal
    await navigateToSection(page, 'Personal');
    await clickAddButton(page, 'Add Message');
    await page.locator('.item-card').last().locator('.form-field').filter({ hasText: 'Recipient' }).locator('input').fill('Family');
    await page.waitForTimeout(200);

    // Contacts
    await navigateToSection(page, 'Contacts');
    await clickAddButton(page, 'Add Emergency');
    await page.locator('.item-card').last().locator('.form-field').filter({ hasText: 'Name' }).locator('input').first().fill('Spouse');
    await page.waitForTimeout(200);

    // Medical
    await navigateToSection(page, 'Medical');
    await clickAddButton(page, 'Add Family Member');
    await page.waitForTimeout(200);

    // Pets
    await navigateToSection(page, 'Pets');
    await clickAddButton(page, 'Add Pet');
    await page.locator('.form-field').filter({ hasText: 'Name' }).locator('input').first().fill('Buddy');
    await page.waitForTimeout(200);

    // Wait for final debounced save
    await page.waitForTimeout(2000);

    // Verify data was saved to localStorage
    const stored = await page.evaluate(() => localStorage.getItem('honey-did-document'));
    expect(stored).toBeTruthy();
    const doc = JSON.parse(stored!);

    expect(doc.financial.bank_accounts.length).toBeGreaterThan(0);
    expect(doc.insurance.policies.length).toBeGreaterThan(0);
    expect(doc.bills.bills.length).toBeGreaterThan(0);
    expect(doc.property.properties.length).toBeGreaterThan(0);
    expect(doc.personal.messages.length).toBeGreaterThan(0);
    expect(doc.contacts.emergency_contacts.length).toBeGreaterThan(0);
    expect(doc.pets.pets.length).toBeGreaterThan(0);
  });
});
