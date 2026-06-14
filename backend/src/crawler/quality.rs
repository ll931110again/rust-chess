const LOW_SIGNAL: &[&str] = &[
    "top 10",
    "top 5",
    "you won't believe",
    "click here",
    "sponsored",
    "affiliate",
    "giveaway",
    "listicle",
    "hot take",
    "drama",
    "breaking:",
    "slams",
    "destroys",
];

const TECHNICAL: &[&str] = &[
    "architecture",
    "design doc",
    "implementation",
    "algorithm",
    "distributed",
    "database",
    "protocol",
    "benchmark",
    "latency",
    "throughput",
    "specification",
    "rfc",
    "research",
    "engineering",
    "system",
    "scalability",
    "reliability",
    "consistency",
    "deployment",
    "infrastructure",
];

const TRUSTED: &[&str] = &[
    "research.google",
    "openai.com",
    "anthropic.com",
    "engineering.fb.com",
    "aws.amazon.com",
    "netflixtechblog.com",
    "eng.uber.com",
    "stripe.com",
    "cloudflare.com",
    "github.blog",
    "martinfowler.com",
    "blog.rust-lang.org",
    "llvm.org",
    "arxiv.org",
    "acm.org",
    "ieee.org",
];

pub fn score_document(title: &str, content: &str, source_url: &str) -> i64 {
    let text = format!("{} {}", title.to_lowercase(), content.to_lowercase());
    let mut score: i64 = 60;

    for phrase in LOW_SIGNAL {
        if text.contains(phrase) {
            score -= 15;
        }
    }

    for term in TECHNICAL {
        if text.contains(term) {
            score += 5;
        }
    }

    for domain in TRUSTED {
        if source_url.contains(domain) {
            score += 12;
            break;
        }
    }

    let words = content.split_whitespace().count();
    if (400..8000).contains(&words) {
        score += 10;
    } else if words < 200 {
        score -= 20;
    }

    if title.chars().any(|c| c.is_ascii_digit()) && title.to_lowercase().contains("tip") {
        score -= 10;
    }

    score.clamp(25, 98)
}

pub fn min_ingest_score() -> i64 {
    75
}
