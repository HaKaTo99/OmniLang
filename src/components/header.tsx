"use client"
import { Book, Code, Share2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet"
import { ShareSnippetDialog } from "./share-snippet-dialog"
import { Documentation } from "./documentation"

interface HeaderProps {
  code: string;
  onValidate?: () => void;
  isValidating?: boolean;
}

export function Header({ code, onValidate, isValidating }: HeaderProps) {
  return (
    <header className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 sticky top-0 z-50">
      <div className="container flex h-14 items-center justify-between px-4">
        <div className="flex items-center gap-2">
          <div className="bg-primary p-1.5 rounded-lg">
            <Code className="h-5 w-5 text-primary-foreground" />
          </div>
          <div>
            <h1 className="text-lg font-bold tracking-tight">OmniLang Studio</h1>
            <p className="text-[10px] text-muted-foreground uppercase tracking-widest font-medium">Universal Intent Language v1.0</p>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={onValidate}
            disabled={isValidating}
            className="hidden sm:flex"
          >
            {isValidating ? "Validating..." : "Validate Policy"}
          </Button>

          <ShareSnippetDialog code={code} />

          <Sheet>
            <SheetTrigger asChild>
              <Button variant="ghost" size="icon">
                <Book className="h-5 w-5" />
                <span className="sr-only">Documentation</span>
              </Button>
            </SheetTrigger>
            <SheetContent side="right" className="p-0 w-[400px] sm:w-[540px]">
              <div className="p-6 h-full flex flex-col">
                <div className="flex items-center justify-between mb-4">
                  <h2 className="text-lg font-semibold">Documentation</h2>
                </div>
                <div className="flex-grow overflow-y-auto">
                  <Documentation />
                </div>
              </div>
            </SheetContent>
          </Sheet>
        </div>
      </div>
    </header>
  )
}
