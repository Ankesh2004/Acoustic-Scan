"use client"

import type React from "react"

import { useState } from "react"
import { Check, Copy } from "lucide-react"
import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"

interface CopyButtonProps extends React.HTMLAttributes<HTMLButtonElement> {
  value: string
  src?: string
}

export function CopyButton({ value, className, src, ...props }: CopyButtonProps) {
  const [hasCopied, setHasCopied] = useState(false)

  const copyToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(value)
      setHasCopied(true)
      setTimeout(() => setHasCopied(false), 2000)
    } catch (error) {
      console.error("Failed to copy text: ", error)
    }
  }

  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            size="icon"
            variant="ghost"
            className={cn("relative h-8 w-8 text-muted-foreground hover:bg-muted hover:text-foreground", className)}
            onClick={copyToClipboard}
            {...props}
          >
            <span className="sr-only">{hasCopied ? "Copied" : "Copy"}</span>
            {hasCopied ? <Check className="h-4 w-4 text-green-500" /> : <Copy className="h-4 w-4" />}
          </Button>
        </TooltipTrigger>
        <TooltipContent>{hasCopied ? "Copied!" : "Copy to clipboard"}</TooltipContent>
      </Tooltip>
    </TooltipProvider>
  )
}

