"use client"

import { useEffect, useRef, useState } from "react"
import { usePlayer } from "@/contexts/player-context"
import { Button } from "@/components/ui/button"
import { Slider } from "@/components/ui/slider"
import { Play, Pause, Volume2, VolumeX, Maximize, Minimize } from "lucide-react"

declare global {
  interface Window {
    YT: any
    onYouTubeIframeAPIReady: () => void
  }
}

// Create a singleton for YouTube API loading to prevent multiple loads
let youtubeApiLoaded = false
let youtubeApiLoading = false
let youtubeApiCallbacks: (() => void)[] = []

function loadYouTubeApi(): Promise<void> {
  return new Promise((resolve) => {
    // If already loaded, resolve immediately
    if (youtubeApiLoaded && window.YT) {
      resolve()
      return
    }

    // If currently loading, add to callbacks
    if (youtubeApiLoading) {
      youtubeApiCallbacks.push(() => resolve())
      return
    }

    // Start loading
    youtubeApiLoading = true

    // Set up global callback
    window.onYouTubeIframeAPIReady = () => {
      youtubeApiLoaded = true
      youtubeApiLoading = false
      resolve()

      // Call any queued callbacks
      youtubeApiCallbacks.forEach((callback) => callback())
      youtubeApiCallbacks = []
    }

    // Create script tag if it doesn't exist
    if (!document.getElementById("youtube-api-script")) {
      const tag = document.createElement("script")
      tag.id = "youtube-api-script"
      tag.src = "https://www.youtube.com/iframe_api"

      // Append to document
      document.head.appendChild(tag)
    }
  })
}

export function MusicPlayer() {
  const { currentSong, isPlaying, volume, togglePlay, setVolume } = usePlayer()
  const [currentTime, setCurrentTime] = useState(0)
  const [duration, setDuration] = useState(0)
  const [isMuted, setIsMuted] = useState(false)
  const [isMinimized, setIsMinimized] = useState(false)
  const [playerReady, setPlayerReady] = useState(false)

  const playerRef = useRef<any>(null)
  const playerContainerRef = useRef<HTMLDivElement>(null)
  const playerInstanceRef = useRef<any>(null)

  // Initialize YouTube player when song changes
  useEffect(() => {
    if (!currentSong) return

    let isMounted = true
    let playerContainer: HTMLDivElement | null = null

    const initializePlayer = async () => {
      try {
        // Load YouTube API if needed
        await loadYouTubeApi()

        // If component unmounted during API load, abort
        if (!isMounted) return

        // Clean up any existing player
        if (playerInstanceRef.current) {
          try {
            playerInstanceRef.current.destroy()
          } catch (error) {
            console.error("Error destroying previous player:", error)
          }
          playerInstanceRef.current = null
        }

        // Create a new container for the player
        if (playerContainerRef.current) {
          // Remove any existing children
          while (playerContainerRef.current.firstChild) {
            playerContainerRef.current.removeChild(playerContainerRef.current.firstChild)
          }

          // Create a new div for the player
          playerContainer = document.createElement("div")
          playerContainer.id = `youtube-player-${Date.now()}`
          playerContainerRef.current.appendChild(playerContainer)

          // Create new player
          playerInstanceRef.current = new window.YT.Player(playerContainer.id, {
            height: "100%",
            width: "100%",
            videoId: currentSong.youtubeId,
            playerVars: {
              autoplay: isPlaying ? 1 : 0,
              controls: 0,
              modestbranding: 1,
              rel: 0,
              showinfo: 0,
            },
            events: {
              onReady: (event: any) => {
                if (!isMounted) return

                console.log("YouTube player ready")
                playerRef.current = event.target
                setPlayerReady(true)

                try {
                  event.target.setVolume(volume)
                  setDuration(event.target.getDuration())

                  if (isPlaying) {
                    event.target.playVideo()
                  }
                } catch (error) {
                  console.error("Error during player initialization:", error)
                }
              },
              onStateChange: (event: any) => {
                if (!isMounted) return

                if (event.data === window.YT.PlayerState.ENDED) {
                  togglePlay()
                }
              },
              onError: (event: any) => {
                console.error("YouTube player error:", event.data)
              },
            },
          })
        }
      } catch (error) {
        console.error("Error initializing YouTube player:", error)
      }
    }

    initializePlayer()

    // Cleanup function
    return () => {
      isMounted = false

      // Clean up player on unmount
      if (playerInstanceRef.current) {
        try {
          playerInstanceRef.current.destroy()
        } catch (error) {
          console.error("Error destroying player on unmount:", error)
        }
        playerInstanceRef.current = null
      }

      setPlayerReady(false)
      playerRef.current = null
    }
  }, [currentSong?.youtubeId, isPlaying, volume, togglePlay])

  // Handle play/pause
  useEffect(() => {
    if (!playerRef.current || !playerReady) return

    try {
      if (isPlaying) {
        playerRef.current.playVideo()
      } else {
        playerRef.current.pauseVideo()
      }
    } catch (error) {
      console.error("Error controlling video playback:", error)
    }
  }, [isPlaying, playerReady])

  // Handle volume change
  useEffect(() => {
    if (!playerRef.current || !playerReady) return

    try {
      playerRef.current.setVolume(isMuted ? 0 : volume)
    } catch (error) {
      console.error("Error setting volume:", error)
    }
  }, [volume, isMuted, playerReady])

  // Update current time
  useEffect(() => {
    if (!playerRef.current || !playerReady || !isPlaying) return

    const interval = setInterval(() => {
      try {
        if (playerRef.current) {
          setCurrentTime(playerRef.current.getCurrentTime())
        }
      } catch (error) {
        console.error("Error getting current time:", error)
      }
    }, 1000)

    return () => clearInterval(interval)
  }, [isPlaying, playerReady])

  const handleSeek = (value: number[]) => {
    if (!playerRef.current || !playerReady) return

    try {
      const seekTime = (value[0] / 100) * duration
      playerRef.current.seekTo(seekTime)
      setCurrentTime(seekTime)
    } catch (error) {
      console.error("Error seeking:", error)
    }
  }

  const handleVolumeChange = (value: number[]) => {
    setVolume(value[0])
    setIsMuted(value[0] === 0)
  }

  const toggleMute = () => {
    setIsMuted(!isMuted)
  }

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60)
    const secs = Math.floor(seconds % 60)
    return `${mins}:${secs < 10 ? "0" : ""}${secs}`
  }

  if (!currentSong) return null

  return (
    <div
      className={`fixed bottom-0 left-0 right-0 bg-background border-t border-border/40 transition-all duration-300 z-50 ${
        isMinimized ? "h-16" : "h-24 sm:h-20"
      }`}
    >
      <div className="container h-full mx-auto px-4">
        <div className="flex h-full items-center gap-4">
          {/* Thumbnail and info */}
          <div className="flex items-center gap-3 flex-1 min-w-0">
            <div
              className={`relative aspect-video bg-muted rounded-md overflow-hidden ${
                isMinimized ? "w-12 h-12" : "w-16 h-16 sm:w-12 sm:h-12"
              }`}
            >
              {!isMinimized && <div ref={playerContainerRef} className="absolute inset-0" />}
              {isMinimized && (
                <img
                  src={currentSong.thumbnailUrl || "/placeholder.svg"}
                  alt={currentSong.name}
                  className="w-full h-full object-cover"
                />
              )}
            </div>

            <div className="min-w-0 flex-1">
              <h4 className="font-medium text-sm truncate">{currentSong.name}</h4>
              <p className="text-xs text-muted-foreground truncate">{currentSong.artist}</p>
            </div>
          </div>

          {/* Controls */}
          <div className={`flex items-center gap-4 ${isMinimized ? "flex-1 justify-end" : "flex-[2]"}`}>
            {!isMinimized && (
              <>
                <div className="hidden sm:flex items-center gap-2 text-xs text-muted-foreground w-16 justify-end">
                  {formatTime(currentTime)}
                </div>

                <div className="flex-1 hidden sm:block">
                  <Slider
                    value={[currentTime ? (currentTime / duration) * 100 : 0]}
                    onValueChange={handleSeek}
                    className="w-full [&>span:first-child]:h-1 [&>span:first-child]:bg-muted [&_[role=slider]]:h-3 [&_[role=slider]]:w-3 [&_[role=slider]]:border-0 [&>span:first-child_span]:bg-primary"
                  />
                </div>

                <div className="hidden sm:flex items-center gap-2 text-xs text-muted-foreground w-16">
                  {formatTime(duration)}
                </div>
              </>
            )}

            <div className="flex items-center gap-2">
              <Button size="icon" variant="ghost" className="h-8 w-8 rounded-full" onClick={togglePlay}>
                {isPlaying ? <Pause className="h-4 w-4" /> : <Play className="h-4 w-4" />}
              </Button>

              <div className="hidden sm:flex items-center gap-2">
                <Button size="icon" variant="ghost" className="h-8 w-8 rounded-full" onClick={toggleMute}>
                  {isMuted ? <VolumeX className="h-4 w-4" /> : <Volume2 className="h-4 w-4" />}
                </Button>

                <Slider
                  value={[isMuted ? 0 : volume]}
                  min={0}
                  max={100}
                  step={1}
                  onValueChange={handleVolumeChange}
                  className="w-20 [&>span:first-child]:h-1 [&>span:first-child]:bg-muted [&_[role=slider]]:h-3 [&_[role=slider]]:w-3 [&_[role=slider]]:border-0 [&>span:first-child_span]:bg-primary"
                />
              </div>

              <Button
                size="icon"
                variant="ghost"
                className="h-8 w-8 rounded-full"
                onClick={() => setIsMinimized(!isMinimized)}
              >
                {isMinimized ? <Maximize className="h-4 w-4" /> : <Minimize className="h-4 w-4" />}
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

