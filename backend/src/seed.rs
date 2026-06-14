use sqlx::SqlitePool;

pub struct SeedDoc {
    pub title: &'static str,
    pub slug: &'static str,
    pub source_url: &'static str,
    pub source_name: &'static str,
    pub summary: &'static str,
    pub tags: &'static str,
    pub content_md: &'static str,
}

pub fn seed_documents() -> Vec<SeedDoc> {
    vec![
        SeedDoc {
            title: "Attention Is All You Need",
            slug: "attention-is-all-you-need",
            source_url: "https://arxiv.org/abs/1706.03762",
            source_name: "Google Research",
            summary: "The transformer architecture that replaced recurrence with self-attention, enabling parallel training and state-of-the-art sequence modeling.",
            tags: r#"["transformers","nlp","architecture"]"#,
            content_md: ATTENTION_PAPER,
        },
        SeedDoc {
            title: "Scaling Laws for Neural Language Models",
            slug: "scaling-laws-neural-lm",
            source_url: "https://arxiv.org/abs/2001.08361",
            source_name: "OpenAI",
            summary: "Empirical study of how loss scales as a power law with model size, dataset size, and compute budget.",
            tags: r#"["scaling","training","llm"]"#,
            content_md: SCALING_LAWS,
        },
        SeedDoc {
            title: "Constitutional AI: Harmlessness from AI Feedback",
            slug: "constitutional-ai",
            source_url: "https://www.anthropic.com/research/constitutional-ai",
            source_name: "Anthropic",
            summary: "Training language models to be helpful and harmless using AI-generated critiques guided by a constitution of principles.",
            tags: r#"["alignment","safety","rlhf"]"#,
            content_md: CONSTITUTIONAL_AI,
        },
        SeedDoc {
            title: "MapReduce: Simplified Data Processing on Large Clusters",
            slug: "mapreduce",
            source_url: "https://research.google/pubs/mapreduce-simplified-data-processing-on-large-clusters/",
            source_name: "Google Research",
            summary: "Programming model for processing large datasets in parallel across commodity clusters using map and reduce phases.",
            tags: r#"["distributed-systems","data-processing"]"#,
            content_md: MAPREDUCE,
        },
        SeedDoc {
            title: "The Raft Consensus Algorithm",
            slug: "raft-consensus",
            source_url: "https://raft.github.io/raft.pdf",
            source_name: "Stanford",
            summary: "A consensus algorithm designed for understandability, decomposing replication into leader election, log replication, and safety.",
            tags: r#"["consensus","distributed-systems"]"#,
            content_md: RAFT,
        },
        SeedDoc {
            title: "Rust Ownership and Borrowing",
            slug: "rust-ownership",
            source_url: "https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html",
            source_name: "Rust Book",
            summary: "Core memory management model: each value has one owner, references follow borrowing rules enforced at compile time.",
            tags: r#"["rust","memory","systems"]"#,
            content_md: RUST_OWNERSHIP,
        },
        SeedDoc {
            title: "Dynamo: Amazon's Highly Available Key-value Store",
            slug: "dynamo",
            source_url: "https://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf",
            source_name: "Amazon",
            summary: "Eventually consistent storage system using consistent hashing, vector clocks, and quorum-based replication.",
            tags: r#"["databases","distributed-systems","availability"]"#,
            content_md: DYNAMO,
        },
        SeedDoc {
            title: "Spanner: Google's Globally-Distributed Database",
            slug: "spanner",
            source_url: "https://research.google/pubs/spanner-googles-globally-distributed-database/",
            source_name: "Google Research",
            summary: "Globally distributed database combining external consistency with SQL semantics using TrueTime and two-phase commit.",
            tags: r#"["databases","distributed-systems"]"#,
            content_md: SPANNER,
        },
        SeedDoc {
            title: "LoRA: Low-Rank Adaptation of Large Language Models",
            slug: "lora",
            source_url: "https://arxiv.org/abs/2106.09685",
            source_name: "Microsoft Research",
            summary: "Parameter-efficient fine-tuning by injecting trainable rank-decomposition matrices into transformer layers.",
            tags: r#"["fine-tuning","llm","efficiency"]"#,
            content_md: LORA,
        },
        SeedDoc {
            title: "The Design of the UNIX Operating System",
            slug: "unix-design",
            source_url: "https://en.wikipedia.org/wiki/The_UNIX_Programming_Environment",
            source_name: "Bell Labs",
            summary: "Philosophy of small composable tools, everything-is-a-file abstraction, and pipeline composition.",
            tags: r#"["unix","systems","philosophy"]"#,
            content_md: UNIX_DESIGN,
        },
    ]
}

pub async fn run_seed(pool: &SqlitePool) -> anyhow::Result<()> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM documents")
        .fetch_one(pool)
        .await?;

    if count.0 > 0 {
        return Ok(());
    }

    for doc in seed_documents() {
        sqlx::query(
            "INSERT INTO documents (title, slug, source_url, source_name, content_md, summary, tags, quality_score)
             VALUES (?, ?, ?, ?, ?, ?, ?, 95)",
        )
        .bind(doc.title)
        .bind(doc.slug)
        .bind(doc.source_url)
        .bind(doc.source_name)
        .bind(doc.content_md)
        .bind(doc.summary)
        .bind(doc.tags)
        .execute(pool)
        .await?;
    }

    tracing::info!("Seeded {} documents", seed_documents().len());
    Ok(())
}

const ATTENTION_PAPER: &str = r#"# Attention Is All You Need

## Abstract

The dominant sequence transduction models are based on complex recurrent or convolutional neural networks that include an encoder and a decoder. The best performing models also connect the encoder and decoder through an attention mechanism. We propose a new simple network architecture, the **Transformer**, based solely on attention mechanisms, dispensing with recurrence and convolutions entirely.

## Introduction

Recurrent neural networks, long short-term memory and gated recurrent neural networks in particular, have been firmly established as state of the art approaches in sequence modeling. The fundamental constraint of sequential computation, however, remains.

## Model Architecture

Most competitive neural sequence transduction models have an encoder-decoder structure. The encoder maps an input sequence to a continuous representation. The decoder generates an output sequence one element at a time.

### Self-Attention

Self-attention, sometimes called intra-attention, is an attention mechanism relating different positions of a single sequence in order to compute a representation of the sequence. Self-attention has been used successfully in a variety of tasks including reading comprehension, abstractive summarization, and text entailment.

### Multi-Head Attention

Instead of performing a single attention function with d_model-dimensional keys, values and queries, we found it beneficial to linearly project the queries, keys and values h times with different, learned linear projections. On each of these projected versions, attention is performed in parallel, yielding d_v-dimensional output values.

```
Attention(Q, K, V) = softmax(QK^T / sqrt(d_k)) V
MultiHead(Q,K,V) = Concat(head_1, ..., head_h) W^O
```

## Positional Encoding

Since our model contains no recurrence and no convolution, in order for the model to make use of the order of the sequence, we must inject some information about the relative or absolute position of the tokens. We add positional encodings to the input embeddings at the bottoms of the encoder and decoder stacks.

## Results

On the WMT 2014 English-to-German translation task, the Transformer achieves 28.4 BLEU, improving over the existing best results by over 2 BLEU. On the WMT 2014 English-to-French translation task, our model establishes a new single-model state-of-the-art BLEU score of 41.8.

## Design Decisions

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Attention heads | 8 | Balance parallelization and representational capacity |
| Model dimension | 512 | Standard for base transformer |
| Feed-forward dim | 2048 | 4x expansion ratio |
| Dropout | 0.1 | Regularization across all layers |

The Transformer training cost is significantly lower than recurrent or convolutional architectures with comparable performance.
"#;

const SCALING_LAWS: &str = r#"# Scaling Laws for Neural Language Models

## Overview

We study empirical scaling laws for language model performance on the cross-entropy loss. The loss scales as a power-law with model size, dataset size, and the amount of compute used for training, with some trends spanning more than seven orders of magnitude.

## Key Findings

### Power Law Relationships

When not bottlenecked by data or model size, the test loss L follows:

```
L(N) = (N_c / N)^α_N
L(D) = (D_c / D)^α_D
L(C) = (C_c / C)^α_C
```

Where N is model parameters, D is dataset tokens, and C is compute in petaFLOP-days.

### Compute-Optimal Training

For a given compute budget, there is an optimal allocation between model size and training tokens. Larger models should be trained on more data than suggested by naive scaling.

## Implications for Training

1. **Predictable improvement**: Loss decreases smoothly with scale, enabling extrapolation
2. **Efficiency**: Smaller models trained longer can match larger models trained briefly
3. **Planning**: Compute budgets can be allocated optimally before training begins

## Methodology

We trained autoregressive transformers ranging from 768 parameters to 1.5B parameters on datasets from 22M to 23B tokens. We measured test loss on held-out data and fit power laws using least squares in log space.

## Practical Recommendations

| Compute Budget | Suggested Model Size | Training Tokens |
|----------------|---------------------|-----------------|
| Small (1e18 FLOPs) | 125M | 1.6B |
| Medium (1e20 FLOPs) | 1.3B | 26B |
| Large (1e22 FLOPs) | 13B | 260B |

These relationships hold across architecture variants including width, depth, and attention head count, suggesting fundamental limits rather than architectural artifacts.
"#;

const CONSTITUTIONAL_AI: &str = r#"# Constitutional AI: Harmlessness from AI Feedback

## Problem Statement

Training helpful and harmless AI assistants traditionally requires extensive human feedback. Constitutional AI (CAI) reduces reliance on human labels for harmlessness by using AI-generated critiques guided by a set of principles—a "constitution."

## Approach

### Phase 1: Supervised Learning

1. Generate responses to potentially harmful prompts
2. Ask the model to critique its own response according to constitutional principles
3. Revise the response based on the critique
4. Fine-tune on revised (harmless) responses

### Phase 2: Reinforcement Learning from AI Feedback (RLAIF)

Replace human preference labels with AI-generated preference judgments. The AI evaluator uses the same constitutional principles to compare response pairs.

## The Constitution

Principles are written in natural language, for example:

- "Choose the response that is least intended to build a relationship with the user"
- "Choose the response that is least likely to be viewed as harmful or offensive"
- "Choose the response that is most supportive and encouraging of life"

## Results

CAI-trained models show reduced harmful outputs while maintaining helpfulness. RLAIF achieves comparable performance to RLHF with human labels on harmlessness benchmarks.

## Architecture Considerations

```
Prompt → Initial Response → Self-Critique → Revision → SFT Dataset
                                    ↓
                          Preference Pairs → RLAIF → Aligned Model
```

The approach scales constitutional principles without requiring proportional human labeling effort.

## Limitations

Constitutional principles encode designer values. Principle selection remains a human judgment. The method does not eliminate all failure modes but shifts the bottleneck from labeling scale to principle design.
"#;

const MAPREDUCE: &str = r#"# MapReduce: Simplified Data Processing on Large Clusters

## Motivation

Processing multi-terabyte datasets requires distributing computation across hundreds or thousands of machines. Managing parallelization, fault tolerance, and data distribution manually is error-prone.

## Programming Model

MapReduce provides two primitives:

### Map

The map function processes a key/value pair to generate a set of intermediate key/value pairs:

```
map(k1, v1) → list(k2, v2)
```

### Reduce

The reduce function merges all intermediate values associated with the same intermediate key:

```
reduce(k2, list(v2)) → list(v2)
```

## Execution Overview

1. **Input splitting**: Input files split into M splits (typically 16-64 MB)
2. **Map phase**: Map workers process splits, emit intermediate pairs to local buffers
3. **Shuffle**: Intermediate pairs partitioned by reduce function, transferred to reduce workers
4. **Reduce phase**: Reduce workers sort and merge, apply reduce function
5. **Output**: R reduce output files written to distributed filesystem

## Fault Tolerance

### Worker Failure

Map and reduce tasks are re-executed on failure. Completed map outputs are stored on local disk; re-execution reads from checkpoint.

### Master Failure

Master checkpoints state periodically. On failure, restart from checkpoint or abort job.

## Optimizations

- **Combiner functions**: Local aggregation before shuffle reduces network I/O
- **Backup tasks**: Speculative execution of straggler tasks
- **Partitioning**: Custom partition functions for skewed key distributions
- **Compression**: Intermediate data compressed to reduce bandwidth

## Example: Word Count

```python
def map(name, document):
    for word in document.split():
        emit(word, "1")

def reduce(word, counts):
    emit(word, sum(counts))
```

This abstraction enabled Google's index rebuild, log analysis, and machine learning pipelines at petabyte scale.
"#;

const RAFT: &str = r#"# The Raft Consensus Algorithm

## Goals

Raft was designed as an alternative to Paxos with emphasis on understandability. It decomposes consensus into three relatively independent subproblems:

1. **Leader election**
2. **Log replication**
3. **Safety**

## Server States

Each server is in one of three states: **follower**, **candidate**, or **leader**. Normal operation has exactly one leader and all others are followers.

## Leader Election

- Time divided into terms of arbitrary length
- Each term begins with election; one or more candidates request votes
- Servers become candidates if they receive no heartbeat from leader
- Candidate wins if it receives majority of votes
- Split votes trigger new election with randomized timeouts

## Log Replication

1. Client sends command to leader
2. Leader appends to its log, issues AppendEntries RPCs to followers
3. Entry committed once replicated on majority
4. Leader applies committed entries, responds to client

## Safety Properties

### Election Restriction

A candidate's log must be at least as up-to-date as any other log in the majority. "Up-to-date" compares last term, then log length.

### Commitment

Leader only commits entries from its current term. Previous-term entries committed indirectly through current-term entries.

## Membership Changes

Joint consensus protocol allows configuration changes without sacrificing safety. Cluster transitions through joint configuration combining old and new member sets.

## Performance

Raft performance is comparable to Paxos. Leader handles all client requests; followers passively replicate. Read-only operations can be served by followers with lease-based linearizability.
"#;

const RUST_OWNERSHIP: &str = r#"# Rust Ownership and Borrowing

## The Problem

Systems programming requires manual memory management (error-prone) or garbage collection (runtime overhead, pause times). Rust provides memory safety without GC through ownership.

## Ownership Rules

1. Each value has exactly one owner
2. When the owner goes out of scope, the value is dropped
3. Ownership can be transferred (move semantics)

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved; no longer valid
// println!("{}", s1); // compile error
```

## Borrowing

References allow using values without taking ownership:

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but is not dropped (no ownership)
```

### Borrowing Rules

- At any time, either one mutable reference OR any number of immutable references
- References must always be valid (no dangling references)

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
// let r3 = &mut s; // compile error: cannot borrow as mutable
```

## Lifetimes

The compiler tracks how long references are valid:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Lifetime annotations tell the compiler that returned references live as long as both inputs.

## Why This Matters

Ownership enables:
- Zero-cost abstractions without GC pauses
- Thread safety (Send/Sync traits derived from borrowing rules)
- Prevention of use-after-free, double-free, and data races at compile time

The borrow checker rejects programs that would have undefined behavior in C/C++, shifting errors left to development time.
"#;

const DYNAMO: &str = r#"# Dynamo: Amazon's Highly Available Key-value Store

## Requirements

Amazon's shopping cart and session state require always-on storage. Partition tolerance and high availability take precedence over strong consistency.

## Architecture

### Consistent Hashing

Keys mapped to nodes via hash ring. Virtual nodes improve load distribution when nodes join or leave.

### Vector Clocks

Capture causality between concurrent writes. Clients resolve conflicts during reads (last-write-wins or application-specific merge).

### Sloppy Quorum

W replicas written, R replicas read. R + W > N ensures overlap but allows temporary inconsistency.

### Gossip Protocol

Membership and failure detection via epidemic protocols. No central coordinator.

### Merkle Trees

Anti-entropy: compare hash trees to efficiently synchronize divergent replicas.

## Operations

| Operation | Behavior |
|-----------|----------|
| Get | Read from R nodes, return reconciled value |
| Put | Write to W nodes, vector clock incremented |
| Hinted handoff | Temporary storage when preferred node unavailable |

## Lessons

1. **Application-level resolution**: Push conflict resolution to readers when appropriate
2. **Incremental scalability**: Add nodes without full rebalancing
3. **Heterogeneity**: Exploit varying node capacities via virtual nodes
4. **Operational simplicity**: Gossip beats centralized monitoring at scale

Dynamo influenced Cassandra, Riak, and Voldemort. Its design tradeoffs remain relevant for session stores, shopping carts, and user preferences.
"#;

const SPANNER: &str = r#"# Spanner: Google's Globally-Distributed Database

## Overview

Spanner is the first system to distribute data at global scale and support externally-consistent distributed transactions. It combines SQL semantics with horizontal scaling across datacenters.

## TrueTime API

Spanner uses GPS and atomic clocks to bound clock uncertainty:

```
TT.now() → [earliest, latest]
TT.after(t) → true if t has passed
TT.before(t) → true if t is in the future
```

Commit wait ensures global ordering: transaction waits until TT.after(commit_timestamp) before acknowledging.

## Data Model

- Hierarchical: directories group related data for locality
- Tables sharded across Paxos groups
- Schema changes are non-blocking versioned metadata updates

## Transactions

### Read-Write Transactions

Two-phase commit across Paxos groups with TrueTime commit timestamps. Serializable isolation without locking reads in the common case.

### Read-Only Transactions

Execute at a timestamp chosen at transaction start. No locks, no writes, consistent snapshot across the globe.

## Replication

Each shard replicated via Multi-Paxos. Leader in one datacenter serves reads and writes; followers catch up asynchronously.

## Performance

- Read latency: tens of milliseconds (dominated by WAN RTT for global reads)
- Write latency: includes commit wait (~7ms average for TrueTime uncertainty)
- Throughput: millions of ops/sec across Google's fleet

Spanner demonstrates that strong consistency and global distribution are compatible when time uncertainty is bounded and exposed as a first-class API.
"#;

const LORA: &str = r#"# LoRA: Low-Rank Adaptation of Large Language Models

## Problem

Full fine-tuning of large language models requires updating all parameters—prohibitively expensive for models with billions of parameters and impractical for deploying many task-specific variants.

## Method

LoRA freezes pre-trained weights and injects trainable rank-decomposition matrices into each transformer layer:

```
W' = W + BA
```

Where W ∈ R^(d×k) is frozen, B ∈ R^(d×r), A ∈ R^(r×k), and rank r << min(d, k).

### Parameter Efficiency

For GPT-3 175B with r=4, LoRA reduces trainable parameters by 10,000× and GPU memory by 3× compared to full fine-tuning.

## Which Weights to Adapt

Empirically, adapting attention weights (W_q, W_k, W_v, W_o) matches or exceeds adapting all weights. MLP layers show diminishing returns.

## Inference

LoRA modules can be merged into frozen weights post-training:

```
W_merged = W + BA
```

No inference latency overhead compared to base model.

## Results

LoRA matches full fine-tuning on GLUE, E2E NLG, and commonsense reasoning benchmarks while enabling rapid task switching by swapping LoRA adapters.

## Design Implications

| Aspect | Full FT | LoRA |
|--------|---------|------|
| Trainable params | 100% | ~0.01% |
| Memory | High | Low |
| Multi-task | N copies | N adapters |
| Quality | Baseline | Comparable |

LoRA enables democratized fine-tuning and efficient personalization at scale.
"#;

const UNIX_DESIGN: &str = r#"# The Design of the UNIX Operating System

## Philosophy

UNIX emerged from Bell Labs with a design philosophy that prioritized simplicity, composability, and programmer productivity over feature completeness.

## Core Principles

### Everything Is a File

Devices, sockets, processes (in /proc), and pipes expose file-like interfaces. Uniform read/write/open/close semantics simplify APIs.

### Small Tools

Programs do one thing well. Complex workflows emerge from composition:

```bash
grep "error" /var/log/syslog | awk '{print $1}' | sort | uniq -c | sort -rn
```

### Text Streams

Programs communicate via newline-delimited text. Human-readable, debuggable, language-agnostic.

### Kernel Minimalism

Policy in user space; mechanism in kernel. Filesystems, window systems, and networking stacks live outside the core.

## Process Model

- fork() creates copy-on-write child
- exec() replaces image with new program
- wait() synchronizes parent and child
- Pipes connect stdout to stdin between processes

## File System

Hierarchical namespace starting at /. Hard links, symbolic links, and permissions (owner/group/other) provide flexible organization.

## Impact

UNIX influenced Linux, macOS, BSD, and the POSIX standard. Its design patterns—pipes, filters, everything-is-a-file—appear in modern cloud infrastructure (Kubernetes pods, S3 objects, Unix sockets).

## Lessons for System Design

1. **Constraints enable creativity**: Limited primitives force elegant compositions
2. **Optimize for debugging**: Text interfaces are inspectable without specialized tools
3. **Separate policy from mechanism**: User-space flexibility without kernel recompilation
"#;
