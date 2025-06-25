"use client"

import { createContext, useContext, useState, useRef, useEffect, ReactNode } from "react"

export interface Song {
  id: string;
  name: string;
  artist: string;
  youtubeId: string;
  thumbnailUrl?: string;
}

interface PlayerContextType {
  currentSong: Song | null;
  isPlaying: boolean;
  isPaused: boolean;
  currentTime: number;
  duration: number;
  volume: number;
  playSong: (song: Song) => void;
  pauseSong: () => void;
  resumeSong: () => void;
  togglePlayPause: () => void;
  seekTo: (time: number) => void;
  setVolume: (volume: number) => void;
  nextSong: () => void;
  previousSong: () => void;
}

const PlayerContext = createContext<PlayerContextType | undefined>(undefined)

interface PlayerProviderProps {
  children: ReactNode;
}

export function PlayerProvider({ children }: PlayerProviderProps) {
  const [currentSong, setCurrentSong] = useState<Song | null>(null)
  const [isPlaying, setIsPlaying] = useState(false)
  const [isPaused, setIsPaused] = useState(false)
  const [currentTime, setCurrentTime] = useState(0)
  const [duration, setDuration] = useState(0)
  const [volume, setVolumeState] = useState(0.7)
  
  // YouTube player reference
  const playerRef = useRef<any>(null)
  const intervalRef = useRef<NodeJS.Timeout | null>(null)

  // Initialize YouTube API
  useEffect(() => {
    const initYouTubeAPI = () => {
      if (typeof window !== 'undefined' && !window.YT) {
        const script = document.createElement('script')
        script.src = 'https://www.youtube.com/iframe_api'
        document.body.appendChild(script)

        window.onYouTubeIframeAPIReady = () => {
          console.log('YouTube API ready')
        }
      }
    }

    initYouTubeAPI()
  }, [])

  // Update current time periodically when playing
  useEffect(() => {
    if (isPlaying && !isPaused && playerRef.current) {
      intervalRef.current = setInterval(() => {
        if (playerRef.current && typeof playerRef.current.getCurrentTime === 'function') {
          const time = playerRef.current.getCurrentTime()
          setCurrentTime(time)
        }
      }, 1000)
    } else {
      if (intervalRef.current) {
        clearInterval(intervalRef.current)
        intervalRef.current = null
      }
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current)
      }
    }
  }, [isPlaying, isPaused])

  const createPlayer = (videoId: string) => {
    if (typeof window !== 'undefined' && window.YT && window.YT.Player) {
      // Destroy existing player if it exists
      if (playerRef.current) {
        playerRef.current.destroy()
      }

      playerRef.current = new window.YT.Player('youtube-player', {
        height: '0',
        width: '0',
        videoId: videoId,
        playerVars: {
          autoplay: 1,
          controls: 0,
        },
        events: {
          onReady: (event: any) => {
            event.target.setVolume(volume * 100)
            setDuration(event.target.getDuration())
            setIsPlaying(true)
            setIsPaused(false)
          },
          onStateChange: (event: any) => {
            const state = event.data
            if (state === window.YT.PlayerState.PLAYING) {
              setIsPlaying(true)
              setIsPaused(false)
            } else if (state === window.YT.PlayerState.PAUSED) {
              setIsPlaying(false)
              setIsPaused(true)
            } else if (state === window.YT.PlayerState.ENDED) {
              setIsPlaying(false)
              setIsPaused(false)
              setCurrentTime(0)
            }
          },
        },
      })
    }
  }

  const playSong = (song: Song) => {
    // If it's the same song and it's paused, just resume
    if (currentSong?.id === song.id && isPaused) {
      resumeSong()
      return
    }

    // If it's the same song and it's playing, pause it
    if (currentSong?.id === song.id && isPlaying) {
      pauseSong()
      return
    }

    // Play new song
    setCurrentSong(song)
    setCurrentTime(0)
    
    if (typeof window !== 'undefined' && window.YT) {
      createPlayer(song.youtubeId)
    } else {
      // Wait for YouTube API to load
      const checkYT = setInterval(() => {
        if (typeof window !== 'undefined' && window.YT) {
          clearInterval(checkYT)
          createPlayer(song.youtubeId)
        }
      }, 100)
    }
  }

  const pauseSong = () => {
    if (playerRef.current && typeof playerRef.current.pauseVideo === 'function') {
      playerRef.current.pauseVideo()
    }
  }

  const resumeSong = () => {
    if (playerRef.current && typeof playerRef.current.playVideo === 'function') {
      playerRef.current.playVideo()
    }
  }

  const togglePlayPause = () => {
    if (!currentSong) return
    
    if (isPlaying) {
      pauseSong()
    } else {
      resumeSong()
    }
  }

  const seekTo = (time: number) => {
    if (playerRef.current && typeof playerRef.current.seekTo === 'function') {
      playerRef.current.seekTo(time, true)
      setCurrentTime(time)
    }
  }

  const setVolume = (newVolume: number) => {
    setVolumeState(newVolume)
    if (playerRef.current && typeof playerRef.current.setVolume === 'function') {
      playerRef.current.setVolume(newVolume * 100)
    }
  }

  const nextSong = () => {
    // Implement next song logic if you have a playlist
    console.log('Next song functionality not implemented yet')
  }

  const previousSong = () => {
    // Implement previous song logic if you have a playlist
    console.log('Previous song functionality not implemented yet')
  }

  const value: PlayerContextType = {
    currentSong,
    isPlaying,
    isPaused,
    currentTime,
    duration,
    volume,
    playSong,
    pauseSong,
    resumeSong,
    togglePlayPause,
    seekTo,
    setVolume,
    nextSong,
    previousSong,
  }

  return (
    <PlayerContext.Provider value={value}>
      {children}
      {/* Hidden YouTube player div */}
      <div id="youtube-player" style={{ display: 'none' }} />
    </PlayerContext.Provider>
  )
}

export function usePlayer() {
  const context = useContext(PlayerContext)
  if (context === undefined) {
    throw new Error('usePlayer must be used within a PlayerProvider')
  }
  return context
}

// YouTube API types
declare global {
  interface Window {
    YT: any;
    onYouTubeIframeAPIReady: () => void;
  }
}

