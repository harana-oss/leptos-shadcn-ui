import { test, expect, Page } from '@playwright/test';

/**
 * Select Component Keyboard + ARIA E2E Tests
 *
 * Follows the WAI-ARIA select-only combobox pattern implemented in
 * packages/leptos/select (see src/keyboard.rs for the pure key logic).
 */

test.describe('Select Keyboard and ARIA', () => {
  let page: Page;

  test.beforeEach(async ({ page: testPage }) => {
    page = testPage;
    await page.goto('/components/select');
    await page.waitForLoadState('networkidle');
  });

  const trigger = () => page.getByRole('combobox').first();
  const listboxFor = async () => {
    const id = await trigger().getAttribute('aria-controls');
    expect(id).toBeTruthy();
    return page.locator(`[id="${id}"]`);
  };

  test('trigger exposes combobox ARIA wiring', async () => {
    const t = trigger();
    await expect(t).toHaveAttribute('aria-haspopup', 'listbox');
    await expect(t).toHaveAttribute('aria-expanded', 'false');
    const listbox = await listboxFor();
    await expect(listbox).toHaveAttribute('role', 'listbox');
    await expect(listbox).toBeHidden();
  });

  test('listbox ids are unique per instance', async () => {
    const ids = await page
      .getByRole('combobox')
      .evaluateAll((els) => els.map((el) => el.getAttribute('aria-controls')));
    expect(new Set(ids).size).toBe(ids.length);
  });

  for (const key of ['Enter', ' ', 'ArrowDown']) {
    test(`"${key === ' ' ? 'Space' : key}" opens the listbox`, async () => {
      await trigger().focus();
      await page.keyboard.press(key === ' ' ? 'Space' : key);
      await expect(trigger()).toHaveAttribute('aria-expanded', 'true');
      await expect(await listboxFor()).toBeVisible();
      await expect(trigger()).toHaveAttribute('aria-activedescendant', /.+/);
    });
  }

  test('arrow keys, Home and End move the active option', async () => {
    await trigger().focus();
    await page.keyboard.press('ArrowDown');
    const listbox = await listboxFor();
    const options = listbox.locator('[role="option"]:not([aria-disabled="true"])');
    const count = await options.count();
    test.skip(count < 2, 'needs at least two enabled options');

    const activeId = () => trigger().getAttribute('aria-activedescendant');
    const firstId = await options.first().getAttribute('id');
    const lastId = await options.last().getAttribute('id');

    await page.keyboard.press('Home');
    expect(await activeId()).toBe(firstId);
    await page.keyboard.press('ArrowDown');
    expect(await activeId()).toBe(await options.nth(1).getAttribute('id'));
    await page.keyboard.press('End');
    expect(await activeId()).toBe(lastId);
    await page.keyboard.press('ArrowUp');
    expect(await activeId()).toBe(await options.nth(count - 2).getAttribute('id'));
    await expect(page.locator(`[id="${lastId}"]`)).not.toHaveAttribute('data-highlighted', /.*/);
  });

  test('Enter selects the active option and closes', async () => {
    await trigger().focus();
    await page.keyboard.press('ArrowDown');
    await page.keyboard.press('End');
    const activeId = await trigger().getAttribute('aria-activedescendant');
    await page.keyboard.press('Enter');
    await expect(trigger()).toHaveAttribute('aria-expanded', 'false');
    await expect(page.locator(`[id="${activeId}"]`)).toHaveAttribute('aria-selected', 'true');
    await expect(trigger()).toBeFocused();
  });

  test('Escape closes and returns focus to the trigger', async () => {
    await trigger().focus();
    await page.keyboard.press('Enter');
    await expect(trigger()).toHaveAttribute('aria-expanded', 'true');
    await page.keyboard.press('Escape');
    await expect(trigger()).toHaveAttribute('aria-expanded', 'false');
    await expect(trigger()).toBeFocused();
  });

  test('clicking outside closes the listbox', async () => {
    await trigger().click();
    await expect(trigger()).toHaveAttribute('aria-expanded', 'true');
    await page.mouse.click(1, 1);
    await expect(trigger()).toHaveAttribute('aria-expanded', 'false');
  });

  test('renders a hidden input for native forms', async () => {
    const input = page.locator('input[type="hidden"][data-select-input]').first();
    await expect(input).toHaveCount(1);
    await trigger().focus();
    await page.keyboard.press('ArrowDown');
    await page.keyboard.press('Home');
    const activeId = await trigger().getAttribute('aria-activedescendant');
    const value = await page.locator(`[id="${activeId}"]`).getAttribute('data-value');
    await page.keyboard.press('Enter');
    await expect(input).toHaveValue(value ?? '');
  });
});
