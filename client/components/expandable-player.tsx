"use client"

import { useEffect, useRef, useState } from "react"
import { usePlayer } from "@/contexts/player-context"
import { Button } from "@/components/ui/button"
import { Slider } from "@/components/ui/slider"
import { Play, Pause, Volume2, VolumeX, Maximize2, SkipBack, SkipForward, X, BarChart3 } from "lucide-react"
import { cn } from "@/lib/utils"

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

interface ExpandablePlayerProps {
  onExpandChange?: (expanded: boolean) => void
}

export function ExpandablePlayer({ onExpandChange }: ExpandablePlayerProps) {
  const { currentSong, isPlaying, volume, togglePlay, setVolume } = usePlayer()
  const [currentTime, setCurrentTime] = useState(0)
  const [duration, setDuration] = useState(0)
  const [isMuted, setIsMuted] = useState(false)
  const [isExpanded, setIsExpanded] = useState(false)
  const [isTransitioning, setIsTransitioning] = useState(false)
  const [playerReady, setPlayerReady] = useState(false)

  const playerRef = useRef<any>(null)
  const playerContainerRef = useRef<HTMLDivElement>(null)
  const playerInstanceRef = useRef<any>(null)
  const fullscreenPlayerContainerRef = useRef<HTMLDivElement>(null)

  // Handle expand/collapse transitions
  const handleToggleExpand = () => {
    setIsTransitioning(true)
    const newExpandedState = !isExpanded
    setIsExpanded(newExpandedState)

    // Notify parent component about expansion state change
    if (onExpandChange) {
      onExpandChange(newExpandedState)
    }

    // Reset transition state after animation completes
    setTimeout(() => {
      setIsTransitioning(false)
    }, 300) // Match this with the CSS transition duration
  }

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

        // Determine which container to use based on expanded state
        const containerRef = isExpanded ? fullscreenPlayerContainerRef : playerContainerRef

        // Create a new container for the player
        if (containerRef.current) {
          // Remove any existing children
          while (containerRef.current.firstChild) {
            containerRef.current.removeChild(containerRef.current.firstChild)
          }

          // Create a new div for the player
          playerContainer = document.createElement("div")
          playerContainer.id = `youtube-player-${Date.now()}`
          containerRef.current.appendChild(playerContainer)

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
  }, [currentSong, isExpanded, isPlaying, volume, onExpandChange])

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

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!currentSong) return

      // ESC key to exit fullscreen
      if (e.key === "Escape" && isExpanded) {
        handleToggleExpand()
      }

      // Space bar to toggle play/pause
      if (e.key === " " && e.target === document.body) {
        e.preventDefault()
        togglePlay()
      }
    }

    document.addEventListener("keydown", handleKeyDown)

    return () => {
      document.removeEventListener("keydown", handleKeyDown)
    }
  }, [isExpanded, togglePlay, currentSong, onExpandChange])

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

  // Skip to previous or next song (mock implementation)
  const handleSkipPrevious = () => {
    // In a real implementation, you would get the previous song from a playlist
    console.log("Skip to previous song")
  }

  const handleSkipNext = () => {
    // In a real implementation, you would get the next song from a playlist
    console.log("Skip to next song")
  }

  if (!currentSong) return null

  // Render the expanded fullscreen player
  if (isExpanded) {
    return (
      <div className={cn("fixed inset-0 z-50 bg-background flex flex-col", isTransitioning ? "animate-fade-in" : "")}>
        <div className="container mx-auto flex-1 flex flex-col px-4">
          {/* Header */}
          <div className="flex items-center justify-between py-4 border-b">
            <h2 className="text-lg sm:text-xl font-bold">Now Playing</h2>
            <Button variant="ghost" size="icon" onClick={handleToggleExpand} className="rounded-full">
              <X className="h-5 w-5" />
              <span className="sr-only">Close</span>
            </Button>
          </div>

          {/* Main content */}
          <div className="flex-1 py-4 sm:py-8 grid grid-cols-1 lg:grid-cols-2 gap-6 sm:gap-8">
            {/* Left side - Thumbnail */}
            <div className="aspect-video bg-muted rounded-lg overflow-hidden shadow-lg relative">
              <div ref={fullscreenPlayerContainerRef} className="absolute inset-0" />
            </div>

            {/* Right side - Song details and controls */}
            <div className="flex flex-col space-y-6 sm:space-y-8">
              <div className="space-y-2">
                <h1 className="text-2xl sm:text-3xl font-bold gradient-text">{currentSong.name}</h1>
                <p className="text-lg sm:text-xl text-muted-foreground">{currentSong.artist}</p>

                <div className="flex items-center gap-2 mt-4">
                  <BarChart3 className="h-4 w-4 sm:h-5 sm:w-5 text-muted-foreground" />
                  <span className="text-xs sm:text-sm text-muted-foreground">
                    Scanned {currentSong.scanCount?.toLocaleString() || 0} times
                  </span>
                </div>
              </div>

              {/* Progress bar */}
              <div className="space-y-2">
                <Slider
                  value={[currentTime ? (currentTime / duration) * 100 : 0]}
                  onValueChange={handleSeek}
                  className="w-full [&>span:first-child]:h-1.5 sm:h-2 [&>span:first-child]:bg-muted [&_[role=slider]]:h-4 sm:h-5 [&_[role=slider]]:w-4 sm:w-5 [&_[role=slider]]:border-0 [&>span:first-child_span]:bg-primary"
                />

                <div className="flex justify-between text-xs sm:text-sm text-muted-foreground">
                  <span>{formatTime(currentTime)}</span>
                  <span>{formatTime(duration)}</span>
                </div>
              </div>

              {/* Controls */}
              <div className="flex items-center justify-center gap-4 sm:gap-6">
                <Button
                  size="icon"
                  variant="ghost"
                  className="h-10 w-10 sm:h-12 sm:w-12 rounded-full"
                  onClick={handleSkipPrevious}
                >
                  <SkipBack className="h-5 w-5 sm:h-6 sm:w-6" />
                </Button>

                <Button
                  size="icon"
                  variant="default"
                  className="h-14 w-14 sm:h-16 sm:w-16 rounded-full gradient-bg"
                  onClick={togglePlay}
                >
                  {isPlaying ? (
                    <Pause className="h-7 w-7 sm:h-8 sm:w-8" />
                  ) : (
                    <Play className="h-7 w-7 sm:h-8 sm:w-8 ml-1" />
                  )}
                </Button>

                <Button
                  size="icon"
                  variant="ghost"
                  className="h-10 w-10 sm:h-12 sm:w-12 rounded-full"
                  onClick={handleSkipNext}
                >
                  <SkipForward className="h-5 w-5 sm:h-6 sm:w-6" />
                </Button>
              </div>

              {/* Volume control */}
              <div className="flex items-center gap-4">
                <Button
                  size="icon"
                  variant="ghost"
                  className="h-8 w-8 sm:h-10 sm:w-10 rounded-full"
                  onClick={toggleMute}
                >
                  {isMuted ? (
                    <VolumeX className="h-4 w-4 sm:h-5 sm:w-5" />
                  ) : (
                    <Volume2 className="h-4 w-4 sm:h-5 sm:w-5" />
                  )}
                </Button>

                <Slider
                  value={[isMuted ? 0 : volume]}
                  min={0}
                  max={100}
                  step={1}
                  onValueChange={handleVolumeChange}
                  className="w-full max-w-xs [&>span:first-child]:h-1 sm:h-1.5 [&>span:first-child]:bg-muted [&_[role=slider]]:h-3 sm:h-4 [&_[role=slider]]:w-3 sm:w-4 [&_[role=slider]]:border-0 [&>span:first-child_span]:bg-primary"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    )
  }

  // Render the collapsed mini player
  return (
    <div
      className={cn(
        "fixed bottom-0 left-0 right-0 bg-background border-t border-border/40 h-16 sm:h-20 z-50",
        isTransitioning ? "animate-slide-up" : "",
      )}
    >
      <div className="container h-full mx-auto px-2 sm:px-4">
        <div className="flex h-full items-center gap-2 sm:gap-4">
          {/* Thumbnail and info */}
          <div className="flex items-center gap-2 sm:gap-3 flex-1 min-w-0">
            <div className="relative w-10 h-10 sm:w-12 sm:h-12 rounded-md overflow-hidden flex-shrink-0 bg-muted">
              <div ref={playerContainerRef} className="absolute inset-0" />
            </div>

            <div className="min-w-0 flex-1">
              <h4 className="font-medium text-sm truncate">{currentSong.name}</h4>
              <p className="text-xs text-muted-foreground truncate">{currentSong.artist}</p>
            </div>
          </div>

          {/* Controls */}
          <div className="flex items-center gap-2 sm:gap-4">
            {/* Time and progress - hidden on mobile */}
            <div className="hidden md:flex items-center gap-2 text-xs text-muted-foreground w-12 justify-end">
              {formatTime(currentTime)}
            </div>

            <div className="flex-1 hidden md:block w-32 lg:w-auto">
              <Slider
                value={[currentTime ? (currentTime / duration) * 100 : 0]}
                onValueChange={handleSeek}
                className="w-full [&>span:first-child]:h-1 [&>span:first-child]:bg-muted [&_[role=slider]]:h-3 [&_[role=slider]]:w-3 [&_[role=slider]]:border-0 [&>span:first-child_span]:bg-primary"
              />
            </div>

            <div className="hidden md:flex items-center gap-2 text-xs text-muted-foreground w-12">
              {formatTime(duration)}
            </div>

            <div className="flex items-center">
              <Button size="icon" variant="ghost" className="h-8 w-8 rounded-full" onClick={togglePlay}>
                {isPlaying ? <Pause className="h-4 w-4" /> : <Play className="h-4 w-4" />}
              </Button>

              <div className="hidden md:flex items-center gap-2">
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

              <Button size="icon" variant="ghost" className="h-8 w-8 rounded-full" onClick={handleToggleExpand}>
                <Maximize2 className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

