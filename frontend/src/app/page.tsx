import Link from "next/link";
import { Suspense } from "react";
import { SearchBar, TagFilter } from "@/components/SearchBar";
import { fetchDocuments, parseTags } from "@/lib/api";

export const dynamic = "force-dynamic";

async function DocumentGrid({
  q,
  tag,
}: {
  q?: string;
  tag?: string;
}) {
  const { documents, total } = await fetchDocuments({ q, tag });

  const allTags = Array.from(
    new Set(documents.flatMap((d) => parseTags(d.tags)))
  ).sort();

  if (documents.length === 0) {
    return (
      <div className="text-center py-20 text-[var(--muted)]">
        No documents match your search.
      </div>
    );
  }

  return (
    <>
      <div className="mb-8">
        <TagFilter tags={allTags} activeTag={tag} />
      </div>
      <p className="text-sm text-[var(--muted)] mb-6">
        {total} document{total !== 1 ? "s" : ""}
      </p>
      <div className="grid gap-5 md:grid-cols-2">
        {documents.map((doc) => (
          <Link
            key={doc.slug}
            href={`/docs/${doc.slug}`}
            className="group block p-6 rounded-xl bg-[var(--surface)] border border-[var(--border)] hover:border-[var(--accent)]/40 transition"
          >
            <div className="flex items-start justify-between gap-4 mb-3">
              <span className="text-xs font-medium text-[var(--accent)] uppercase tracking-wider">
                {doc.source_name}
              </span>
              <span className="text-xs text-[var(--muted)]">
                {doc.quality_score}/100
              </span>
            </div>
            <h2 className="text-lg font-semibold mb-2 group-hover:text-[var(--accent)] transition">
              {doc.title}
            </h2>
            <p className="text-sm text-[var(--muted)] line-clamp-2 mb-4">
              {doc.summary}
            </p>
            <div className="flex flex-wrap gap-1.5">
              {parseTags(doc.tags).map((t) => (
                <span
                  key={t}
                  className="text-xs px-2 py-0.5 rounded bg-[var(--bg)] text-[var(--muted)]"
                >
                  {t}
                </span>
              ))}
            </div>
          </Link>
        ))}
      </div>
    </>
  );
}

export default async function Home({
  searchParams,
}: {
  searchParams: Promise<{ q?: string; tag?: string }>;
}) {
  const params = await searchParams;

  return (
    <div className="max-w-6xl mx-auto px-6 py-12">
      <section className="mb-12">
        <h1 className="text-4xl md:text-5xl font-bold tracking-tight mb-4">
          Technical design docs,{" "}
          <span className="gradient-text">without the slop</span>
        </h1>
        <p className="text-lg text-[var(--muted)] max-w-2xl mb-8">
          Curated engineering literature from OpenAI, Google, Anthropic, and
          beyond. Dense, academic, and worth your time.
        </p>
        <Suspense fallback={<div className="h-12" />}>
          <SearchBar initialQuery={params.q || ""} />
        </Suspense>
      </section>

      <Suspense
        fallback={
          <div className="grid gap-5 md:grid-cols-2">
            {[...Array(4)].map((_, i) => (
              <div
                key={i}
                className="h-48 rounded-xl bg-[var(--surface)] animate-pulse"
              />
            ))}
          </div>
        }
      >
        <DocumentGrid q={params.q} tag={params.tag} />
      </Suspense>
    </div>
  );
}
