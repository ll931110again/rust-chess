import Link from "next/link";

export default function Home() {
  return (
    <div className="max-w-6xl mx-auto px-6 py-16">
      <section className="text-center mb-20">
        <h1 className="font-[family-name:var(--font-display)] text-5xl md:text-6xl font-bold tracking-tight mb-6">
          Search the web on{" "}
          <span className="gradient-brand">your terms</span>
        </h1>
        <p className="text-lg text-[var(--muted)] max-w-2xl mx-auto mb-10">
          Omnisearch ranks for quality and substance. Curated News filters out
          sensationalism. Two tools, one standard: information worth your
          attention.
        </p>
        <div className="flex flex-wrap justify-center gap-4">
          <Link
            href="/search"
            className="px-6 py-3 rounded-xl bg-[var(--accent)] text-[var(--bg)] font-semibold hover:opacity-90 transition glow-accent"
          >
            Start searching
          </Link>
          <Link
            href="/news"
            className="px-6 py-3 rounded-xl border border-[var(--border)] text-[var(--text)] hover:border-[var(--accent)] transition"
          >
            Browse curated feed
          </Link>
        </div>
      </section>

      <section className="grid md:grid-cols-2 gap-6">
        <div className="p-8 rounded-2xl bg-[var(--surface)] border border-[var(--border)] glow-accent">
          <div className="w-12 h-12 rounded-xl bg-[var(--surface-2)] flex items-center justify-center mb-5 text-2xl">
            🔍
          </div>
          <h2 className="font-[family-name:var(--font-display)] text-2xl font-semibold mb-3">
            Omnisearch
          </h2>
          <p className="text-[var(--muted)] mb-4">
            Cross-site search with quality-weighted ranking. Filter by domain or
            search globally across indexed technical content.
          </p>
          <Link href="/search" className="text-[var(--accent)] text-sm font-medium">
            Open search →
          </Link>
        </div>
        <div className="p-8 rounded-2xl bg-[var(--surface)] border border-[var(--border)]">
          <div className="w-12 h-12 rounded-xl bg-[var(--surface-2)] flex items-center justify-center mb-5 text-2xl">
            📰
          </div>
          <h2 className="font-[family-name:var(--font-display)] text-2xl font-semibold mb-3">
            Curated News
          </h2>
          <p className="text-[var(--muted)] mb-4">
            A feed scored for informativeness over clickbait. Under-reported
            topics surfaced alongside essential technical news.
          </p>
          <Link href="/news" className="text-[var(--accent-2)] text-sm font-medium">
            View feed →
          </Link>
        </div>
      </section>
    </div>
  );
}
