import type { Page } from "playwright";

type StateDelta = {
  state: string;
  target_node_id?: string;
  target_signature?: string;
  changed_properties: Record<string, { before: string; after: string }>;
  safe_interaction: boolean;
};

const TRACKED_PROPERTIES = [
  "color",
  "background-color",
  "border-color",
  "border-radius",
  "box-shadow",
  "outline-color",
  "outline-style",
  "outline-width",
  "opacity",
  "transform"
];

export async function captureStateDeltas(
  page: Page,
  states: string[],
  options: { safeInteractions: boolean; noClick: boolean; noFormSubmit: boolean }
): Promise<StateDelta[]> {
  if (states.length === 0 || !options.safeInteractions) {
    return [];
  }

  const allowedStates = states.filter((state) => ["hover", "focus", "focus-visible"].includes(state));
  if (allowedStates.length === 0) {
    return [];
  }

  const targets = await page.evaluate(() => {
    const elements = Array.from(document.querySelectorAll("*"));
    return elements
      .map((element, index) => {
        const tag = element.tagName.toLowerCase();
        const role = element.getAttribute("role");
        const interactive = tag === "button"
          || tag === "a"
          || tag === "input"
          || tag === "select"
          || tag === "textarea"
          || role === "button"
          || role === "link"
          || role === "tab"
          || role === "menuitem";
        return interactive ? {
          index,
          nodeId: `node_${index + 1}`,
          signature: role ? `${tag}[role=${role}]` : tag
        } : undefined;
      })
      .filter(Boolean)
      .slice(0, 20) as Array<{ index: number; nodeId: string; signature: string }>;
  });

  const deltas: StateDelta[] = [];
  for (const target of targets) {
    for (const state of allowedStates) {
      const locator = page.locator("*").nth(target.index);
      const before = await readTrackedStyle(page, target.index);
      if (state === "hover") {
        await locator.hover({ trial: false }).catch(() => undefined);
      } else {
        await locator.focus().catch(() => undefined);
      }
      const after = await readTrackedStyle(page, target.index);
      const changed_properties = diffStyles(before, after);
      if (Object.keys(changed_properties).length > 0) {
        deltas.push({
          state,
          target_node_id: target.nodeId,
          target_signature: target.signature,
          changed_properties,
          safe_interaction: true
        });
      }
      await page.mouse.move(0, 0).catch(() => undefined);
    }
  }

  return deltas;
}

async function readTrackedStyle(page: Page, index: number): Promise<Record<string, string>> {
  return page.evaluate((payload) => {
    const element = document.querySelectorAll("*").item(payload.index);
    if (!element) {
      return {};
    }
    const style = getComputedStyle(element);
    const values: Record<string, string> = {};
    for (const property of payload.properties) {
      values[property] = style.getPropertyValue(property);
    }
    return values;
  }, { index, properties: TRACKED_PROPERTIES });
}

function diffStyles(
  before: Record<string, string>,
  after: Record<string, string>
): Record<string, { before: string; after: string }> {
  const diff: Record<string, { before: string; after: string }> = {};
  for (const property of TRACKED_PROPERTIES) {
    if (before[property] !== after[property]) {
      diff[toSnakeCase(property)] = {
        before: before[property] ?? "",
        after: after[property] ?? ""
      };
    }
  }
  return diff;
}

function toSnakeCase(value: string): string {
  return value.replaceAll("-", "_");
}
