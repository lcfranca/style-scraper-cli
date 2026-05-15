import type { Page } from "playwright";

export async function captureAccessibilityTree(page: Page): Promise<unknown | null> {
  const accessibility = (page as unknown as {
    accessibility?: {
      snapshot(options?: { interestingOnly?: boolean }): Promise<unknown>;
    };
  }).accessibility;

  if (!accessibility?.snapshot) {
    return null;
  }

  return accessibility.snapshot({ interestingOnly: false }).catch(() => null);
}
