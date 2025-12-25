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

  lines.forEach((line, index) => {
    const lineNumber = index + 1;
    const trimmedLine = line.trim();

    // Skip empty lines or comments
    if (trimmedLine.length === 0 || trimmedLine.startsWith('//') || trimmedLine.startsWith('#')) {
      return;
    }

    // Check if line starts with a keyword followed by colon
    const match = trimmedLine.match(/^([A-Z]+):/);
    if (match) {
      const keyword = match[1];

      // Validate if it is a valid core keyword
      if (!canonicalOrder.includes(keyword)) {
        errors.push({
          line: lineNumber,
          message: `Unknown keyword '${keyword}'. Expected one of: ${canonicalOrder.join(', ')}`
        });
        return;
      }

      // Record found section
      if (foundSections.has(keyword)) {
        errors.push({
          line: lineNumber,
          message: `Duplicate section '${keyword}'. Each section should appear only once.`
        });
      }
      foundSections.set(keyword, lineNumber);

      // Validate Order
      const newSectionIndex = canonicalOrder.indexOf(keyword);
      if (newSectionIndex < currentSectionIndex) {
        errors.push({
          line: lineNumber,
          message: `Section '${keyword}' appears out of order. Canonical order violation.`
        });
      }
      currentSectionIndex = newSectionIndex;

      // Validate Content Presence (Basic)
      if (trimmedLine.length <= keyword.length + 1) {
        errors.push({
          line: lineNumber,
          message: `Section '${keyword}' must have content.`
        });
      }
    } else if (trimmedLine.startsWith('-')) {
      // List item, generally checking if inside a section
      if (currentSectionIndex === -1) {
        errors.push({
          line: lineNumber,
          message: "List item found outside of any section."
        });
      }
    } else {
      // Regular text line. If not part of a section (no header seen yet), it might be invalid
      if (currentSectionIndex === -1) {
        errors.push({
          line: lineNumber,
          message: "Content found before any valid section header (e.g., INTENT:)."
        });
      }
    }
  });

  // Final Validation: Check for Missing Mandatory Sections
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

