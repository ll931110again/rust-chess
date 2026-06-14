pub struct FeedSource {
    pub name: &'static str,
    pub url: &'static str,
}

pub fn doc_sources() -> Vec<FeedSource> {
    vec![
        FeedSource {
            name: "Google Research",
            url: "https://research.google/blog/rss/",
        },
        FeedSource {
            name: "Google Developers",
            url: "https://developers.googleblog.com/feeds/posts/default",
        },
        FeedSource {
            name: "OpenAI",
            url: "https://openai.com/blog/rss.xml",
        },
        FeedSource {
            name: "Anthropic",
            url: "https://www.anthropic.com/news/rss",
        },
        FeedSource {
            name: "Rust Blog",
            url: "https://blog.rust-lang.org/feed.xml",
        },
        FeedSource {
            name: "AWS Architecture",
            url: "https://aws.amazon.com/blogs/architecture/feed/",
        },
        FeedSource {
            name: "Cloudflare Blog",
            url: "https://blog.cloudflare.com/rss/",
        },
        FeedSource {
            name: "GitHub Engineering",
            url: "https://github.blog/feed/",
        },
        FeedSource {
            name: "Stripe",
            url: "https://stripe.com/blog/feed.rss",
        },
        FeedSource {
            name: "Martin Fowler",
            url: "https://martinfowler.com/feed.atom",
        },
        FeedSource {
            name: "Meta Engineering",
            url: "https://engineering.fb.com/feed/",
        },
        FeedSource {
            name: "Uber Engineering",
            url: "https://eng.uber.com/feed/",
        },
        FeedSource {
            name: "Netflix Tech",
            url: "https://netflixtechblog.com/feed",
        },
        FeedSource {
            name: "Arxiv CS",
            url: "https://rss.arxiv.org/rss/cs",
        },
        FeedSource {
            name: "LLVM Blog",
            url: "https://blog.llvm.org/feeds/posts/default",
        },
    ]
}
