"use client"

import { useState, useCallback } from "react"
import { useDropzone } from "react-dropzone"
import { Cloud, type File, UploadCloud } from "lucide-react"
import { cn } from "@/lib/utils"

interface DropzoneProps {
  className?: string
  maxFiles?: number
  maxSize?: number
  disabled?: boolean
  accept?: Record<string, string[]>
  onFileUpload: (file: File) => void
  acceptedTypes?: string[]
}

export function Dropzone({
  className,
  onFileUpload,
  maxFiles = 1,
  maxSize = 1024 * 1024 * 10, // 10MB
  disabled = false,
  accept = {
    "audio/*": [".mp3", ".wav", ".ogg", ".flac"],
  },
}: DropzoneProps) {
  const [fileRejections, setFileRejections] = useState<any[]>([])

  const onDrop = useCallback(
    (acceptedFiles: File[], rejectedFiles: any[]) => {
      if (acceptedFiles?.length) {
        acceptedFiles.forEach(file => onFileUpload(file))
      }

      if (rejectedFiles?.length) {
        setFileRejections(rejectedFiles)
      }
    },
    [onFileUpload],
  )

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    maxFiles,
    maxSize,
    disabled,
    accept,
  })

  return (
    <div
      {...getRootProps()}
      className={cn(
        "group relative flex cursor-pointer flex-col items-center justify-center rounded-lg border border-dashed border-muted-foreground/25 px-6 py-10 text-center transition hover:bg-accent/50",
        isDragActive && "border-primary/50 bg-accent/50",
        disabled && "pointer-events-none opacity-60",
        className,
      )}
    >
      <input {...getInputProps()} />

      <div className="flex flex-col items-center justify-center gap-4 text-muted-foreground">
        <div className="relative rounded-full bg-muted p-4 transition-colors group-hover:bg-muted/70">
          {isDragActive ? (
            <Cloud className="h-10 w-10 text-primary animate-pulse" />
          ) : (
            <UploadCloud className="h-10 w-10 transition-colors group-hover:text-primary" />
          )}
        </div>
        <div className="space-y-2">
          <p className="text-sm font-medium">{isDragActive ? "Drop the files here" : "Drag & drop files here"}</p>
          <p className="text-xs">or click to browse</p>
        </div>
      </div>

      {fileRejections.length > 0 && (
        <div className="mt-4 w-full max-w-xs rounded-md bg-destructive/15 p-2 text-xs text-destructive">
          {fileRejections.map(({ file, errors }) => (
            <div key={file.name} className="space-y-1">
              <p className="font-medium">{file.name}</p>
              {errors.map((error: any) => (
                <p key={error.code}>{error.message}</p>
              ))}
            </div>
          ))}
        </div>
      )}
    </div>
  )
}

