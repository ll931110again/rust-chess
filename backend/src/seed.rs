use sqlx::SqlitePool;

use crate::quality;

pub struct SeedPage {
    pub url: &'static str,
    pub domain: &'static str,
    pub title: &'static str,
    pub content: &'static str,
    pub summary: &'static str,
    pub content_type: &'static str,
    pub topics: &'static str,
}

pub fn seed_pages() -> Vec<SeedPage> {
    vec![
        SeedPage {
            url: "https://arxiv.org/abs/2303.08774",
            domain: "arxiv.org",
            title: "GPT-4 Technical Report",
            summary: "Report on GPT-4 capabilities, limitations, and safety evaluations across diverse benchmarks.",
            content_type: "paper",
            topics: r#"["ai","llm","research"]"#,
            content: "We report the development of GPT-4, a large-scale multimodal model accepting image and text inputs. GPT-4 exhibits human-level performance on various professional and academic benchmarks including passing a simulated bar exam in the top 10 percent of test takers.",
        },
        SeedPage {
            url: "https://arxiv.org/abs/2401.04088",
            domain: "arxiv.org",
            title: "Mixtral of Experts",
            summary: "Sparse mixture of experts architecture achieving strong performance with efficient inference.",
            content_type: "paper",
            topics: r#"["ai","llm","architecture"]"#,
            content: "We introduce Mixtral 8x7B, a sparse mixture of experts model. Mixtral outperforms Llama 2 70B on most benchmarks while using fewer active parameters during inference. The model supports a context length of 32k tokens.",
        },
        SeedPage {
            url: "https://research.google/blog/looking-back-at-speculative-decoding/",
            domain: "research.google",
            title: "Speculative Decoding for Faster LLM Inference",
            summary: "Technique using a small draft model to accelerate token generation from large models.",
            content_type: "article",
            topics: r#"["ai","inference","performance"]"#,
            content: "Speculative decoding accelerates autoregressive generation by using a smaller draft model to propose tokens verified in parallel by the target model. This reduces latency without changing output distribution when implemented correctly.",
        },
        SeedPage {
            url: "https://www.anthropic.com/research/claude-character",
            domain: "anthropic.com",
            title: "Claude's Character and Values",
            summary: "How Anthropic approaches AI character training and value alignment in Claude.",
            content_type: "article",
            topics: r#"["ai","alignment","safety"]"#,
            content: "Character training shapes how Claude responds across contexts. We document principles including honesty, helpfulness, and avoiding harm while maintaining nuanced understanding of complex topics.",
        },
        SeedPage {
            url: "https://blog.rust-lang.org/2024/02/08/Rust-1.76.0.html",
            domain: "blog.rust-lang.org",
            title: "Rust 1.76.0 Release Notes",
            summary: "Language improvements including expanded const evaluation and API stabilizations.",
            content_type: "article",
            topics: r#"["rust","programming","release"]"#,
            content: "Rust 1.76 stabilizes several APIs and improves compile times for large projects. The release continues incremental progress on const generics and async trait support.",
        },
        SeedPage {
            url: "https://github.blog/engineering/infrastructure/",
            domain: "github.blog",
            title: "Scaling GitHub's Database Infrastructure",
            summary: "Engineering deep dive into GitHub's database sharding and migration strategies.",
            content_type: "article",
            topics: r#"["databases","infrastructure","scaling"]"#,
            content: "GitHub migrated critical workloads to sharded MySQL clusters while maintaining availability. The approach combines Vitess for horizontal scaling with careful application-level routing.",
        },
        SeedPage {
            url: "https://www.nature.com/articles/s41586-023-06924-6",
            domain: "nature.com",
            title: "Room-Temperature Superconductor Research Update",
            summary: "Peer-reviewed analysis of superconductivity claims and replication efforts.",
            content_type: "paper",
            topics: r#"["physics","materials","science"]"#,
            content: "Recent claims of ambient-pressure room-temperature superconductivity prompted extensive replication attempts. This review synthesizes experimental findings and methodological considerations across laboratories.",
        },
        SeedPage {
            url: "https://distill.pub/2020/grand-canonical/",
            domain: "distill.pub",
            title: "Understanding Neural Networks Through Visualization",
            summary: "Interactive exploration of how neural networks build hierarchical representations.",
            content_type: "article",
            topics: r#"["ai","visualization","education"]"#,
            content: "Distill articles combine clear prose with interactive diagrams. This piece walks through feature visualization techniques that reveal what intermediate layers detect in image classification networks.",
        },
        SeedPage {
            url: "https://martinfowler.com/articles/microservices.html",
            domain: "martinfowler.com",
            title: "Microservices Architecture Guide",
            summary: "Foundational essay on decomposing applications into independently deployable services.",
            content_type: "article",
            topics: r#"["architecture","microservices","software"]"#,
            content: "Microservices trade monolithic simplicity for independent deployability and technology diversity. Successful adoption requires mature DevOps practices, observability, and careful service boundary design.",
        },
        SeedPage {
            url: "https://cacm.acm.org/research/the-scalability-efficiency-and-contention-of-threads/",
            domain: "cacm.acm.org",
            title: "Thread Scalability and Contention",
            summary: "Analysis of when threading helps versus hurts performance on modern hardware.",
            content_type: "paper",
            topics: r#"["systems","concurrency","performance"]"#,
            content: "Not all parallelization improves throughput. Lock contention, false sharing, and scheduler overhead can negate benefits. The paper provides quantitative frameworks for deciding thread granularity.",
        },
        SeedPage {
            url: "https://blog.cloudflare.com/how-we-built-r2/",
            domain: "blog.cloudflare.com",
            title: "Building Cloudflare R2 Object Storage",
            summary: "Design decisions behind S3-compatible storage without egress fees.",
            content_type: "article",
            topics: r#"["storage","cloud","infrastructure"]"#,
            content: "R2 leverages Cloudflare's global network for object storage with zero egress fees. The architecture separates metadata control plane from data plane distributed across edge locations.",
        },
        SeedPage {
            url: "https://stripe.com/blog/how-we-built-it-stripe-radar",
            domain: "stripe.com",
            title: "How Stripe Built Radar for Fraud Detection",
            summary: "Machine learning pipeline for real-time payment fraud scoring at scale.",
            content_type: "article",
            topics: r#"["ml","fraud","fintech"]"#,
            content: "Stripe Radar combines gradient boosted trees with deep networks on billions of transactions. Feature engineering emphasizes behavioral signals and graph relationships between entities.",
        },
        SeedPage {
            url: "https://www.quantamagazine.org/the-hidden-structure-of-chemical-reactions-20240115/",
            domain: "quantamagazine.org",
            title: "Hidden Structure in Chemical Reactions",
            summary: "Mathematical framework revealing patterns in reaction networks beyond traditional chemistry.",
            content_type: "article",
            topics: r#"["chemistry","mathematics","science"]"#,
            content: "Researchers apply category theory and graph algorithms to classify reaction mechanisms. The approach unifies seemingly disparate reactions under common structural templates.",
        },
        SeedPage {
            url: "https://www.lesswrong.com/posts/introduction-to-transformer-circuits",
            domain: "lesswrong.com",
            title: "Introduction to Transformer Circuits",
            summary: "Mechanistic interpretability research mapping computational subgraphs in transformers.",
            content_type: "article",
            topics: r#"["ai","interpretability","research"]"#,
            content: "Transformer circuits analysis identifies attention heads implementing specific algorithms like induction or previous token copying. Understanding these circuits may improve safety and capability predictions.",
        },
        SeedPage {
            url: "https://blog.openai.com/new-models-and-developer-products",
            domain: "openai.com",
            title: "OpenAI Developer Platform Updates",
            summary: "New model releases and API improvements for structured outputs and function calling.",
            content_type: "article",
            topics: r#"["ai","api","developers"]"#,
            content: "OpenAI continues expanding developer tooling with JSON mode, parallel function calling, and improved latency. Documentation emphasizes production deployment patterns and safety best practices.",
        },
        SeedPage {
            url: "https://www.pnas.org/doi/10.1073/pnas.2301234567",
            domain: "pnas.org",
            title: "Ocean Carbon Sink Variability",
            summary: "Long-term measurements of how oceans absorb atmospheric CO2 under climate change.",
            content_type: "paper",
            topics: r#"["climate","oceanography","science"]"#,
            content: "Ocean carbon uptake varies regionally and seasonally. Satellite and in-situ data combined with models suggest weakening sinks in tropical regions offset by Southern Ocean gains.",
        },
        SeedPage {
            url: "https://engineering.fb.com/2023/07/17/data-infrastructure/meta-genai-recsys/",
            domain: "engineering.fb.com",
            title: "Meta's GenAI Recommendation Infrastructure",
            summary: "Infrastructure supporting generative AI features in large-scale recommendation systems.",
            content_type: "article",
            topics: r#"["ai","recommendations","infrastructure"]"#,
            content: "Integrating generative models into feed ranking required new caching layers and latency budgets. Meta describes GPU fleet orchestration and A/B testing frameworks for generative features.",
        },
        SeedPage {
            url: "https://blog.jupyter.org/jupyterlab-4-release/",
            domain: "blog.jupyter.org",
            title: "JupyterLab 4.0 Release",
            summary: "Major update to the interactive computing environment with improved extension API.",
            content_type: "article",
            topics: r#"["tools","data-science","open-source"]"#,
            content: "JupyterLab 4 introduces async kernel management, improved text editor performance, and a stable extension API. The release focuses on developer experience for notebook-based workflows.",
        },
        SeedPage {
            url: "https://www.brookings.edu/articles/quantum-computing-policy-framework/",
            domain: "brookings.edu",
            title: "Quantum Computing Policy Framework",
            summary: "Non-partisan analysis of national quantum investment and regulatory considerations.",
            content_type: "article",
            topics: r#"["policy","quantum","technology"]"#,
            content: "Quantum computing policy must balance research funding, export controls, and workforce development. This framework evaluates international competition without sensational claims about imminent breakthroughs.",
        },
        SeedPage {
            url: "https://blog.crunchydata.com/blog/postgres-as-a-graph-database",
            domain: "blog.crunchydata.com",
            title: "Graph Queries in PostgreSQL",
            summary: "Using recursive CTEs and extensions for graph traversal without dedicated graph DB.",
            content_type: "article",
            topics: r#"["databases","postgres","graphs"]"#,
            content: "Many graph workloads fit PostgreSQL with proper indexing and recursive queries. Apache AGE extension adds openCypher support while retaining ACID guarantees of Postgres.",
        },
        SeedPage {
            url: "https://simonwillison.net/2024/Jan/1/llm-predictions/",
            domain: "simonwillison.net",
            title: "Thoughtful LLM Predictions for 2024",
            summary: "Measured analysis of likely AI developments without hype or doom framing.",
            content_type: "article",
            topics: r#"["ai","analysis","predictions"]"#,
            content: "Predictions focus on practical deployment: smaller fine-tuned models, improved tooling for RAG, and regulatory clarity. Avoids sensational claims about AGI timelines.",
        },
        SeedPage {
            url: "https://www.ietf.org/blog/QUIC-standardization/",
            domain: "ietf.org",
            title: "QUIC Protocol Standardization",
            summary: "Technical overview of HTTP/3 transport layer design and deployment status.",
            content_type: "article",
            topics: r#"["networking","protocols","standards"]"#,
            content: "QUIC multiplexes streams over UDP with built-in TLS 1.3. Connection migration and reduced head-of-line blocking improve mobile and lossy network performance compared to TCP.",
        },
        SeedPage {
            url: "https://blog.mozilla.org/en/firefox/firefox-translations/",
            domain: "blog.mozilla.org",
            title: "Client-Side Translation in Firefox",
            summary: "Privacy-preserving on-device translation using local neural models.",
            content_type: "article",
            topics: r#"["privacy","browser","nlp"]"#,
            content: "Firefox Translations runs Bergamot models locally without sending text to cloud servers. The approach trades model size for privacy and offline capability.",
        },
        SeedPage {
            url: "https://www.sciencedirect.com/science/article/pii/example-battery",
            domain: "sciencedirect.com",
            title: "Solid-State Battery Electrolyte Advances",
            summary: "Materials science progress toward safer high-density battery electrolytes.",
            content_type: "paper",
            topics: r#"["energy","materials","batteries"]"#,
            content: "Solid electrolytes promise higher energy density and reduced fire risk compared to liquid electrolytes. Recent ceramic-polymer composites show improved ionic conductivity at room temperature.",
        },
        SeedPage {
            url: "https://blog.replit.com/ghostwriter-architecture",
            domain: "blog.replit.com",
            title: "Replit Ghostwriter Architecture",
            summary: "How Replit integrated code completion models into their cloud IDE.",
            content_type: "article",
            topics: r#"["devtools","ai","ide"]"#,
            content: "Ghostwriter combines retrieval over codebase context with fine-tuned completion models. Latency budgets require aggressive caching and incremental context window management.",
        },
        SeedPage {
            url: "https://www.eff.org/deeplinks/2024/privacy-preserving-ml",
            domain: "eff.org",
            title: "Privacy-Preserving Machine Learning",
            summary: "Survey of federated learning, differential privacy, and homomorphic encryption tradeoffs.",
            content_type: "article",
            topics: r#"["privacy","ml","policy"]"#,
            content: "Privacy-preserving ML techniques enable training on sensitive data with mathematical guarantees. Each approach trades off accuracy, compute cost, and threat model assumptions differently.",
        },
        SeedPage {
            url: "https://blog.tensorflow.org/2023//tensorflow-2-15-release.html",
            domain: "blog.tensorflow.org",
            title: "TensorFlow 2.15 Release Highlights",
            summary: "Performance improvements and Keras 3 multi-backend support in latest TensorFlow.",
            content_type: "article",
            topics: r#"["ml","framework","tensorflow"]"#,
            content: "TensorFlow 2.15 improves XLA compilation and adds experimental support for JAX interoperability through Keras 3. Documentation covers migration paths from earlier versions.",
        },
        SeedPage {
            url: "https://www.architecture.org/analysis/urban-heat-islands/",
            domain: "architecture.org",
            title: "Urban Heat Island Mitigation Strategies",
            summary: "Evidence-based urban design approaches to reduce city temperature extremes.",
            content_type: "article",
            topics: r#"["urban-planning","climate","architecture"]"#,
            content: "Green roofs, reflective materials, and corridor ventilation reduce localized heating. Studies quantify benefits across climate zones without overstating single-solution fixes.",
        },
        SeedPage {
            url: "https://blog.llvm.org/posts/2024-01-10-flang-fortran/",
            domain: "blog.llvm.org",
            title: "Flang Fortran Frontend in LLVM",
            summary: "Progress on modern Fortran compiler infrastructure within the LLVM project.",
            content_type: "article",
            topics: r#"["compilers","fortran","llvm"]"#,
            content: "Flang aims to replace legacy Fortran compilers with LLVM-based toolchain. Scientific computing communities depend on Fortran for HPC; modern compiler support remains essential.",
        },
        SeedPage {
            url: "https://www.nist.gov/news-events/news/2024/01/post-quantum-cryptography",
            domain: "nist.gov",
            title: "NIST Post-Quantum Cryptography Standards",
            summary: "Finalized algorithms for quantum-resistant encryption and digital signatures.",
            content_type: "article",
            topics: r#"["cryptography","security","standards"]"#,
            content: "NIST selected CRYSTALS-Kyber for key encapsulation and CRYSTALS-Dilithium for signatures. Migration planning requires inventory of cryptographic dependencies across systems.",
        },
        SeedPage {
            url: "https://blog.discord.com/how-discord-stores-trillions-of-messages",
            domain: "blog.discord.com",
            title: "Discord's Message Storage Architecture",
            summary: "Scalable storage design for trillions of chat messages with low-latency retrieval.",
            content_type: "article",
            topics: r#"["databases","messaging","scaling"]"#,
            content: "Discord migrated from MongoDB to Cassandra for message storage at scale. Partitioning strategies align with channel IDs and time-based bucketing for efficient queries.",
        },
        SeedPage {
            url: "https://www.lrb.co.uk/the-paper/v46/n03/example-essay",
            domain: "lrb.co.uk",
            title: "Long-form Essay on Algorithmic Governance",
            summary: "Literary review examining how algorithms shape institutional decision-making.",
            content_type: "article",
            topics: r#"["society","technology","essay"]"#,
            content: "Algorithmic systems increasingly mediate welfare, credit, and hiring decisions. The essay traces historical parallels to bureaucratic automation without resorting to technopanic framing.",
        },
    ]
}

pub async fn run_seed(pool: &SqlitePool) -> anyhow::Result<()> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pages")
        .fetch_one(pool)
        .await?;

    if count.0 > 0 {
        return Ok(());
    }

    for page in seed_pages() {
        let score = quality::score_content(page.title, page.content, page.domain);

        sqlx::query(
            "INSERT INTO pages (url, domain, title, content, summary, content_type, quality_score, topics)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(page.url)
        .bind(page.domain)
        .bind(page.title)
        .bind(page.content)
        .bind(page.summary)
        .bind(page.content_type)
        .bind(score)
        .bind(page.topics)
        .execute(pool)
        .await?;
    }

    crate::db::rebuild_fts(pool).await?;
    tracing::info!("Seeded {} pages", seed_pages().len());
    Ok(())
}
