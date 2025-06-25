"use client"

import Link from "next/link"
import { usePathname } from "next/navigation"
import { Music } from "lucide-react"

export function Navbar() {
  const pathname = usePathname()

  return (
    <header className="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container flex h-16 items-center">
        <Link href="/" className="flex items-center gap-2 mr-6">
          <div className="relative flex h-8 w-8 items-center justify-center rounded-full bg-gradient-to-r from-acoustic-purple to-acoustic-blue">
            <Music className="h-4 w-4 text-white" />
          </div>
          <span className="hidden font-bold sm:inline-block">Acoustic Scan</span>
        </Link>

        <nav className="flex flex-1 items-center gap-6 text-sm">
          <Link
            href="/scan"
            className={`transition-colors hover:text-foreground/80 ${
              pathname === "/scan" ? "text-foreground font-medium" : "text-foreground/60"
            }`}
          >
            Scan
          </Link>
          <Link
            href="/api-docs"
            className={`transition-colors hover:text-foreground/80 ${
              pathname === "/api-docs" ? "text-foreground font-medium" : "text-foreground/60"
            }`}
          >
            API Docs
          </Link>
        </nav>
      </div>
    </header>
  )
}

