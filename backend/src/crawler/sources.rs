pub struct FeedSource {
    pub name: &'static str,
    pub url: &'static str,
    pub content_type: &'static str,
}

pub fn news_sources() -> Vec<FeedSource> {
    vec![
        FeedSource {
            name: "Ars Technica",
            url: "https://feeds.arstechnica.com/arstechnica/index",
            content_type: "article",
        },
        FeedSource {
            name: "BBC Science",
            url: "https://feeds.bbci.co.uk/news/science_and_environment/rss.xml",
            content_type: "article",
        },
        FeedSource {
            name: "Nature News",
            url: "https://www.nature.com/nature.rss",
            content_type: "article",
        },
        FeedSource {
            name: "Quanta Magazine",
            url: "https://www.quantamagazine.org/feed/",
            content_type: "article",
        },
        FeedSource {
            name: "MIT Technology Review",
            url: "https://www.technologyreview.com/feed/",
            content_type: "article",
        },
        FeedSource {
            name: "NYT Science",
            url: "https://rss.nytimes.com/services/xml/rss/nyt/Science.xml",
            content_type: "article",
        },
        FeedSource {
            name: "NASA",
            url: "https://www.nasa.gov/rss/dyn/breaking_news.rss",
            content_type: "article",
        },
        FeedSource {
            name: "The Conversation",
            url: "https://theconversation.com/us/articles.atom",
            content_type: "article",
        },
        FeedSource {
            name: "ScienceDaily",
            url: "https://www.sciencedaily.com/rss/all.xml",
            content_type: "article",
        },
        FeedSource {
            name: "Reuters Science",
            url: "https://www.reutersagency.com/feed/?taxonomy=best-topics&post_type=best",
            content_type: "article",
        },
        FeedSource {
            name: "Brookings Tech",
            url: "https://www.brookings.edu/topic/innovation/feed/",
            content_type: "article",
        },
        FeedSource {
            name: "IEEE Spectrum",
            url: "https://spectrum.ieee.org/feeds/feed.rss",
            content_type: "article",
        },
    ]
}
