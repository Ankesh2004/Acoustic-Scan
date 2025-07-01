const API_BASE_URL =
  process.env.NEXT_PUBLIC_API_URL ||
  (process.env.NODE_ENV === "production"
    ? "https://acoustic-scan.onrender.com"
    : "http://localhost:8080");

export interface SongMatch {
  song_id: number;
  song_title: string;
  song_artist: string;
  youtube_id: string;
  timestamp: number;
  score: number;
}

export interface LibrarySong {
  id: number;
  name: string;
  artist: string;
  youtube_id: string;
  thumbnail_url?: string;
}

export class ApiClient {
  static async identifySong(audioFile: File): Promise<SongMatch[]> {
    const formData = new FormData();
    formData.append("file", audioFile);

    const response = await fetch(`${API_BASE_URL}/api/find`, {
      method: "POST",
      body: formData,
    });

    if (!response.ok) {
      throw new Error(`Failed to identify song: ${response.statusText}`);
    }

    return response.json();
  }

  static async downloadSong(spotifyUrl: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/api/download`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ url: spotifyUrl }),
    });

    if (!response.ok) {
      throw new Error(`Failed to download song: ${response.statusText}`);
    }
  }

  static async saveSong(
    audioFile: File,
    force: boolean = false
  ): Promise<void> {
    const formData = new FormData();
    formData.append("file", audioFile);

    const url = `${API_BASE_URL}/api/save${force ? "?force=true" : ""}`;
    const response = await fetch(url, {
      method: "POST",
      body: formData,
    });

    if (!response.ok) {
      throw new Error(`Failed to save song: ${response.statusText}`);
    }
  }

  static async eraseDatabase(): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/api/erase`, {
      method: "POST",
    });

    if (!response.ok) {
      throw new Error(`Failed to erase database: ${response.statusText}`);
    }
  }

  static async getLibrary(): Promise<LibrarySong[]> {
    const response = await fetch(`${API_BASE_URL}/api/library`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });

    if (!response.ok) {
      throw new Error(`Failed to fetch library: ${response.statusText}`);
    }

    return response.json();
  }
}
