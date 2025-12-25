"use client"

import { useState, useEffect, useMemo, useCallback } from "react"
import { Textarea } from "@/components/ui/textarea"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { codeAutocompletion } from "@/ai/flows/code-autocompletion"
import { parseOmniLang, type OmniLangError } from "@/lib/omniling-parser"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Lightbulb, TriangleAlert } from "lucide-react"
import { Skeleton } from "@/components/ui/skeleton"
import { useToast } from "@/hooks/use-toast"

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
}

export function CodeEditorPanel({ code, setCode }: CodeEditorPanelProps) {
  const [errors, setErrors] = useState<OmniLangError[]>([])
  const [suggestions, setSuggestions] = useState<string[]>([])
  const [isSuggesting, setIsSuggesting] = useState(false)
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

  return (
    <div className="flex flex-col h-full bg-card rounded-lg border shadow-sm overflow-hidden">
      <div className="flex-grow p-1 relative">
        <Textarea
          value={code}
          onChange={(e) => setCode(e.target.value)}
          placeholder="Define your Policy Intent here... (Start with INTENT:)"
          className="w-full h-full resize-none border-0 focus-visible:ring-0 focus-visible:ring-offset-0 font-code text-base bg-card absolute inset-0"
          aria-label="OmniLang Policy Editor"
        />
      </div>
      <div className="border-t shrink-0 h-[250px] flex flex-col">
        <Tabs defaultValue="suggestions" className="h-full flex flex-col">
          <TabsList className="px-2 pt-2 bg-card justify-start rounded-none border-b shrink-0">
            <TabsTrigger value="suggestions">AI Suggestions</TabsTrigger>
            <TabsTrigger value="errors">
              Errors {errors.length > 0 && <span className="ml-2 bg-destructive text-destructive-foreground h-5 w-5 text-xs rounded-full flex items-center justify-center">{errors.length}</span>}
            </TabsTrigger>
          </TabsList>
          <div className="p-4 overflow-y-auto flex-grow">
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
