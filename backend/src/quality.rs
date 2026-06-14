const SENSATIONAL: &[&str] = &[
    "breaking",
    "shocking",
    "you won't believe",
    "mind-blowing",
    "destroyed",
    "slams",
    "meltdown",
    "bombshell",
    "outrage",
    "viral",
    "clickbait",
    "trump",
    "epstein",
];

const TECHNICAL_BOOST: &[&str] = &[
    "algorithm",
    "architecture",
    "analysis",
    "research",
    "benchmark",
    "implementation",
    "protocol",
    "framework",
    "distributed",
    "inference",
    "optimization",
    "peer-reviewed",
    "methodology",
    "quantitative",
];

const ALLOWLIST: &[&str] = &[
    "arxiv.org",
    "nature.com",
    "pnas.org",
    "research.google",
    "anthropic.com",
    "openai.com",
    "martinfowler.com",
    "distill.pub",
    "cacm.acm.org",
    "ietf.org",
    "nist.gov",
    "quantamagazine.org",
    "brookings.edu",
    "blog.rust-lang.org",
    "llvm.org",
    "github.blog",
];

pub fn score_content(title: &str, content: &str, domain: &str) -> i64 {
    let text = format!("{} {}", title.to_lowercase(), content.to_lowercase());
    let mut score: i64 = 55;

    for word in SENSATIONAL {
        if text.contains(word) {
            score -= 12;
        }
    }

    for word in TECHNICAL_BOOST {
        if text.contains(word) {
            score += 4;
        }
    }

    for allowed in ALLOWLIST {
        if domain.contains(allowed) {
            score += 15;
            break;
        }
    }

    let word_count = content.split_whitespace().count();
    if (150..2500).contains(&word_count) {
        score += 8;
    }

    score.clamp(20, 98)
}

pub fn quality_boost(score: i64) -> f64 {
    0.3 + 0.7 * (score as f64 / 100.0)
}
