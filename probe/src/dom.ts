import type { Page } from "playwright";
import type { CaptureRequest } from "./schema";

export type ObservedPageFacts = {
  url: string;
  title: string;
  user_agent_hash?: string;
  dom: { nodes: unknown[] };
  cssom: { computed_styles: unknown[] };
  layout: { boxes: unknown[] };
  accessibility: { nodes: unknown[]; tree?: unknown };
  screenshots: unknown[];
  diagnostics: Array<{ code: string; message: string; phase: string; severity: string }>;
};

export async function collectObservedPage(
  page: Page,
  request: CaptureRequest
): Promise<ObservedPageFacts> {
  return page.evaluate(async (options) => {
    const diagnostics: Array<{ code: string; message: string; phase: string; severity: string }> = [];
    const elements: Element[] = [];
    const parentOverrides = new Map<Element, Element | null>();
    const shadowHosts = new Map<Element, Element>();

    collectDocument(document, null);
    collectAccessibleIframes();

    const ids = new Map<Element, string>();
    elements.forEach((element, index) => ids.set(element, `node_${index + 1}`));

    const domNodes = [];
    const computedStyles = [];
    const layoutBoxes = [];
    const accessibilityNodes = [];

    for (const [index, element] of elements.entries()) {
      const nodeId = ids.get(element)!;
      const parentElement = parentOverrides.has(element)
        ? parentOverrides.get(element)
        : element.parentElement;
      const parentId = parentElement ? ids.get(parentElement) ?? null : null;
      const rect = element.getBoundingClientRect();
      const view = element.ownerDocument.defaultView ?? window;
      const style = view.getComputedStyle(element);
      const text = normalizedText((element as HTMLElement).innerText ?? element.textContent ?? "");
      const visible = rect.width > 0
        && rect.height > 0
        && style.display !== "none"
        && style.visibility !== "hidden"
        && style.opacity !== "0";
      const role = explicitOrNativeRole(element);
      const name = accessibleNameSeed(element, text);

      domNodes.push({
        id: nodeId,
        parent_id: parentId,
        tag: element.tagName.toLowerCase(),
        attributes: safeAttributes(element, options.redactAttributes, shadowHosts.get(element) ? ids.get(shadowHosts.get(element)!) : undefined),
        text_hash: text && options.hashText && !options.redactText ? await sha256(text) : undefined,
        text_value: text && options.includeText && !options.redactText ? text : undefined,
        text_length: text.length,
        visible,
        child_index: index
      });

      computedStyles.push({
        id: `style:${nodeId}`,
        node_id: nodeId,
        display: style.display,
        position: style.position,
        color: style.color,
        background_color: style.backgroundColor,
        font_family: style.fontFamily,
        font_size: style.fontSize,
        font_weight: style.fontWeight,
        line_height: style.lineHeight,
        letter_spacing: style.letterSpacing,
        padding: style.padding,
        margin: style.margin,
        gap: style.gap,
        row_gap: style.rowGap,
        column_gap: style.columnGap,
        border_radius: style.borderRadius,
        border_width: style.borderWidth,
        border_style: style.borderStyle,
        border_color: style.borderColor,
        box_shadow: style.boxShadow,
        opacity: style.opacity,
        z_index: style.zIndex,
        transition_duration: style.transitionDuration,
        transition_timing_function: style.transitionTimingFunction,
        animation_duration: style.animationDuration,
        transform: style.transform
      });

      layoutBoxes.push({
        id: `box:${nodeId}`,
        node_id: nodeId,
        x: round(rect.x),
        y: round(rect.y),
        width: round(rect.width),
        height: round(rect.height),
        z_index: parseZIndex(style.zIndex),
        visible
      });

      if (role) {
        accessibilityNodes.push({
          id: `aom:${nodeId}`,
          node_id: nodeId,
          role,
          name_hash: name ? await sha256(name) : undefined,
          disabled: (element as HTMLButtonElement).disabled === true || element.getAttribute("aria-disabled") === "true",
          focused: document.activeElement === element,
          checked: parseAriaBoolean(element.getAttribute("aria-checked")),
          expanded: parseAriaBoolean(element.getAttribute("aria-expanded")),
          selected: parseAriaBoolean(element.getAttribute("aria-selected")),
          heading_level: headingLevel(element)
        });
      }
    }

    return {
      url: location.href,
      title: document.title,
      user_agent_hash: await sha256(navigator.userAgent),
      dom: { nodes: domNodes },
      cssom: { computed_styles: computedStyles },
      layout: { boxes: layoutBoxes },
      accessibility: { nodes: accessibilityNodes },
      screenshots: [],
      diagnostics
    };

    function safeAttributes(
      element: Element,
      redactAttributes: boolean,
      shadowHostId?: string
    ): Record<string, string> {
      const allowed = ["id", "class", "role", "type", "aria-current"];
      const attributes: Record<string, string> = {};
      if (shadowHostId) {
        attributes["shadow-host-id"] = shadowHostId;
      }
      if (redactAttributes) {
        const role = element.getAttribute("role");
        const type = element.getAttribute("type");
        if (role) attributes.role = role;
        if (type) attributes.type = type;
        return attributes;
      }
      for (const name of allowed) {
        const value = element.getAttribute(name);
        if (value) {
          attributes[name] = value;
        }
      }
      return attributes;
    }

    function normalizedText(text: string): string {
      return text.replace(/\s+/g, " ").trim();
    }

    function explicitOrNativeRole(element: Element): string | undefined {
      const explicit = element.getAttribute("role");
      if (explicit) {
        return explicit;
      }
      const tag = element.tagName.toLowerCase();
      if (tag === "button") return "button";
      if (tag === "a" && element.hasAttribute("href")) return "link";
      if (tag === "input" || tag === "textarea") return "textbox";
      if (tag === "select") return "combobox";
      if (tag === "img") return "img";
      if (tag === "nav") return "navigation";
      if (tag === "header") return "banner";
      if (tag === "main") return "main";
      if (tag === "footer") return "contentinfo";
      if (tag === "aside") return "complementary";
      if (/^h[1-6]$/.test(tag)) return "heading";
      return undefined;
    }

    function accessibleNameSeed(element: Element, text: string): string {
      return element.getAttribute("aria-label")
        ?? element.getAttribute("alt")
        ?? element.getAttribute("title")
        ?? text;
    }

    function headingLevel(element: Element): number | undefined {
      const tag = element.tagName.toLowerCase();
      if (/^h[1-6]$/.test(tag)) {
        return Number(tag.slice(1));
      }
      const ariaLevel = element.getAttribute("aria-level");
      return ariaLevel ? Number(ariaLevel) : undefined;
    }

    function parseAriaBoolean(value: string | null): boolean | undefined {
      if (value === "true") return true;
      if (value === "false") return false;
      return undefined;
    }

    function parseZIndex(value: string): number | undefined {
      if (value === "auto") {
        return undefined;
      }
      const parsed = Number(value);
      return Number.isFinite(parsed) ? parsed : undefined;
    }

    function round(value: number): number {
      return Math.round(value * 1000) / 1000;
    }

    async function sha256(value: string): Promise<string> {
      const bytes = new TextEncoder().encode(value);
      const digest = await crypto.subtle.digest("SHA-256", bytes);
      return `sha256:${Array.from(new Uint8Array(digest)).map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
    }

    function collectDocument(rootDocument: Document, ownerFrame: Element | null): void {
      for (const element of Array.from(rootDocument.querySelectorAll("*"))) {
        if (ownerFrame && !parentOverrides.has(element)) {
          parentOverrides.set(element, element.parentElement ?? ownerFrame);
        }
        elements.push(element);

        const shadowRoot = element.shadowRoot;
        if (shadowRoot) {
          diagnostics.push({
            code: "OPEN_SHADOW_DOM_CAPTURED",
            message: `Captured open Shadow DOM for <${element.tagName.toLowerCase()}>.`,
            phase: "capture",
            severity: "info"
          });
          for (const shadowElement of Array.from(shadowRoot.querySelectorAll("*"))) {
            shadowHosts.set(shadowElement, element);
            parentOverrides.set(shadowElement, shadowElement.parentElement ?? element);
            elements.push(shadowElement);
          }
        }
      }
    }

    function collectAccessibleIframes(): void {
      for (const iframe of Array.from(document.querySelectorAll("iframe"))) {
        try {
          const frameDocument = iframe.contentDocument;
          if (!frameDocument) {
            diagnostics.push({
              code: "IFRAME_UNAVAILABLE",
              message: "Iframe document was unavailable or cross-origin.",
              phase: "capture",
              severity: "warning"
            });
            continue;
          }
          diagnostics.push({
            code: "SAME_ORIGIN_IFRAME_CAPTURED",
            message: "Captured accessible same-origin iframe DOM/CSSOM/layout evidence.",
            phase: "capture",
            severity: "info"
          });
          collectDocument(frameDocument, iframe);
        } catch {
          diagnostics.push({
            code: "CROSS_ORIGIN_IFRAME_LIMITED",
            message: "Cross-origin iframe cannot be inspected; browser security boundary preserved.",
            phase: "capture",
            severity: "warning"
          });
        }
      }
    }
  }, request);
}

function safeAttributes(
  element: Element,
  redactAttributes: boolean,
  shadowHostId?: string
): Record<string, string> {
  const allowed = ["id", "class", "role", "type", "aria-current"];
  const attributes: Record<string, string> = {};
  if (shadowHostId) {
    attributes["shadow-host-id"] = shadowHostId;
  }
  if (redactAttributes) {
    const role = element.getAttribute("role");
    const type = element.getAttribute("type");
    if (role) attributes.role = role;
    if (type) attributes.type = type;
    return attributes;
  }
  for (const name of allowed) {
    const value = element.getAttribute(name);
    if (value) {
      attributes[name] = value;
    }
  }
  return attributes;
}

function normalizedText(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

function explicitOrNativeRole(element: Element): string | undefined {
  const explicit = element.getAttribute("role");
  if (explicit) {
    return explicit;
  }
  const tag = element.tagName.toLowerCase();
  if (tag === "button") return "button";
  if (tag === "a" && element.hasAttribute("href")) return "link";
  if (tag === "input" || tag === "textarea") return "textbox";
  if (tag === "select") return "combobox";
  if (tag === "img") return "img";
  if (tag === "nav") return "navigation";
  if (tag === "header") return "banner";
  if (tag === "main") return "main";
  if (tag === "footer") return "contentinfo";
  if (tag === "aside") return "complementary";
  if (/^h[1-6]$/.test(tag)) return "heading";
  return undefined;
}

function accessibleNameSeed(element: Element, text: string): string {
  return element.getAttribute("aria-label")
    ?? element.getAttribute("alt")
    ?? element.getAttribute("title")
    ?? text;
}

function headingLevel(element: Element): number | undefined {
  const tag = element.tagName.toLowerCase();
  if (/^h[1-6]$/.test(tag)) {
    return Number(tag.slice(1));
  }
  const ariaLevel = element.getAttribute("aria-level");
  return ariaLevel ? Number(ariaLevel) : undefined;
}

function parseAriaBoolean(value: string | null): boolean | undefined {
  if (value === "true") return true;
  if (value === "false") return false;
  return undefined;
}

function parseZIndex(value: string): number | undefined {
  if (value === "auto") {
    return undefined;
  }
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : undefined;
}

function round(value: number): number {
  return Math.round(value * 1000) / 1000;
}

async function sha256(value: string): Promise<string> {
  const bytes = new TextEncoder().encode(value);
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return `sha256:${Array.from(new Uint8Array(digest)).map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}
