Goal

Omnisearch is our own search engine that works on any website.

Right now, when we browse a website, we have to rely on their own search engine, which often priortizes for metrics that are very different from ours.

Example: Youtube, where 80 percent of returning video is trash and low quality. Similarly for Reddit, Facebook, shopping websites, and so forth.

They dictate what we are allowed to see and not to see.

Instead, we will be building our own search engine that can be pluggable to any website. I expect it to be working on public facing websites (we will not be attempting login paywall here).

Given a search query, we will be returning:
* Top K search result item across all websites.
* Top K search result from a website, if the user requests it.

Again, we expect the search results to be high quality, non clickbait, and non sensasional news.

----

Design

We build our own crawler and indexing engine to index all the web pages. This will be continuously running to keep the index fresh.

We expect to embed these web pages into a vector database for fast retrieval of data.

When entering a query, we search through the indexes to find the most relevant items (texts, videos, etc.). Then we use GPT 5.5 or Opus 4.8 to rank the quality of these news items
and only surface / rank the items of high quality.
