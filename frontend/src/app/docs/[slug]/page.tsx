import Link from "next/link";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import rehypeHighlight from "rehype-highlight";
import { ExplainPanel } from "@/components/ExplainPanel";
import { fetchDocument, parseTags } from "@/lib/api";

export const dynamic = "force-dynamic";

export default async function DocPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const doc = await fetchDocument(slug);
  const tags = parseTags(doc.tags);

  const headings = doc.content_md
    .split("\n")
    .filter((line) => line.startsWith("## "))
    .map((line) => ({
      id: line.replace("## ", "").toLowerCase().replace(/\s+/g, "-"),
      text: line.replace("## ", ""),
    }));

  return (
    <div className="max-w-6xl mx-auto px-6 py-10">
      <Link
        href="/"
        className="inline-flex items-center gap-2 text-sm text-[var(--muted)] hover:text-[var(--accent)] mb-8 transition"
      >
        ← Back to library
      </Link>

      <div className="grid lg:grid-cols-[220px_1fr] gap-10">
        {headings.length > 0 && (
          <aside className="hidden lg:block">
            <nav className="sticky top-24 space-y-2">
              <p className="text-xs uppercase tracking-wider text-[var(--muted)] mb-3">
                Contents
              </p>
              {headings.map((h) => (
                <a
                  key={h.id}
                  href={`#${h.id}`}
                  className="block text-sm text-[var(--muted)] hover:text-[var(--accent)] transition truncate"
                >
                  {h.text}
                </a>
              ))}
            </nav>
          </aside>
        )}

        <article>
          <header className="mb-10 pb-8 border-b border-[var(--border)]">
            <div className="flex flex-wrap items-center gap-3 mb-4">
              <span className="text-sm font-medium text-[var(--accent)]">
                {doc.source_name}
              </span>
              <span className="text-[var(--border)]">·</span>
              <span className="text-sm text-[var(--muted)]">
                Quality {doc.quality_score}/100
              </span>
            </div>
            <h1 className="text-3xl md:text-4xl font-bold tracking-tight mb-4">
              {doc.title}
            </h1>
            <p className="text-[var(--muted)] mb-4">{doc.summary}</p>
            <div className="flex flex-wrap gap-2 mb-4">
              {tags.map((t) => (
                <span
                  key={t}
                  className="text-xs px-2.5 py-1 rounded-full bg-[var(--surface)] border border-[var(--border)]"
                >
                  {t}
                </span>
              ))}
            </div>
            <a
              href={doc.source_url}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm text-[var(--accent)] hover:underline"
            >
              View original source →
            </a>
          </header>

          <div className="prose-docs max-w-none">
            <ReactMarkdown
              remarkPlugins={[remarkGfm]}
              rehypePlugins={[rehypeHighlight]}
              components={{
                h2: ({ children }) => {
                  const text = String(children);
                  const id = text.toLowerCase().replace(/\s+/g, "-");
                  return <h2 id={id}>{children}</h2>;
                },
              }}
            >
              {doc.content_md}
            </ReactMarkdown>
          </div>
        </article>
      </div>

      <ExplainPanel slug={slug} />
    </div>
  );
}
