const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

export interface DocumentSummary {
  id: number;
  title: string;
  slug: string;
  source_url: string;
  source_name: string;
  summary: string;
  tags: string;
  quality_score: number;
  fetched_at: string;
}

export interface Document extends DocumentSummary {
  content_md: string;
}

export interface DocumentListResponse {
  documents: DocumentSummary[];
  total: number;
}

export interface ExplainResponse {
  explanation: string;
  ai_powered: boolean;
}

export async function fetchDocuments(params?: {
  q?: string;
  tag?: string;
}): Promise<DocumentListResponse> {
  const search = new URLSearchParams();
  if (params?.q) search.set("q", params.q);
  if (params?.tag) search.set("tag", params.tag);

  const res = await fetch(`${API_URL}/api/documents?${search}`, {
    next: { revalidate: 60 },
  });
  if (!res.ok) throw new Error("Failed to fetch documents");
  return res.json();
}

export async function fetchDocument(slug: string): Promise<Document> {
  const res = await fetch(`${API_URL}/api/documents/${slug}`, {
    next: { revalidate: 60 },
  });
  if (!res.ok) throw new Error("Document not found");
  return res.json();
}

export async function explainSection(
  slug: string,
  section: string,
  instruction?: string
): Promise<ExplainResponse> {
  const res = await fetch(`${API_URL}/api/documents/${slug}/explain`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ section, instruction }),
  });
  if (!res.ok) throw new Error("Explain failed");
  return res.json();
}

export function parseTags(tagsJson: string): string[] {
  try {
    return JSON.parse(tagsJson);
  } catch {
    return [];
  }
}
