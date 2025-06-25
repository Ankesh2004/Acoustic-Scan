import Link from "next/link"
import { ArrowLeft, ExternalLink } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Navbar } from "@/components/navbar"

export default function YouTubeIdHelpPage() {
  return (
    <div className="flex min-h-screen flex-col">
      <Navbar />
      <main className="flex-1 container py-8 md:py-12">
        <div className="mx-auto max-w-3xl space-y-6">
          <div className="flex items-center gap-2">
            <Button variant="ghost" size="sm" asChild>
              <Link href="/scan" className="flex items-center gap-2">
                <ArrowLeft className="h-4 w-4" />
                Back to Contribute
              </Link>
            </Button>
          </div>

          <Card>
            <CardHeader>
              <CardTitle className="text-2xl">How to Get a YouTube Video ID</CardTitle>
              <CardDescription>Follow these simple steps to find the ID of any YouTube video</CardDescription>
            </CardHeader>
            <CardContent className="space-y-6">
              <div className="space-y-4">
                <div className="space-y-2">
                  <h3 className="text-lg font-medium">Method 1: From the YouTube URL</h3>
                  <ol className="list-decimal list-inside space-y-3 text-muted-foreground">
                    <li className="pl-2">Go to the YouTube video you want to identify</li>
                    <li className="pl-2">
                      Look at the URL in your browser's address bar. It will look something like:
                      <div className="mt-2 rounded-md bg-muted p-3 flex items-center justify-between">
                        <code className="text-sm">
                          https://www.youtube.com/watch?v=
                          <span className="text-acoustic-purple font-semibold">dQw4w9WgXcQ</span>
                        </code>
                      </div>
                    </li>
                    <li className="pl-2">
                      The part after <code className="text-sm">v=</code> is the video ID (highlighted in purple above)
                    </li>
                    <li className="pl-2">
                      If there are additional parameters (like <code className="text-sm">&t=30s</code>), don't include
                      them
                    </li>
                  </ol>
                </div>

                <div className="space-y-2">
                  <h3 className="text-lg font-medium">Method 2: From a Shared Link</h3>
                  <ol className="list-decimal list-inside space-y-3 text-muted-foreground">
                    <li className="pl-2">Click the "Share" button below the YouTube video</li>
                    <li className="pl-2">
                      Copy the shortened URL that appears. It will look like:
                      <div className="mt-2 rounded-md bg-muted p-3 flex items-center justify-between">
                        <code className="text-sm">
                          https://youtu.be/<span className="text-acoustic-purple font-semibold">dQw4w9WgXcQ</span>
                        </code>
                      </div>
                    </li>
                    <li className="pl-2">
                      The part after <code className="text-sm">youtu.be/</code> is the video ID
                    </li>
                  </ol>
                </div>

                <div className="space-y-2">
                  <h3 className="text-lg font-medium">Method 3: From Embedded Videos</h3>
                  <ol className="list-decimal list-inside space-y-3 text-muted-foreground">
                    <li className="pl-2">Right-click on an embedded YouTube video and select "Copy video URL"</li>
                    <li className="pl-2">The URL will contain the video ID in one of the formats above</li>
                  </ol>
                </div>
              </div>

              <div className="rounded-md border p-4 bg-muted/50">
                <h3 className="font-medium mb-2">Example YouTube IDs:</h3>
                <ul className="space-y-2 text-sm">
                  <li className="flex items-center justify-between">
                    <span>Rick Astley - Never Gonna Give You Up</span>
                    <code className="text-acoustic-purple">dQw4w9WgXcQ</code>
                  </li>
                  <li className="flex items-center justify-between">
                    <span>Billie Eilish - bad guy</span>
                    <code className="text-acoustic-purple">DyDfgMOUjCI</code>
                  </li>
                  <li className="flex items-center justify-between">
                    <span>Queen - Bohemian Rhapsody</span>
                    <code className="text-acoustic-purple">fJ9rUzIMcZQ</code>
                  </li>
                </ul>
              </div>

              <div className="flex justify-between items-center">
                <Button asChild>
                  <Link href="/scan">
                    <ArrowLeft className="mr-2 h-4 w-4" />
                    Back to Contribute
                  </Link>
                </Button>
                <Button variant="outline" asChild>
                  <a
                    href="https://www.youtube.com"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="flex items-center"
                  >
                    Go to YouTube
                    <ExternalLink className="ml-2 h-4 w-4" />
                  </a>
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </main>
    </div>
  )
}

