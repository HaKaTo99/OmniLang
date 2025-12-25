"use client"
import { Book, Code, Share2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet"
import { ShareSnippetDialog } from "./share-snippet-dialog"
import { Documentation } from "./documentation"

interface HeaderProps {
  code: string;
}

export function Header({ code }: HeaderProps) {
  return (
    <header className="flex h-16 shrink-0 items-center justify-between border-b bg-[#1e1e1e] text-white px-4 md:px-6 z-10">
      <div className="flex items-center gap-3">
        <Code className="h-7 w-7 text-primary" />
        <h1 className="text-xl font-bold font-headline">OmniLang Studio</h1>
      </div>
      <div className="flex items-center gap-2">
        <div className="hidden md:block">
            <ShareSnippetDialog code={code} />
        </div>
        <div className="md:hidden">
          <Sheet>
            <SheetTrigger asChild>
              <Button variant="outline" size="icon">
                <Book className="h-5 w-5" />
                <span className="sr-only">Menu</span>
              </Button>
            </SheetTrigger>
            <SheetContent side="right" className="w-full max-w-sm p-0 flex flex-col">
                <div className="p-4 border-b">
                    <ShareSnippetDialog code={code} />
                </div>
                <div className="p-4 overflow-y-auto flex-grow">
                    <Documentation />
                </div>
            </SheetContent>
          </Sheet>
        </div>
      </div>
    </header>
  )
}
