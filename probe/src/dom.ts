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
  pseudo_elements: unknown[];
  assets: unknown[];
  stylesheet_provenance: unknown;
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
    const pseudoElements = [];
    const assets = [];

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
        background_image: style.backgroundImage,
        mask_image: style.getPropertyValue("mask-image") || style.getPropertyValue("-webkit-mask-image"),
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
        transform: style.transform,
        css_variables: cssVariables(style)
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

      for (const pseudo of ["::before", "::after"] as const) {
        const pseudoFact = pseudoElementFact(element, nodeId, pseudo, view);
        if (pseudoFact) {
          pseudoElements.push(pseudoFact);
        }
      }

      assets.push(...await assetFacts(element, nodeId, style, assets.length));
    }

    assets.push(...await faviconFacts(assets.length));

    return {
      url: location.href,
      title: document.title,
      user_agent_hash: await sha256(navigator.userAgent),
      dom: { nodes: domNodes },
      cssom: { computed_styles: computedStyles },
      layout: { boxes: layoutBoxes },
      accessibility: { nodes: accessibilityNodes },
      screenshots: [],
      pseudo_elements: pseudoElements,
      assets,
      stylesheet_provenance: await stylesheetProvenance(ids),
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

    function cssVariables(style: CSSStyleDeclaration): Record<string, string> {
      const variables: Record<string, string> = {};
      for (let index = 0; index < style.length; index += 1) {
        const property = style.item(index);
        if (property.startsWith("--")) {
          variables[property] = style.getPropertyValue(property).trim();
        }
      }
      return variables;
    }

    function pseudoElementFact(
      element: Element,
      nodeId: string,
      pseudo: "::before" | "::after",
      view: Window
    ): Record<string, unknown> | undefined {
      const style = view.getComputedStyle(element, pseudo);
      const content = style.content;
      const hasContent = Boolean(content && content !== "none" && content !== "normal" && content !== "\"\"");
      const hasPaint = style.backgroundImage !== "none"
        || style.getPropertyValue("mask-image") !== "none"
        || style.getPropertyValue("-webkit-mask-image") !== "none"
        || style.borderWidth !== "0px";
      const rect = element.getBoundingClientRect();
      const width = parseCssPx(style.width) ?? 0;
      const height = parseCssPx(style.height) ?? 0;
      const visible = (hasContent || hasPaint)
        && style.display !== "none"
        && style.visibility !== "hidden"
        && style.opacity !== "0";
      if (!visible) {
        return undefined;
      }
      return {
        node_id: nodeId,
        pseudo,
        content: hasContent ? content : undefined,
        computed_style: {
          display: style.display,
          position: style.position,
          color: style.color,
          background_color: style.backgroundColor,
          background_image: style.backgroundImage,
          mask_image: style.getPropertyValue("mask-image") || style.getPropertyValue("-webkit-mask-image"),
          width: style.width,
          height: style.height,
          margin: style.margin,
          padding: style.padding,
          border_radius: style.borderRadius,
          border_width: style.borderWidth,
          box_shadow: style.boxShadow,
          opacity: style.opacity,
          transform: style.transform
        },
        layout_estimate: {
          id: `box:${nodeId}:${pseudo}`,
          node_id: nodeId,
          x: round(rect.x),
          y: round(rect.y),
          width: round(width || rect.width),
          height: round(height || rect.height),
          visible
        },
        visible
      };
    }

    async function assetFacts(
      element: Element,
      nodeId: string,
      style: CSSStyleDeclaration,
      startIndex: number
    ): Promise<Record<string, unknown>[]> {
      const facts: Record<string, unknown>[] = [];
      const tag = element.tagName.toLowerCase();
      const pushAsset = async (
        kind: string,
        url: string,
        usage?: string,
        intrinsic_size?: { width: number; height: number }
      ) => {
        if (!url || url === "none") {
          return;
        }
        facts.push({
          id: `asset:${options.viewport.width}x${options.viewport.height}:${String(startIndex + facts.length + 1).padStart(3, "0")}`,
          kind,
          source_node_id: nodeId,
          url_hash: await sha256(resolveCssUrl(url) ?? url),
          resolved_url_redacted: true,
          intrinsic_size,
          usage
        });
      };

      if (tag === "img") {
        const image = element as HTMLImageElement;
        await pushAsset("image", image.currentSrc || image.src, "content-image", {
          width: image.naturalWidth || Math.round(image.getBoundingClientRect().width),
          height: image.naturalHeight || Math.round(image.getBoundingClientRect().height)
        });
      }
      if (tag === "picture") {
        const source = element.querySelector("source[srcset]");
        if (source) {
          await pushAsset("picture-source", source.getAttribute("srcset") ?? "", "responsive-image");
        }
      }
      if (tag === "svg") {
        const rect = element.getBoundingClientRect();
        await pushAsset("inline-svg", `inline-svg:${nodeId}:${Math.round(rect.width)}x${Math.round(rect.height)}`, "logo-or-icon-candidate", {
          width: Math.round(rect.width),
          height: Math.round(rect.height)
        });
      }
      await pushAsset("background-image", style.backgroundImage, "background-or-sprite-candidate");
      await pushAsset("mask-image", style.getPropertyValue("mask-image") || style.getPropertyValue("-webkit-mask-image"), "mask-or-icon-candidate");
      return facts;
    }

    async function faviconFacts(startIndex: number): Promise<Record<string, unknown>[]> {
      const icons = Array.from(document.querySelectorAll("link[rel~='icon'], link[rel='shortcut icon'], link[rel='apple-touch-icon']"));
      const facts: Record<string, unknown>[] = [];
      for (const icon of icons) {
        const href = icon.getAttribute("href");
        if (!href) {
          continue;
        }
        facts.push({
          id: `asset:${options.viewport.width}x${options.viewport.height}:${String(startIndex + facts.length + 1).padStart(3, "0")}`,
          kind: "favicon",
          url_hash: await sha256(new URL(href, location.href).href),
          resolved_url_redacted: true,
          usage: "favicon"
        });
      }
      return facts;
    }

    function resolveCssUrl(value: string): string | undefined {
      const match = /url\((['"]?)(.*?)\1\)/.exec(value);
      const raw = match?.[2] ?? value;
      if (!raw || raw === "none") {
        return undefined;
      }
      try {
        return new URL(raw, location.href).href;
      } catch {
        return raw;
      }
    }

    function parseCssPx(value: string): number | undefined {
      const parsed = Number(value.trim().replace(/px$/, ""));
      return Number.isFinite(parsed) ? parsed : undefined;
    }

    async function stylesheetProvenance(ids: Map<Element, string>): Promise<Record<string, unknown>> {
      const stylesheets: Array<Record<string, unknown>> = [];
      const mediaQueries: Array<Record<string, unknown>> = [];
      const fontFaces: Array<Record<string, unknown>> = [];
      const cssVariablesList: Array<Record<string, unknown>> = [];
      const matchedRules: Array<Record<string, unknown>> = [];

      for (const sheet of Array.from(document.styleSheets)) {
        const href = sheet.href ?? undefined;
        const owner = sheet.ownerNode instanceof Element ? sheet.ownerNode : undefined;
        const inlineText = href ? undefined : owner?.textContent ?? "";
        const sheetId = `stylesheet:${stylesheets.length + 1}`;
        stylesheets.push({
          id: sheetId,
          href_hash: href ? await sha256(new URL(href, location.href).href) : undefined,
          inline_hash: inlineText ? await sha256(inlineText) : undefined,
          origin: href ? "external" : "inline",
          rules_accessible: safeCssRules(sheet).accessible
        });
        const rules = safeCssRules(sheet).rules;
        for (const rule of rules) {
          const cssText = rule.cssText;
          if (rule instanceof CSSMediaRule) {
            mediaQueries.push({
              stylesheet_id: sheetId,
              condition_text: rule.conditionText,
              rule_hash: await sha256(cssText)
            });
          } else if (typeof CSSFontFaceRule !== "undefined" && rule instanceof CSSFontFaceRule) {
            fontFaces.push({
              stylesheet_id: sheetId,
              rule_hash: await sha256(cssText),
              css_text_hash: await sha256(cssText)
            });
          }
        }
      }

      for (const [element, nodeId] of ids) {
        const style = getComputedStyle(element);
        const variables = cssVariables(style);
        for (const [name, value] of Object.entries(variables)) {
          cssVariablesList.push({
            node_id: nodeId,
            name,
            value_hash: await sha256(value),
            value_redacted: true
          });
        }
        const matched = matchedCssRules(element).slice(0, 20);
        for (const rule of matched) {
          matchedRules.push({
            node_id: nodeId,
            selector_hash: await sha256(rule.selectorText),
            selector_redacted: true,
            media: rule.parentRule instanceof CSSMediaRule ? rule.parentRule.conditionText : undefined,
            rule_hash: await sha256(rule.cssText)
          });
        }
      }

      return {
        stylesheets,
        css_variables: cssVariablesList,
        font_faces: fontFaces,
        media_queries: mediaQueries,
        matched_rules: matchedRules
      };
    }

    function safeCssRules(sheet: CSSStyleSheet): { accessible: boolean; rules: CSSRule[] } {
      try {
        return { accessible: true, rules: Array.from(sheet.cssRules) };
      } catch {
        return { accessible: false, rules: [] };
      }
    }

    function matchedCssRules(element: Element): CSSStyleRule[] {
      const rules: CSSStyleRule[] = [];
      for (const sheet of Array.from(document.styleSheets)) {
        for (const rule of safeCssRules(sheet).rules) {
          collectMatchedRule(element, rule, rules);
        }
      }
      return rules;
    }

    function collectMatchedRule(element: Element, rule: CSSRule, output: CSSStyleRule[]): void {
      if (rule instanceof CSSStyleRule) {
        try {
          if (element.matches(rule.selectorText)) {
            output.push(rule);
          }
        } catch {
          return;
        }
      } else if (rule instanceof CSSMediaRule) {
        for (const child of Array.from(rule.cssRules)) {
          collectMatchedRule(element, child, output);
        }
      }
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
