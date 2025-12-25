'use server';

/**
 * @fileOverview This file defines a Genkit flow for providing AI-powered code autocompletion suggestions for the OmniLang programming language.
 *
 * The flow takes the current code context as input and returns a list of suggested code completions.
 *
 * - codeAutocompletion - The main function to trigger the code autocompletion flow.
 * - CodeAutocompletionInput - The input type for the codeAutocompletion function.
 * - CodeAutocompletionOutput - The output type for the codeAutocompletion function.
 */

import {ai} from '@/ai/genkit';
import {z} from 'genkit';

const CodeAutocompletionInputSchema = z.object({
  codeContext: z
    .string()
    .describe('The current code context in the OmniLang editor.'),
  availableVariables: z
    .string()
    .describe('The available variables in the current scope.'),
  availableLibraries: z
    .string()
    .describe('The available libraries that can be used.'),
});
export type CodeAutocompletionInput = z.infer<typeof CodeAutocompletionInputSchema>;

const CodeAutocompletionOutputSchema = z.object({
  suggestions: z.array(z.string()).describe('A list of suggested code completions.'),
});
export type CodeAutocompletionOutput = z.infer<typeof CodeAutocompletionOutputSchema>;

export async function codeAutocompletion(input: CodeAutocompletionInput): Promise<CodeAutocompletionOutput> {
  return codeAutocompletionFlow(input);
}

const decideWhatToSuggestTool = ai.defineTool(
  {
    name: 'decideWhatToSuggest',
    description: 'Decides what variables and libraries might be suitable for the user based on the current code context.',
    inputSchema: z.object({
      codeContext: z.string().describe('The current code context in the OmniLang editor.'),
      availableVariables: z.string().describe('The available variables in the current scope.'),
      availableLibraries: z.string().describe('The available libraries that can be used.'),
    }),
    outputSchema: z.string(),
  },
  async input => {
    // Implementation of the tool to decide what to suggest based on the context.
    // This is a placeholder; replace with actual logic.
    return `Based on the code context: ${input.codeContext}, available variables: ${input.availableVariables}, and available libraries: ${input.availableLibraries}, suggest appropriate variables and libraries.`;
  }
);

const prompt = ai.definePrompt({
  name: 'codeAutocompletionPrompt',
  input: {schema: CodeAutocompletionInputSchema},
  output: {schema: CodeAutocompletionOutputSchema},
  tools: [decideWhatToSuggestTool],
  prompt: `
ROLE:
  Anda adalah AI assistant untuk code autocompletion yang ahli dalam bahasa pemrograman OmniLang.

INTENT:
  Memberikan saran pelengkapan kode yang relevan, cerdas, dan bermanfaat bagi programmer OmniLang.

AUDIENCE:
  Programmer yang sedang menulis kode di OmniLang Studio.

CONTEXT:
  Programmer sedang berada di tengah sesi coding. Mereka membutuhkan saran cepat dan akurat untuk melanjutkan pekerjaan mereka.

INPUT:
  - Code Context: {{{codeContext}}}
  - Available Variables: {{{availableVariables}}}
  - Available Libraries: {{{availableLibraries}}}

CONSTRAINT:
  - Saran harus berupa sintaks OmniLang yang valid.
  - Berikan saran yang paling mungkin dibutuhkan pengguna berdasarkan konteks.
  - Jangan memberikan saran yang terlalu panjang atau kompleks.

REASONING_MODE:
  Analisis konteks kode, variabel, dan library yang tersedia. Pertimbangkan langkah logis berikutnya yang mungkin diambil oleh programmer. Gunakan tool 'decideWhatToSuggest' untuk membantu penalaran.

OUTPUT_FORMAT:
  Kembalikan hanya array JSON dari string saran. Contoh: {"suggestions": ["suggestion1", "suggestion2"]}.

QUALITY_BAR:
  - Saran harus relevan secara kontekstual.
  - Saran harus membantu mempercepat proses coding.
  - Saran harus akurat secara sintaksis.

ETHICS:
  Tidak memberikan saran kode yang berbahaya atau tidak efisien.
`,
});

const codeAutocompletionFlow = ai.defineFlow(
  {
    name: 'codeAutocompletionFlow',
    inputSchema: CodeAutocompletionInputSchema,
    outputSchema: CodeAutocompletionOutputSchema,
  },
  async input => {
    const {output} = await prompt(input);
    return output!;
  }
);
