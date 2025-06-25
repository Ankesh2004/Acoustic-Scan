"use client"

import { useEffect, useState } from "react"
import { Navbar } from "@/components/navbar"
import { ScanTabs } from "@/components/scan-tabs"
import { ExpandablePlayer } from "@/components/expandable-player"
import { PlayerProvider } from "@/contexts/player-context"

export default function ScanPage() {
  const [isPlayerExpanded, setIsPlayerExpanded] = useState(false)

  // Listen for player expansion state
  useEffect(() => {
    const handlePlayerExpand = (e: CustomEvent) => {
      setIsPlayerExpanded(e.detail.expanded)
    }

    // Add event listener for player expansion
    window.addEventListener("playerExpand" as any, handlePlayerExpand)

    // Prevent body scrolling when player is expanded
    if (isPlayerExpanded) {
      document.body.classList.add("overflow-hidden")
    } else {
      document.body.classList.remove("overflow-hidden")
    }

    return () => {
      window.removeEventListener("playerExpand" as any, handlePlayerExpand)
      document.body.classList.remove("overflow-hidden")
    }
  }, [isPlayerExpanded])

  return (
    <PlayerProvider>
      <div className="flex min-h-screen flex-col">
        <Navbar />
        <main className="flex-1 container py-6 md:py-8 lg:py-12">
          <ScanTabs />
        </main>
        <ExpandablePlayer
          onExpandChange={(expanded) => {
            // Dispatch custom event for player expansion
            window.dispatchEvent(new CustomEvent("playerExpand", { detail: { expanded } }))
          }}
        />
      </div>
    </PlayerProvider>
  )
}

