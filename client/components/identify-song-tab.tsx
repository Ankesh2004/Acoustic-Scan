'use client';

import { useState } from 'react';
import { ApiClient, SongMatch } from '@/lib/api';
import { Dropzone } from './dropzone';
import { ProcessingAnimation } from './processing-animation';
import { AlertTriangle } from 'lucide-react';

export function IdentifySongTab() {
  const [isProcessing, setIsProcessing] = useState(false);
  const [matches, setMatches] = useState<SongMatch[]>([]);
  const [error, setError] = useState<string | null>(null);

  // Confidence threshold - adjust this value based on system's scoring
  const CONFIDENCE_THRESHOLD = 500.0;

  const handleFileUpload = async (file: File) => {
    setIsProcessing(true);
    setError(null);
    setMatches([]);

    try {
      const results = await ApiClient.identifySong(file);
      setMatches(results);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to identify song');
    } finally {
      setIsProcessing(false);
    }
  };

  // Check if top match has low confidence (for general warning)
  const hasLowConfidence = matches.length > 0 && matches[0].score < CONFIDENCE_THRESHOLD;

  return (
    <div className="space-y-4">
      <Dropzone 
        onFileUpload={handleFileUpload}
        disabled={isProcessing}
        acceptedTypes={['audio/*']}
      />
      
      {isProcessing && <ProcessingAnimation onComplete={() => setIsProcessing(false)} />}
      
      {error && (
        <div className="bg-red-50 border border-red-200 rounded-md p-4">
          <p className="text-red-800">{error}</p>
        </div>
      )}
      
      {matches.length > 0 && (
        <div className="space-y-4">
          {hasLowConfidence && (
            <div className="bg-yellow-50 border border-yellow-200 rounded-md p-4">
              <div className="flex items-start gap-2">
                <AlertTriangle className="h-5 w-5 text-yellow-600 mt-0.5 flex-shrink-0" />
                <div>
                  <h4 className="font-medium text-yellow-800">Low Confidence Matches</h4>
                  <p className="text-yellow-700 text-sm mt-1">
                    Some matches have low confidence scores. These songs might not be in our database 
                    or the audio quality may be poor. Try uploading a clearer audio sample.
                  </p>
                </div>
              </div>
            </div>
          )}
          
          <div className="space-y-2">
            <h3 className="text-lg font-semibold">Matches Found:</h3>
            {matches.map((match, index) => (
              <div key={match.song_id} className={`border rounded-lg p-4 ${
                match.score > CONFIDENCE_THRESHOLD ? 'border-yellow-300 bg-yellow-50' : ''
              }`}>

                <div className="flex justify-between items-start">
                  <div>
                    <h4 className={`font-medium ${match.score > CONFIDENCE_THRESHOLD ? 'text-gray-600' : ''}`}>
                      {match.song_title}
                    </h4>
                    <p className="text-gray-600">{match.song_artist}</p>
                    <p className="text-sm text-gray-500">
                      Score: {match.score.toFixed(1)} | Timestamp: {match.timestamp}ms
                      {match.score < CONFIDENCE_THRESHOLD && (
                        <span className="text-yellow-600 ml-2">⚠ Low confidence</span>
                      )}
                    </p>
                  </div>
                  {match.youtube_id && (
                    <a
                      href={`https://youtube.com/watch?v=${match.youtube_id}`}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-blue-600 hover:text-blue-800"
                    >
                      ▶ Play
                    </a>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

