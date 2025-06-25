"use client"

import { useState } from "react"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { IdentifySongTab } from "./identify-song-tab"
import { ContributeSongTab } from "./contribute-song-tab"
import { LibraryTab } from "./library-tab"
import { usePlayer } from "@/contexts/player-context"

export function ScanTabs() {
  const [activeTab, setActiveTab] = useState("identify")
  const { currentSong } = usePlayer()

  return (
    <div className={`w-full max-w-4xl mx-auto ${currentSong ? "mb-20 sm:mb-24" : ""}`}>
      <Tabs defaultValue="identify" className="w-full" onValueChange={setActiveTab}>
        <TabsList className="grid w-full grid-cols-3">
          <TabsTrigger value="identify">Identify Song</TabsTrigger>
          <TabsTrigger value="contribute">Contribute Song</TabsTrigger>
          <TabsTrigger value="library">Library</TabsTrigger>
        </TabsList>

        <TabsContent value="identify" className="animate-slide-up">
          <IdentifySongTab />
        </TabsContent>

        <TabsContent value="contribute" className="animate-slide-up">
          <ContributeSongTab />
        </TabsContent>

        <TabsContent value="library" className="animate-slide-up">
          <LibraryTab />
        </TabsContent>
      </Tabs>
    </div>
  )
}

