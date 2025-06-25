import Link from "next/link"
import { ArrowRight, Music } from "lucide-react"

export default function HomePage() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-slate-900 to-slate-800 px-4 text-center">
      <div className="max-w-3xl space-y-8 animate-fade-in">
        <div className="flex justify-center">
          <div className="relative">
            <div className="absolute -inset-1 rounded-full bg-gradient-to-r from-acoustic-purple to-acoustic-blue opacity-75 blur-lg animate-pulse-opacity"></div>
            <div className="relative flex h-16 w-16 items-center justify-center rounded-full bg-slate-900">
              <Music className="h-8 w-8 text-white" />
            </div>
          </div>
        </div>

        <h1 className="bg-gradient-to-r from-acoustic-purple to-acoustic-blue bg-clip-text text-5xl font-bold tracking-tight text-transparent sm:text-6xl md:text-7xl">
          Acoustic Scan
        </h1>

        <p className="text-xl text-slate-300 sm:text-2xl">Identify. Discover. Enrich.</p>

        <div className="flex flex-col items-center justify-center gap-4 sm:flex-row">
          <Link
            href="/scan"
            className="group relative inline-flex h-12 items-center justify-center overflow-hidden rounded-lg bg-gradient-to-br from-acoustic-purple to-acoustic-blue px-8 py-3 font-medium text-white transition-all duration-300 ease-out hover:scale-105"
          >
            <span className="absolute right-0 -mt-12 h-32 w-8 translate-x-12 rotate-12 transform bg-white opacity-10 transition-all duration-1000 ease-out group-hover:-translate-x-40"></span>
            <span className="relative flex items-center gap-2">
              Get Started
              <ArrowRight className="h-4 w-4 transition-transform duration-300 group-hover:translate-x-1" />
            </span>
          </Link>

          <Link
            href="/api-docs"
            className="group relative inline-flex h-12 items-center justify-center overflow-hidden rounded-lg border border-slate-600 bg-transparent px-8 py-3 font-medium text-slate-200 transition-all duration-300 ease-out hover:border-slate-400 hover:text-white"
          >
            <span className="absolute bottom-0 left-0 h-full w-full bg-gradient-to-r from-acoustic-blue to-acoustic-purple opacity-0 transition-all duration-300 group-hover:opacity-20"></span>
            <span className="relative">Read API Docs</span>
          </Link>
        </div>
      </div>

      <div className="mt-16 text-sm text-slate-500">Powered by Rust-based audio fingerprinting technology</div>
    </div>
  )
}

