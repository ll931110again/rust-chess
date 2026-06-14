use crate::crawler::ingest::{merge_doc, FeedItem, FetchedDoc};
use crate::crawler::sources::FeedSource;
use anyhow::Context;
use futures::stream::{self, StreamExt};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

pub struct FetchConfig {
    pub concurrency: usize,
    pub fetch_full_pages: bool,
    pub max_items_per_feed: usize,
}

impl Default for FetchConfig {
    fn default() -> Self {
        Self {
            concurrency: 10,
            fetch_full_pages: true,
            max_items_per_feed: 12,
        }
    }
}

pub fn build_client() -> anyhow::Result<Client> {
    Ok(Client::builder()
        .user_agent("QualityDocsBot/1.0 (+https://qualitydocs.xyz; technical docs crawler)")
        .timeout(Duration::from_secs(25))
        .connect_timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(8)
        .gzip(true)
        .deflate(true)
        .build()?)
}

pub async fn fetch_feed_items(
    client: &Client,
    source: &FeedSource,
    max_items: usize,
) -> anyhow::Result<Vec<FeedItem>> {
    let body = client
        .get(source.url)
        .send()
        .await
        .with_context(|| format!("fetch feed {}", source.url))?
        .error_for_status()?
        .bytes()
        .await?;

    let bytes = &body[..];

    if source.url.contains(".atom")
        || source.url.contains("/default")
        || std::str::from_utf8(bytes)
            .map(|s| s.contains("<feed"))
            .unwrap_or(false)
    {
        if let Ok(feed) = atom_syndication::Feed::read_from(bytes) {
            return Ok(parse_atom(&feed, max_items));
        }
    }

    if let Ok(channel) = rss::Channel::read_from(bytes) {
        return Ok(parse_rss(&channel, max_items));
    }

    if let Ok(feed) = atom_syndication::Feed::read_from(bytes) {
        return Ok(parse_atom(&feed, max_items));
    }

    anyhow::bail!("unsupported feed format: {}", source.url)
}

fn parse_rss(channel: &rss::Channel, max_items: usize) -> Vec<FeedItem> {
    channel
        .items()
        .iter()
        .take(max_items)
        .filter_map(|item| {
            let url = item.link()?.to_string();
            let title = item.title()?.to_string();
            let summary = item
                .description()
                .map(strip_html)
                .or_else(|| item.content().map(strip_html))
                .unwrap_or_default();
            Some(FeedItem {
                title,
                url,
                summary: summary.clone(),
                content_hint: summary,
            })
        })
        .collect()
}

fn parse_atom(feed: &atom_syndication::Feed, max_items: usize) -> Vec<FeedItem> {
    feed.entries()
        .iter()
        .take(max_items)
        .filter_map(|e| {
            let url = e.links().first()?.href().to_string();
            let title = e.title().value.to_string();
            let summary = e
                .summary()
                .map(|s| strip_html(&s.value))
                .or_else(|| e.content().and_then(|c| c.value()).map(|v| strip_html(v)))
                .unwrap_or_default();
            Some(FeedItem {
                title,
                url,
                summary: summary.clone(),
                content_hint: summary,
            })
        })
        .collect()
}

pub async fn enrich_documents(
    client: &Client,
    source_name: &str,
    items: Vec<FeedItem>,
    config: &FetchConfig,
) -> Vec<FetchedDoc> {
    if !config.fetch_full_pages {
        return items
            .into_iter()
            .map(|item| merge_doc(source_name, &item, None))
            .collect();
    }

    let sem = Arc::new(Semaphore::new(config.concurrency));
    let client = client.clone();
    let source_name = source_name.to_string();

    stream::iter(items)
        .map(|item| {
            let client = client.clone();
            let sem = sem.clone();
            let source_name = source_name.clone();
            async move {
                let _permit = sem.acquire().await.ok()?;
                let html = fetch_page_html(&client, &item.url).await.ok();
                Some(merge_doc(&source_name, &item, html.as_deref()))
            }
        })
        .buffer_unordered(config.concurrency)
        .filter_map(|x| async { x })
        .collect()
        .await
}

async fn fetch_page_html(client: &Client, url: &str) -> anyhow::Result<String> {
    Ok(client.get(url).send().await?.error_for_status()?.text().await?)
}

fn strip_html(raw: &str) -> String {
    if !raw.contains('<') {
        return raw.trim().to_string();
    }
    crate::crawler::extract::html_to_text(raw)
}
