"use client";

import { useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { explainSection } from "@/lib/api";

export function ExplainPanel({ slug }: { slug: string }) {
  const [selectedText, setSelectedText] = useState("");
  const [explanation, setExplanation] = useState("");
  const [loading, setLoading] = useState(false);
  const [aiPowered, setAiPowered] = useState(false);
  const [open, setOpen] = useState(false);

  function captureSelection() {
    const text = window.getSelection()?.toString().trim();
    if (text && text.length > 20) {
      setSelectedText(text);
      setOpen(true);
    }
  }

  async function handleExplain() {
    if (!selectedText) return;
    setLoading(true);
    try {
      const result = await explainSection(slug, selectedText);
      setExplanation(result.explanation);
      setAiPowered(result.ai_powered);
    } catch {
      setExplanation("Unable to generate explanation. Please try again.");
    } finally {
      setLoading(false);
    }
  }

  return (
    <>
      <button
        onClick={captureSelection}
        className="fixed bottom-6 right-6 px-4 py-2.5 rounded-full bg-gradient-to-r from-blue-600 to-violet-600 text-white text-sm font-medium shadow-lg shadow-blue-900/30 hover:opacity-90 transition z-40"
      >
        Explain selection
      </button>

      {open && (
        <div className="fixed inset-y-0 right-0 w-full max-w-md bg-[var(--surface)] border-l border-[var(--border)] shadow-2xl z-50 flex flex-col">
          <div className="p-5 border-b border-[var(--border)] flex items-center justify-between">
            <h3 className="font-semibold">AI Explain</h3>
            <button
              onClick={() => setOpen(false)}
              className="text-[var(--muted)] hover:text-[var(--text)]"
            >
              ✕
            </button>
          </div>
          <div className="p-5 flex-1 overflow-y-auto space-y-4">
            <div>
              <p className="text-xs uppercase tracking-wider text-[var(--muted)] mb-2">
                Selected text
              </p>
              <p className="text-sm bg-[var(--bg)] p-3 rounded-lg border border-[var(--border)] line-clamp-6">
                {selectedText || "Select text in the document, then click Explain."}
              </p>
            </div>
            <button
              onClick={handleExplain}
              disabled={!selectedText || loading}
              className="w-full py-2.5 rounded-lg bg-[var(--accent)] text-white font-medium disabled:opacity-50"
            >
              {loading ? "Explaining..." : "Explain this section"}
            </button>
            {explanation && (
              <div>
                <div className="flex items-center gap-2 mb-2">
                  <p className="text-xs uppercase tracking-wider text-[var(--muted)]">
                    Explanation
                  </p>
                  {aiPowered && (
                    <span className="text-xs px-2 py-0.5 rounded bg-green-900/40 text-green-400">
                      AI
                    </span>
                  )}
                </div>
                <div className="prose-docs text-sm">
                  <ReactMarkdown remarkPlugins={[remarkGfm]}>
                    {explanation}
                  </ReactMarkdown>
                </div>
              </div>
            )}
          </div>
        </div>
      )}
    </>
  );
}
