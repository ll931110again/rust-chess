"use client";

import { FormEvent, useState } from "react";
import {
  fetchSites,
  parseTopics,
  qualityColor,
  search,
  SearchResult,
  SiteInfo,
} from "@/lib/api";
import { useEffect } from "react";

export default function SearchPage() {
  const [query, setQuery] = useState("");
  const [site, setSite] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [sites, setSites] = useState<SiteInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [searched, setSearched] = useState(false);
  const [scope, setScope] = useState<"global" | "site">("global");

  useEffect(() => {
    fetchSites().then(setSites).catch(() => {});
  }, []);

  async function onSubmit(e: FormEvent) {
    e.preventDefault();
    if (!query.trim()) return;
    setLoading(true);
    setSearched(true);
    try {
      const res = await search({
        q: query.trim(),
        site: scope === "site" && site ? site : undefined,
        limit: 20,
      });
      setResults(res.results);
    } catch {
      setResults([]);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="max-w-4xl mx-auto px-6 py-12">
      <h1 className="font-[family-name:var(--font-display)] text-3xl font-bold mb-2">
        Omnisearch
      </h1>
      <p className="text-[var(--muted)] mb-8">
        Quality-weighted search across indexed public content.
      </p>

      <div className="flex gap-2 mb-6">
        <button
          onClick={() => setScope("global")}
          className={`px-4 py-2 rounded-lg text-sm font-medium transition ${
            scope === "global"
              ? "bg-[var(--accent)]/15 text-[var(--accent)] border border-[var(--accent)]/40"
              : "border border-[var(--border)] text-[var(--muted)]"
          }`}
        >
          Global
        </button>
        <button
          onClick={() => setScope("site")}
          className={`px-4 py-2 rounded-lg text-sm font-medium transition ${
            scope === "site"
              ? "bg-[var(--accent-2)]/15 text-[var(--accent-2)] border border-[var(--accent-2)]/40"
              : "border border-[var(--border)] text-[var(--muted)]"
          }`}
        >
          Site-scoped
        </button>
      </div>

      <form onSubmit={onSubmit} className="space-y-4 mb-10">
        <input
          type="search"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search transformers, databases, climate science..."
          className="w-full px-5 py-4 rounded-xl bg-[var(--surface)] border border-[var(--border)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
        />
        {scope === "site" && (
          <select
            value={site}
            onChange={(e) => setSite(e.target.value)}
            className="w-full px-4 py-3 rounded-xl bg-[var(--surface)] border border-[var(--border)] text-[var(--text)]"
          >
            <option value="">Select a site...</option>
            {sites.map((s) => (
              <option key={s.domain} value={s.domain}>
                {s.domain} ({s.page_count} pages)
              </option>
            ))}
          </select>
        )}
        <button
          type="submit"
          disabled={loading || !query.trim()}
          className="px-6 py-3 rounded-xl bg-[var(--accent)] text-[var(--bg)] font-semibold disabled:opacity-50"
        >
          {loading ? "Searching..." : "Search"}
        </button>
      </form>

      {searched && results.length === 0 && !loading && (
        <p className="text-center text-[var(--muted)] py-12">
          No results found. Try different keywords.
        </p>
      )}

      <div className="space-y-4">
        {results.map((r) => (
          <a
            key={r.id}
            href={r.url}
            target="_blank"
            rel="noopener noreferrer"
            className="block p-5 rounded-xl bg-[var(--surface)] border border-[var(--border)] hover:border-[var(--accent)]/30 transition"
          >
            <div className="flex items-center justify-between gap-4 mb-2">
              <span className="text-xs text-[var(--accent-2)]">{r.domain}</span>
              <span
                className={`text-xs px-2 py-0.5 rounded-full font-medium ${qualityColor(r.quality_score)}`}
              >
                Quality {r.quality_score}
              </span>
            </div>
            <h3 className="font-semibold text-lg mb-1">{r.title}</h3>
            <p className="text-sm text-[var(--muted)] mb-3">{r.summary}</p>
            <div className="flex flex-wrap gap-1.5">
              {parseTopics(r.topics).map((t) => (
                <span
                  key={t}
                  className="text-xs px-2 py-0.5 rounded bg-[var(--surface-2)] text-[var(--muted)]"
                >
                  {t}
                </span>
              ))}
            </div>
          </a>
        ))}
      </div>
    </div>
  );
}
