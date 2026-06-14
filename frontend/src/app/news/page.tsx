import { fetchNews, parseTopics, qualityColor } from "@/lib/api";

export const dynamic = "force-dynamic";

export default async function NewsPage() {
  let items: Awaited<ReturnType<typeof fetchNews>>["items"] = [];
  let total = 0;

  try {
    const data = await fetchNews({ limit: 30 });
    items = data.items;
    total = data.total;
  } catch {
    items = [];
    total = 0;
  }

  return (
    <div className="max-w-4xl mx-auto px-6 py-12">
      <h1 className="font-[family-name:var(--font-display)] text-3xl font-bold mb-2">
        Curated News
      </h1>
      <p className="text-[var(--muted)] mb-2">
        High-quality articles scored for substance over sensationalism.
      </p>
      <p className="text-sm text-[var(--muted)] mb-10">
        {total} articles in feed · minimum quality score 70
      </p>

      <div className="space-y-5">
        {items.map((item, i) => (
          <article
            key={item.id}
            className="p-6 rounded-xl bg-[var(--surface)] border border-[var(--border)] hover:border-[var(--accent-2)]/30 transition"
          >
            <div className="flex items-start gap-4">
              <span className="text-2xl font-[family-name:var(--font-display)] text-[var(--border)] font-bold min-w-[2rem]">
                {String(i + 1).padStart(2, "0")}
              </span>
              <div className="flex-1">
                <div className="flex flex-wrap items-center gap-3 mb-2">
                  <span className="text-xs text-[var(--accent-2)]">
                    {item.domain}
                  </span>
                  <span
                    className={`text-xs px-2 py-0.5 rounded-full font-medium ${qualityColor(item.quality_score)}`}
                  >
                    {item.quality_score}/100
                  </span>
                </div>
                <h2 className="text-lg font-semibold mb-2">
                  <a
                    href={item.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="hover:text-[var(--accent)] transition"
                  >
                    {item.title}
                  </a>
                </h2>
                <p className="text-sm text-[var(--muted)] mb-3">
                  {item.summary}
                </p>
                <div className="flex flex-wrap gap-1.5">
                  {parseTopics(item.topics).map((t) => (
                    <span
                      key={t}
                      className="text-xs px-2 py-0.5 rounded bg-[var(--surface-2)] text-[var(--muted)]"
                    >
                      {t}
                    </span>
                  ))}
                </div>
              </div>
            </div>
          </article>
        ))}
      </div>
    </div>
  );
}
