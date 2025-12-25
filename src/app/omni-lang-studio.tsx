"use client";

import { useState, useEffect } from "react";
import { useSearchParams } from "next/navigation";
import { Header } from "@/components/header";
import { Documentation } from "@/components/documentation";
import { CodeEditorPanel } from "@/components/code-editor-panel";

const initialCode = `// OmniLang Policy Document
// Define System Intent & Boundaries

INTENT: Melindungi keselamatan dan martabat pekerja
ACTOR:
- Primary: SafetySystem
- Secondary: Supervisor
CONTEXT:
- Domain: Factory Automation
- Lokasi: Assembly Line A
- Fase: Operation
RULE:
- IF WorkerDistance < 1m
- THEN StopMachineImmediate
CONSTRAINT:
- Legal: ISO-45001
- Ethical: HumanFirst
IMPACT:
- Benefit: Mencegah kecelakaan kerja
- Risk: Downtime produksi meningkat
TRACE:
- Moral: Value of Life
- Regulation: UU K3 No.1/1970
`;

export function OmniLangStudio() {
  const [code, setCode] = useState(initialCode);
  const searchParams = useSearchParams();

  useEffect(() => {
    const codeFromUrl = searchParams.get("code");
    if (codeFromUrl) {
      try {
        const decodedCode = atob(codeFromUrl);
        setCode(decodedCode);
      } catch (e) {
        console.error("Failed to decode code from URL:", e);
      }
    }
  }, [searchParams]);

  return (
    <div className="flex h-screen flex-col bg-background text-foreground">
      <Header code={code} />
      <main className="flex flex-1 overflow-hidden">
        <div className="flex-1 flex flex-col p-4 gap-4 min-w-0">
          <CodeEditorPanel code={code} setCode={setCode} />
        </div>
        <aside className="hidden md:block w-1/3 max-w-sm border-l bg-card p-4 overflow-y-auto">
          <Documentation />
        </aside>
      </main>
    </div>
  );
}
