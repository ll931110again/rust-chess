import type { Metadata } from "next";
import { Space_Grotesk, IBM_Plex_Sans } from "next/font/google";
import Link from "next/link";
import "./globals.css";

const display = Space_Grotesk({
  subsets: ["latin"],
  variable: "--font-display",
});

const body = IBM_Plex_Sans({
  subsets: ["latin"],
  weight: ["400", "500", "600"],
  variable: "--font-body",
});

export const metadata: Metadata = {
  title: "Omnisearch — Quality Search & Curated News",
  description:
    "Cross-site search and curated news feed ranked for substance, not sensationalism.",
};

export default function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en" className={`${display.variable} ${body.variable}`}>
      <body className="font-[family-name:var(--font-body)] min-h-screen antialiased">
        <header className="border-b border-[var(--border)] bg-[var(--surface)]/90 backdrop-blur-md sticky top-0 z-50">
          <div className="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
            <Link href="/" className="flex items-center gap-2.5">
              <div className="w-9 h-9 rounded-xl gradient-brand flex items-center justify-center font-[family-name:var(--font-display)] font-bold text-[var(--bg)] text-lg">
                O
              </div>
              <span className="font-[family-name:var(--font-display)] text-xl font-semibold">
                Omni<span className="gradient-brand">search</span>
              </span>
            </Link>
            <nav className="flex items-center gap-6 text-sm">
              <Link
                href="/search"
                className="text-[var(--muted)] hover:text-[var(--accent)] transition"
              >
                Search
              </Link>
              <Link
                href="/news"
                className="text-[var(--muted)] hover:text-[var(--accent)] transition"
              >
                Curated News
              </Link>
            </nav>
          </div>
        </header>
        <main>{children}</main>
      </body>
    </html>
  );
}
