const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

export interface SearchResult {
  id: number;
  url: string;
  domain: string;
  title: string;
  summary: string;
  content_type: string;
  quality_score: number;
  topics: string;
  indexed_at: string;
  relevance: number;
}

export interface SearchResponse {
  results: SearchResult[];
  query: string;
  site_filter: string | null;
}

export interface NewsItem {
  id: number;
  url: string;
  domain: string;
  title: string;
  summary: string;
  quality_score: number;
  topics: string;
  indexed_at: string;
}

export interface NewsResponse {
  items: NewsItem[];
  total: number;
}

export interface SiteInfo {
  domain: string;
  page_count: number;
}

export async function search(params: {
  q: string;
  site?: string;
  limit?: number;
}): Promise<SearchResponse> {
  const search = new URLSearchParams({ q: params.q });
  if (params.site) search.set("site", params.site);
  if (params.limit) search.set("limit", String(params.limit));

  const res = await fetch(`${API_URL}/api/search?${search}`, {
    cache: "no-store",
  });
  if (!res.ok) throw new Error("Search failed");
  return res.json();
}

export async function fetchNews(params?: {
  limit?: number;
  offset?: number;
}): Promise<NewsResponse> {
  const search = new URLSearchParams();
  if (params?.limit) search.set("limit", String(params.limit));
  if (params?.offset) search.set("offset", String(params.offset));

  const res = await fetch(`${API_URL}/api/news?${search}`, {
    next: { revalidate: 60 },
  });
  if (!res.ok) throw new Error("News fetch failed");
  return res.json();
}

export async function fetchSites(): Promise<SiteInfo[]> {
  const res = await fetch(`${API_URL}/api/sites`, {
    next: { revalidate: 300 },
  });
  if (!res.ok) throw new Error("Sites fetch failed");
  return res.json();
}

export function parseTopics(topicsJson: string): string[] {
  try {
    return JSON.parse(topicsJson);
  } catch {
    return [];
  }
}

export function qualityColor(score: number): string {
  if (score >= 85) return "text-emerald-400 bg-emerald-900/30";
  if (score >= 70) return "text-blue-400 bg-blue-900/30";
  return "text-amber-400 bg-amber-900/30";
}
