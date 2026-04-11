# iab-specs-agentic_audience

Agentic Audience v1.0 (Draft) support for the [iab-specs](https://crates.io/crates/iab-specs) ecosystem.

## Overview

Implements the Agentic Audience v1.0 Draft specification for embedding exchange protocol enabling agent-to-agent audience targeting.

- **Embedding envelope** — EmbeddingEnvelope, EmbeddingModel, EmbeddingContext, Embedding
- **Signal taxonomy** — 24 EmbeddingType variants across 7 signal categories
- **Campaign scoring** — CampaignHead, ScoringRequest, ScoringResponse, CampaignScore
- **OpenRTB extension** — EmbeddingSegmentExt for `user.data.segment.ext` transport
- **Enumerations** — ModelType, DistanceMetric, SignalType, TemporalScope, CompositionType, and more

## License

Apache-2.0
