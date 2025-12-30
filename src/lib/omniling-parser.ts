export interface OmniLangError {
  line: number;
  message: string;
}

export function parseOmniLang(code: string): OmniLangError[] {
  const errors: OmniLangError[] = [];
  const lines = code.split('\n');

  // Define the canonical order of sections as per v1.0 Spec
  const canonicalOrder = [
    'INTENT',
    'ACTOR',
    'CONTEXT',
    'ASSUMPTION', // Optional (handled in logic)
    'RULE',
    'CONSTRAINT',
    'IMPACT',
    'TRACE',
    'REVIEW'      // Optional
  ];

  const foundSections: Map<string, number> = new Map();
  let currentSectionIndex = -1;

  const isHeaderLine = (raw: string) => {
    const m = raw.match(/^\s*([A-Za-z]+):/);
    return m ? m[1].toUpperCase() : null;
  };

  const hasListItemsBeforeNextHeader = (startIdx: number) => {
    for (let j = startIdx + 1; j < lines.length; j++) {
      const look = lines[j].trim();
      if (look.length === 0 || look.startsWith('//') || look.startsWith('#')) continue;
      if (isHeaderLine(look)) return false; // hit next header first
      if (look.startsWith('-')) return true; // found list entry
      // regular text counts as content too
      return true;
    }
    return false;
  };

  lines.forEach((line, index) => {
    const lineNumber = index + 1;
    const trimmedLine = line.trim();

    // Skip empty lines or comments
    if (trimmedLine.length === 0 || trimmedLine.startsWith('//') || trimmedLine.startsWith('#')) {
      return;
    }

    const keyword = isHeaderLine(trimmedLine);
    if (keyword) {
      if (!canonicalOrder.includes(keyword)) {
        errors.push({
          line: lineNumber,
          message: `Unknown keyword '${keyword}'. Expected one of: ${canonicalOrder.join(', ')}`
        });
        return;
      }

      if (foundSections.has(keyword)) {
        errors.push({
          line: lineNumber,
          message: `Duplicate section '${keyword}'. Each section should appear only once.`
        });
      }
      foundSections.set(keyword, lineNumber);

      const newSectionIndex = canonicalOrder.indexOf(keyword);
      if (newSectionIndex < currentSectionIndex) {
        errors.push({
          line: lineNumber,
          message: `Section '${keyword}' appears out of order. Canonical order violation.`
        });
      }
      currentSectionIndex = newSectionIndex;

      const hasInlineContent = trimmedLine.length > keyword.length + 1;
      if (!hasInlineContent && !hasListItemsBeforeNextHeader(index)) {
        errors.push({
          line: lineNumber,
          message: `Section '${keyword}' must have content or list items.`
        });
      }
    } else if (trimmedLine.startsWith('-')) {
      if (currentSectionIndex === -1) {
        errors.push({
          line: lineNumber,
          message: "List item found outside of any section."
        });
      }
    } else {
      if (currentSectionIndex === -1) {
        errors.push({
          line: lineNumber,
          message: "Content found before any valid section header (e.g., INTENT:)."
        });
      }
    }
  });

  const mandatorySections = ['INTENT', 'ACTOR', 'CONTEXT', 'RULE', 'CONSTRAINT', 'IMPACT', 'TRACE'];

  mandatorySections.forEach(section => {
    if (!foundSections.has(section)) {
      errors.push({
        line: lines.length + 1,
        message: `Missing mandatory section: '${section}'.`
      });
    }
  });

  return errors;
}

