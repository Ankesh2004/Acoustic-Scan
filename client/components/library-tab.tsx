"use client"

import { useState, useEffect } from "react"
import { ApiClient, LibrarySong } from "@/lib/api"
import { Card, CardContent } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Play, Search, RefreshCw, ExternalLink } from "lucide-react"

export function LibraryTab() {
  const [searchQuery, setSearchQuery] = useState("")
  const [library, setLibrary] = useState<LibrarySong[]>([])
  const [filteredSongs, setFilteredSongs] = useState<LibrarySong[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Fetch library from backend
  const fetchLibrary = async () => {
    setIsLoading(true)
    setError(null)
    
    try {
      const songs = await ApiClient.getLibrary()
      setLibrary(songs)
      setFilteredSongs(songs)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load library')
    } finally {
      setIsLoading(false)
    }
  }

  // Load library on component mount
  useEffect(() => {
    fetchLibrary()
  }, [])

  // Filter songs based on search query
  useEffect(() => {
    if (searchQuery.trim() === "") {
      setFilteredSongs(library)
      return
    }

    const query = searchQuery.toLowerCase()
    const filtered = library.filter(
      (song) => 
        song.name.toLowerCase().includes(query) || 
        song.artist.toLowerCase().includes(query)
    )

    setFilteredSongs(filtered)
  }, [searchQuery, library])

  // Handle opening YouTube link
  const handlePlaySong = (song: LibrarySong) => {
    if (song.youtube_id) {
      window.open(`https://youtube.com/watch?v=${song.youtube_id}`, '_blank', 'noopener,noreferrer')
    }
  }

  return (
    <Card className="border-border/50">
      <CardContent className="p-6">
        <div className="space-y-6">
          <div className="text-center space-y-2">
            <div className="flex items-center justify-center gap-2">
              <h2 className="text-2xl font-bold tracking-tight">Song Library</h2>
              <Button
                size="sm"
                variant="ghost"
                onClick={fetchLibrary}
                disabled={isLoading}
              >
                <RefreshCw className={`h-4 w-4 ${isLoading ? 'animate-spin' : ''}`} />
              </Button>
            </div>
            <p className="text-muted-foreground">
              {library.length > 0 
                ? `${library.length} songs in your collection`
                : "Browse songs from your collection"
              }
            </p>
          </div>

          <div className="relative">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              placeholder="Search by song or artist name..."
              className="pl-10"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              disabled={isLoading}
            />
          </div>

          {error && (
            <div className="bg-red-50 border border-red-200 rounded-md p-4">
              <p className="text-red-800">{error}</p>
              <Button
                size="sm"
                variant="outline"
                onClick={fetchLibrary}
                className="mt-2"
              >
                Retry
              </Button>
            </div>
          )}

          <div className="space-y-1">
            {isLoading ? (
              <div className="py-12 text-center">
                <RefreshCw className="h-8 w-8 animate-spin mx-auto mb-2 text-muted-foreground" />
                <p className="text-muted-foreground">Loading your library...</p>
              </div>
            ) : filteredSongs.length === 0 ? (
              <div className="py-12 text-center text-muted-foreground">
                {library.length === 0 
                  ? "No songs in your library yet. Add some songs using the 'Contribute' tab!"
                  : "No songs found matching your search."
                }
              </div>
            ) : (
              filteredSongs.map((song) => (
                <div
                  key={song.id}
                  className="flex items-center gap-3 p-2 rounded-md transition-colors hover:bg-accent"
                >
                  <div className="relative w-12 h-12 rounded-md overflow-hidden flex-shrink-0">
                    <img
                      src={song.thumbnail_url || "/placeholder.svg"}
                      alt={song.name}
                      className="w-full h-full object-cover"
                      onError={(e) => {
                        e.currentTarget.src = "/placeholder.svg"
                      }}
                    />
                  </div>

                  <div className="min-w-0 flex-1">
                    <h4 className="font-medium truncate">{song.name}</h4>
                    <p className="text-sm text-muted-foreground truncate">{song.artist}</p>
                  </div>

                  <Button
                    size="sm"
                    variant="ghost"
                    className="text-blue-600 hover:text-blue-800 flex items-center gap-1"
                    onClick={() => handlePlaySong(song)}
                    disabled={!song.youtube_id}
                  >
                    <Play className="h-4 w-4" />
                    Play
                  </Button>
                </div>
              ))
            )}
          </div>
        </div>
      </CardContent>
    </Card>
  )
}

