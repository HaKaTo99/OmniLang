"use client"

import { useState, useMemo, useEffect } from "react"
import { Copy, Share2, Check } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogFooter,
  DialogClose,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { useToast } from "@/hooks/use-toast"

interface ShareSnippetDialogProps {
  code: string;
}

export function ShareSnippetDialog({ code }: ShareSnippetDialogProps) {
  const [hasCopied, setHasCopied] = useState(false)
  const { toast } = useToast()
  const [shareableLink, setShareableLink] = useState('');

  useEffect(() => {
    if (typeof window !== 'undefined') {
        try {
            const encodedCode = btoa(code);
            setShareableLink(`${window.location.origin}${window.location.pathname}?code=${encodedCode}`);
        } catch (e) {
            console.error("Failed to encode code snippet:", e);
            setShareableLink("Could not generate link.");
        }
    }
  }, [code]);
  

  const handleCopy = async () => {
    if(!shareableLink || shareableLink === "Could not generate link.") return;

    await navigator.clipboard.writeText(shareableLink);
    setHasCopied(true);
    toast({
      description: "Link copied to clipboard!",
    })
    setTimeout(() => setHasCopied(false), 2000);
  }

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button>
          <Share2 className="mr-2 h-4 w-4" />
          Share
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Share Code Snippet</DialogTitle>
          <DialogDescription>
            Anyone with this link will be able to view this OmniLang code snippet.
          </DialogDescription>
        </DialogHeader>
        <div className="flex items-center space-x-2">
          <div className="grid flex-1 gap-2">
            <Label htmlFor="link" className="sr-only">
              Link
            </Label>
            <Input
              id="link"
              value={shareableLink}
              readOnly
              className="font-code"
            />
          </div>
          <Button type="button" size="icon" className="px-3" onClick={handleCopy} disabled={!shareableLink}>
            {hasCopied ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
            <span className="sr-only">Copy</span>
          </Button>
        </div>
        <DialogFooter className="sm:justify-start">
          <DialogClose asChild>
            <Button type="button" variant="secondary">
              Close
            </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
