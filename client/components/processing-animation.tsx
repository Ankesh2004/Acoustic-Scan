"use client"

import { useState, useEffect } from "react"
import { Loader2 } from "lucide-react"
import { Progress } from "@/components/ui/progress"

const processingSteps = [
  "Analyzing audio waveform...",
  "Performing FFT analysis...",
  "Extracting frequency peaks...",
  "Generating audio fingerprints...",
  "Matching fingerprints...",
  "Calculating confidence scores...",
  "Ranking potential matches...",
  "Finalizing results...",
]

interface ProcessingAnimationProps {
  onComplete: () => void
  processingTime?: number
}

export function ProcessingAnimation({ onComplete, processingTime = 5000 }: ProcessingAnimationProps) {
  const [progress, setProgress] = useState(0)
  const [currentStep, setCurrentStep] = useState(0)

  useEffect(() => {
    const stepTime = processingTime / processingSteps.length
    const progressInterval = setInterval(() => {
      setProgress((prev) => {
        const newProgress = prev + 100 / (processingTime / 100)
        return newProgress >= 100 ? 100 : newProgress
      })
    }, 100)

    const stepInterval = setInterval(() => {
      setCurrentStep((prev) => {
        const newStep = prev + 1
        return newStep >= processingSteps.length ? processingSteps.length - 1 : newStep
      })
    }, stepTime)

    const timeout = setTimeout(() => {
      clearInterval(progressInterval)
      clearInterval(stepInterval)
      onComplete()
    }, processingTime)

    return () => {
      clearInterval(progressInterval)
      clearInterval(stepInterval)
      clearTimeout(timeout)
    }
  }, [processingTime, onComplete])

  return (
    <div className="flex flex-col items-center justify-center space-y-6 text-center animate-fade-in">
      <div className="relative flex h-20 w-20 items-center justify-center">
        <div className="absolute h-full w-full animate-pulse rounded-full bg-gradient-to-r from-acoustic-purple to-acoustic-blue opacity-20 blur-lg"></div>
        <Loader2 className="h-10 w-10 animate-spin text-primary" />
      </div>

      <div className="space-y-4 w-full max-w-md">
        <h3 className="text-xl font-semibold tracking-tight">Processing Your Audio</h3>
        <p className="text-muted-foreground animate-pulse">{processingSteps[currentStep]}</p>

        <Progress value={progress} className="h-2 w-full" />

        <p className="text-sm text-muted-foreground">{Math.round(progress)}% complete</p>
      </div>
    </div>
  )
}

