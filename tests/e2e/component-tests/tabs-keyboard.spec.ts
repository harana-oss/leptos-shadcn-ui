import { test, expect, Page } from '@playwright/test';

/**
 * Tabs Component Keyboard + ARIA E2E Tests
 *
 * Follows the WAI-ARIA tabs pattern implemented in packages/leptos/tabs
 * (roving tabindex, automatic activation; see src/keyboard.rs).
 */

test.describe('Tabs Keyboard and ARIA', () => {
  let page: Page;

  test.beforeEach(async ({ page: testPage }) => {
    page = testPage;
    await page.goto('/components/tabs');
    await page.waitForLoadState('networkidle');
  });

  const tablist = () => page.getByRole('tablist').first();
  const tabs = () => tablist().getByRole('tab');
  const enabledTabs = () => tablist().locator('[role="tab"]:not([disabled])');

  test('roving tabindex: only the selected tab is tabbable', async () => {
    const selected = tablist().locator('[role="tab"][aria-selected="true"]');
    await expect(selected).toHaveCount(1);
    await expect(selected).toHaveAttribute('tabindex', '0');
    await expect(tablist().locator('[role="tab"][tabindex="0"]')).toHaveCount(1);
  });

  test('tabs and panels are linked by id', async () => {
    const count = await tabs().count();
    for (let i = 0; i < count; i++) {
      const tab = tabs().nth(i);
      const tabId = await tab.getAttribute('id');
      const panelId = await tab.getAttribute('aria-controls');
      expect(tabId).toBeTruthy();
      expect(panelId).toBeTruthy();
      const panel = page.locator(`[id="${panelId}"]`);
      if ((await panel.count()) > 0) {
        await expect(panel).toHaveAttribute('role', 'tabpanel');
        await expect(panel).toHaveAttribute('aria-labelledby', tabId!);
        await expect(panel).not.toHaveAttribute('aria-selected', /.*/);
      }
    }
  });

  test('ids are unique across Tabs instances', async () => {
    const ids = await page
      .getByRole('tab')
      .evaluateAll((els) => els.map((el) => el.id));
    expect(new Set(ids).size).toBe(ids.length);
  });

  test('ArrowRight/ArrowLeft move focus and activate, wrapping', async () => {
    const count = await enabledTabs().count();
    test.skip(count < 2, 'needs at least two enabled tabs');

    await enabledTabs().first().click();
    await page.keyboard.press('ArrowRight');
    await expect(enabledTabs().nth(1)).toBeFocused();
    await expect(enabledTabs().nth(1)).toHaveAttribute('aria-selected', 'true');
    await expect(enabledTabs().nth(1)).toHaveAttribute('tabindex', '0');

    await page.keyboard.press('ArrowLeft');
    await expect(enabledTabs().first()).toBeFocused();
    await page.keyboard.press('ArrowLeft');
    await expect(enabledTabs().last()).toBeFocused();
  });

  test('Home and End jump to first and last enabled tab', async () => {
    await enabledTabs().first().click();
    await page.keyboard.press('End');
    await expect(enabledTabs().last()).toBeFocused();
    await expect(enabledTabs().last()).toHaveAttribute('aria-selected', 'true');
    await page.keyboard.press('Home');
    await expect(enabledTabs().first()).toBeFocused();
  });

  test('disabled tabs are skipped and not clickable', async () => {
    const disabled = page.locator('[role="tab"][disabled]').first();
    test.skip((await disabled.count()) === 0, 'demo has no disabled tab');
    await expect(disabled).toHaveAttribute('data-disabled', '');
    await disabled.click({ force: true });
    await expect(disabled).toHaveAttribute('aria-selected', 'false');

    const list = disabled.locator('xpath=ancestor::*[@role="tablist"][1]');
    const all = list.locator('[role="tab"]');
    const n = await all.count();
    for (let i = 0; i < n; i++) {
      await list.locator('[role="tab"]:not([disabled])').first().click();
      await page.keyboard.press('ArrowRight');
      await expect(disabled).not.toBeFocused();
    }
  });
});
