"use client";

import { useRouter, useSearchParams } from "next/navigation";
import { FormEvent, useState } from "react";

export function SearchBar({ initialQuery = "" }: { initialQuery?: string }) {
  const router = useRouter();
  const searchParams = useSearchParams();
  const [query, setQuery] = useState(initialQuery);

  function onSubmit(e: FormEvent) {
    e.preventDefault();
    const params = new URLSearchParams(searchParams.toString());
    if (query.trim()) {
      params.set("q", query.trim());
    } else {
      params.delete("q");
    }
    params.delete("tag");
    router.push(`/?${params.toString()}`);
  }

  return (
    <form onSubmit={onSubmit} className="relative max-w-xl w-full">
      <input
        type="search"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        placeholder="Search design docs, papers, architecture..."
        className="w-full px-5 py-3.5 pl-12 rounded-xl bg-[var(--surface)] border border-[var(--border)] text-[var(--text)] placeholder:text-[var(--muted)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/50 focus:border-[var(--accent)] transition"
      />
      <svg
        className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-[var(--muted)]"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
        />
      </svg>
    </form>
  );
}

export function TagFilter({
  tags,
  activeTag,
}: {
  tags: string[];
  activeTag?: string;
}) {
  const router = useRouter();
  const searchParams = useSearchParams();

  function setTag(tag: string | null) {
    const params = new URLSearchParams(searchParams.toString());
    if (tag) {
      params.set("tag", tag);
    } else {
      params.delete("tag");
    }
    router.push(`/?${params.toString()}`);
  }

  return (
    <div className="flex flex-wrap gap-2">
      <button
        onClick={() => setTag(null)}
        className={`px-3 py-1.5 rounded-full text-sm border transition ${
          !activeTag
            ? "bg-[var(--accent-soft)] border-[var(--accent)] text-[var(--accent)]"
            : "border-[var(--border)] text-[var(--muted)] hover:border-[var(--accent)]/50"
        }`}
      >
        All
      </button>
      {tags.map((tag) => (
        <button
          key={tag}
          onClick={() => setTag(tag)}
          className={`px-3 py-1.5 rounded-full text-sm border transition ${
            activeTag === tag
              ? "bg-[var(--accent-soft)] border-[var(--accent)] text-[var(--accent)]"
              : "border-[var(--border)] text-[var(--muted)] hover:border-[var(--accent)]/50"
          }`}
        >
          {tag}
        </button>
      ))}
    </div>
  );
}
