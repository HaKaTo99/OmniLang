"use client"

import { useState, useEffect, useMemo, useCallback, useRef } from "react"
import { Textarea } from "@/components/ui/textarea"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { codeAutocompletion } from "@/ai/flows/code-autocompletion"
import { parseOmniLang, type OmniLangError } from "@/lib/omniling-parser"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Lightbulb, TriangleAlert } from "lucide-react"
import { Skeleton } from "@/components/ui/skeleton"
import { useToast } from "@/hooks/use-toast"
import { Button } from "@/components/ui/button"
import { CheckCircle2 } from "lucide-react"

// Debounce helper
function debounce<F extends (...args: any[]) => any>(func: F, waitFor: number) {
  let timeout: ReturnType<typeof setTimeout> | null = null;

  return (...args: Parameters<F>): Promise<ReturnType<F>> =>
    new Promise(resolve => {
      if (timeout) {
        clearTimeout(timeout);
      }

      timeout = setTimeout(() => resolve(func(...args)), waitFor);
    });
}

interface CodeEditorPanelProps {
  code: string;
  setCode: (code: string) => void;
  validationResult?: { errors: any[]; rules: any[]; actions: string[]; stdout: string; stderr?: string; engine?: string };
  isValidating?: boolean;
  onValidate?: () => void;
}

export function CodeEditorPanel({ code, setCode, validationResult, isValidating, onValidate }: CodeEditorPanelProps) {
  const [errors, setErrors] = useState<OmniLangError[]>([])
  const [suggestions, setSuggestions] = useState<string[]>([])
  const [isSuggesting, setIsSuggesting] = useState(false)
  const [panelHeight, setPanelHeight] = useState(260)
  const [isDragging, setIsDragging] = useState(false)
  const startY = useRef(0)
  const startHeight = useRef(260)
  const MIN_PANEL = 160
  const MAX_PANEL = 420
  const containerRef = useRef<HTMLDivElement | null>(null)
  const { toast } = useToast()

  const availableActors = useMemo(() => {
    const actors: string[] = [];
    const lines = code.split('\n');
    lines.forEach(line => {
      const match = line.match(/- (Primary|Secondary):\s*(.+)/);
      if (match) {
        actors.push(match[2].trim());
      }
    });
    return actors.join(', ');
  }, [code]);

  const availableContexts = "Standard:Safety, Standard:Ethics, Global:UniversalDeclaration";

  const getSuggestions = useCallback(async (currentCode: string) => {
    if (!currentCode.trim()) {
      setSuggestions([]);
      return;
    }
    setIsSuggesting(true);
    try {
      const result = await codeAutocompletion({
        codeContext: currentCode,
        availableVariables: availableActors,
        availableLibraries: availableContexts,
      });
      setSuggestions(result.suggestions || []);
    } catch (error) {
      console.error("AI suggestion failed:", error);
      toast({
        variant: "destructive",
        title: "AI Error",
        description: "Could not fetch AI suggestions. Please try again later.",
      })
      setSuggestions([]);
    } finally {
      setIsSuggesting(false);
    }
  }, [availableActors, toast]);

  const debouncedGetSuggestions = useMemo(() => debounce(getSuggestions, 5000), [getSuggestions]);

  useEffect(() => {
    const newErrors = parseOmniLang(code);
    setErrors(newErrors);
    // AI suggestions are currently disabled to prevent rate-limiting.
    // If you want to re-enable, you can call debouncedGetSuggestions(code) here.
    // Consider adding a manual trigger (e.g., a button) instead of automatic calls.
  }, [code]);

  useEffect(() => {
    if (!isDragging) return;

    const handleMove = (e: MouseEvent) => {
      const delta = startY.current - e.clientY;
      const next = Math.min(MAX_PANEL, Math.max(MIN_PANEL, startHeight.current + delta));
      setPanelHeight(next);
    };

    const handleUp = () => setIsDragging(false);

    window.addEventListener("mousemove", handleMove);
    window.addEventListener("mouseup", handleUp);

    return () => {
      window.removeEventListener("mousemove", handleMove);
      window.removeEventListener("mouseup", handleUp);
    };
  }, [isDragging]);

  return (
    <div ref={containerRef} className="rounded-xl overflow-hidden flex flex-col min-h-[720px] bg-white border border-sky-500">
      <div className="p-4 pb-3 flex-1 min-h-[0] bg-white border-b border-sky-500/80">
        <Textarea
          value={code}
          onChange={(e) => setCode(e.target.value)}
          placeholder="Define your Policy Intent here... (Start with INTENT:)"
          className="w-full h-full min-h-[360px] resize-none border border-sky-500/70 bg-card focus-visible:ring-0 focus-visible:ring-offset-0 font-code text-base"
          aria-label="OmniLang Policy Editor"
        />
      </div>

      <div
        className={`relative h-3 cursor-row-resize bg-sky-500/90 transition-all
          ${isDragging ? "bg-sky-500 shadow-[0_0_0_2px_rgba(59,91,253,0.28)]" : "hover:bg-sky-500"}`}
        onMouseDown={(e) => {
          e.preventDefault();
          startY.current = e.clientY;
          startHeight.current = panelHeight;
          setIsDragging(true);
        }}
        aria-label="Resize validation panel"
      >
        <div className="absolute inset-x-6 top-1/2 -translate-y-1/2 h-[2px] bg-sky-100/70 pointer-events-none rounded-full" />
      </div>

      <div className="border-t border-slate-200 bg-white/95 backdrop-blur" style={{ height: `${panelHeight}px`, minHeight: `${MIN_PANEL}px`, maxHeight: `${MAX_PANEL}px` }}>
        <div className="flex items-center justify-between px-4 py-3 border-b border-slate-200 bg-slate-50/60">
          <div>
            <h3 className="text-sm font-semibold text-slate-800">Validation & Insights</h3>
            <p className="text-xs text-slate-500">Hasil validasi, rekomendasi AI, dan error parsing.</p>
          </div>
        </div>
        <Tabs defaultValue="validation" className="h-full flex flex-col">
          <TabsList className="px-4 pt-2 bg-white justify-start rounded-none border-b border-slate-200 shrink-0 sticky top-0 z-10">
            <TabsTrigger value="validation">Validation</TabsTrigger>
            <TabsTrigger value="suggestions">AI Suggestions</TabsTrigger>
            <TabsTrigger value="errors">
              Errors {errors.length > 0 && <span className="ml-2 bg-destructive text-destructive-foreground h-5 w-5 text-xs rounded-full flex items-center justify-center">{errors.length}</span>}
            </TabsTrigger>
          </TabsList>
          <div className="p-4 overflow-y-auto flex-grow">
            <TabsContent value="validation" className="mt-0 space-y-2">
              <div className="flex items-center gap-2">
                <Button variant="secondary" size="sm" onClick={onValidate} disabled={isValidating}>
                  <CheckCircle2 className="h-4 w-4 mr-2" />
                  {isValidating ? "Validating..." : "Run Validate"}
                </Button>
              </div>
              {validationResult ? (
                <div className="space-y-3">
                  <div>
                    <h4 className="font-semibold">Actions</h4>
                    {validationResult.actions?.length ? (
                      <ul className="list-disc pl-4 space-y-1 font-code text-sm">
                        {validationResult.actions.map((a, i) => (<li key={i}>{a}</li>))}
                      </ul>
                    ) : <p className="text-sm text-muted-foreground">No actions triggered.</p>}
                  </div>
                  <div>
                    <h4 className="font-semibold">Rules Parsed</h4>
                    {validationResult.rules?.length ? (
                      <ul className="list-disc pl-4 space-y-1 font-code text-sm">
                        {validationResult.rules.map((r: any, i: number) => (<li key={i}>IF {r.condition} THEN {r.action}</li>))}
                      </ul>
                    ) : <p className="text-sm text-muted-foreground">No rules detected.</p>}
                  </div>
                  <div>
                    <h4 className="font-semibold">Stdout {validationResult.engine ? `(engine: ${validationResult.engine})` : ""}</h4>
                    <pre className="text-xs bg-secondary rounded p-2 whitespace-pre-wrap">{validationResult.stdout || "(empty)"}</pre>
                    {validationResult.stderr ? (
                      <details className="text-xs text-destructive">
                        <summary>stderr</summary>
                        <pre className="whitespace-pre-wrap">{validationResult.stderr}</pre>
                      </details>
                    ) : null}
                  </div>
                </div>
              ) : (
                <p className="text-sm text-muted-foreground">Run validation to see results.</p>
              )}
            </TabsContent>
            <TabsContent value="suggestions" className="mt-0">
              {isSuggesting ? (
                <div className="space-y-2">
                  <Skeleton className="h-8 w-3/4" />
                  <Skeleton className="h-8 w-1/2" />
                  <Skeleton className="h-8 w-2/3" />
                </div>
              ) : suggestions.length > 0 ? (
                <ul className="space-y-2">
                  {suggestions.map((s, i) => (
                    <li key={i} className="animate-in fade-in-0 duration-300"><Alert><Lightbulb className="h-4 w-4" /><AlertDescription className="font-code">{s}</AlertDescription></Alert></li>
                  ))}
                </ul>
              ) : (
                <p className="text-sm text-muted-foreground text-center pt-8">No suggestions available. Keep typing!</p>
              )}
            </TabsContent>
            <TabsContent value="errors" className="mt-0">
              {errors.length > 0 ? (
                <ul className="space-y-2">
                  {errors.map((e, i) => (
                    <li key={i} className="animate-in fade-in-0 duration-300">
                      <Alert variant="destructive">
                        <TriangleAlert className="h-4 w-4" />
                        <AlertTitle>
                          {e.line > 0 ? `Error on line ${e.line}` : 'General Error'}
                        </AlertTitle>
                        <AlertDescription>{e.message}</AlertDescription>
                      </Alert>
                    </li>
                  ))}
                </ul>
              ) : (
                <p className="text-sm text-muted-foreground text-center pt-8">No errors detected. Great job!</p>
              )}
            </TabsContent>
          </div>
        </Tabs>
      </div>
    </div>
  )
}
