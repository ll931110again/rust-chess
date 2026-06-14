import type { Metadata } from "next";
import { Inter, Source_Serif_4 } from "next/font/google";
import "./globals.css";

const sans = Inter({
  subsets: ["latin"],
  variable: "--font-sans",
});

const serif = Source_Serif_4({
  subsets: ["latin"],
  variable: "--font-serif",
});

export const metadata: Metadata = {
  title: "QualityDocs — Technical Design Library",
  description:
    "High-quality engineering design documents, architecture write-ups, and research deep-dives.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className={`${sans.variable} ${serif.variable}`}>
      <body className="min-h-screen antialiased">
        <header className="border-b border-[var(--border)] bg-[var(--surface)]/80 backdrop-blur-sm sticky top-0 z-50">
          <div className="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
            <a href="/" className="flex items-center gap-3 group">
              <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-blue-500 to-violet-600 flex items-center justify-center text-white text-sm font-bold">
                Q
              </div>
              <span className="text-lg font-semibold tracking-tight">
                Quality<span className="gradient-text">Docs</span>
              </span>
            </a>
            <p className="text-sm text-[var(--muted)] hidden sm:block">
              Technical design documents, curated for depth
            </p>
          </div>
        </header>
        <main>{children}</main>
        <footer className="border-t border-[var(--border)] mt-20 py-8 text-center text-sm text-[var(--muted)]">
          QualityDocs — Open technical literature for engineers
        </footer>
      </body>
    </html>
  );
}
