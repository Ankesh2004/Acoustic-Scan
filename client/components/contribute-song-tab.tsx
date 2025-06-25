'use client';

import { useState } from 'react';
import { ApiClient } from '@/lib/api';
import { Dropzone } from './dropzone';

export function ContributeSongTab() {
  const [isUploading, setIsUploading] = useState(false);
  const [spotifyUrl, setSpotifyUrl] = useState('');
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  const handleFileUpload = async (file: File) => {
    setIsUploading(true);
    setMessage(null);

    try {
      await ApiClient.saveSong(file, true); // force=true to allow without YouTube ID
      setMessage({ type: 'success', text: 'Song saved successfully!' });
    } catch (err) {
      setMessage({ 
        type: 'error', 
        text: err instanceof Error ? err.message : 'Failed to save song' 
      });
    } finally {
      setIsUploading(false);
    }
  };

  const handleSpotifyDownload = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!spotifyUrl.trim()) return;

    setIsUploading(true);
    setMessage(null);

    try {
      await ApiClient.downloadSong(spotifyUrl);
      setMessage({ type: 'success', text: 'Song downloaded and saved successfully!' });
      setSpotifyUrl('');
    } catch (err) {
      setMessage({ 
        type: 'error', 
        text: err instanceof Error ? err.message : 'Failed to download song' 
      });
    } finally {
      setIsUploading(false);
    }
  };

  return (
    <div className="space-y-6">
      {/* File Upload Section */}
      <div>
        <h3 className="text-lg font-semibold mb-2">Upload Audio File</h3>
        <Dropzone 
          onFileUpload={handleFileUpload}
          disabled={isUploading}
          acceptedTypes={['audio/*']}
        />
      </div>

      {/* Spotify Download Section */}
      <div>
        <h3 className="text-lg font-semibold mb-2">Download from Spotify</h3>
        <form onSubmit={handleSpotifyDownload} className="space-y-2">
          <input
            type="url"
            value={spotifyUrl}
            onChange={(e) => setSpotifyUrl(e.target.value)}
            placeholder="https://open.spotify.com/track/..."
            className="w-full p-2 border border-gray-300 rounded-md"
            disabled={isUploading}
          />
          <button
            type="submit"
            disabled={isUploading || !spotifyUrl.trim()}
            className="w-full bg-green-600 text-white py-2 px-4 rounded-md hover:bg-green-700 disabled:opacity-50"
          >
            {isUploading ? 'Downloading...' : 'Download Song'}
          </button>
        </form>
      </div>

      {/* Status Message */}
      {message && (
        <div className={`p-4 rounded-md ${
          message.type === 'success' 
            ? 'bg-green-50 border border-green-200 text-green-800'
            : 'bg-red-50 border border-red-200 text-red-800'
        }`}>
          {message.text}
        </div>
      )}
    </div>
  );
}

