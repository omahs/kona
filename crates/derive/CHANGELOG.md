# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.2](https://github.com/ethereum-optimism/kona/compare/kona-derive-v0.0.1...kona-derive-v0.0.2) - 2024-04-05

### Added
- *(derive)* Add `ecrecover` trait + features ([#90](https://github.com/ethereum-optimism/kona/pull/90))
- *(derive)* Use upstream alloy ([#89](https://github.com/ethereum-optimism/kona/pull/89))
- *(derive)* add next_attributes test
- *(workspace)* Add `rustfmt.toml`
- *(derive)* `SpanBatch` type implementation WIP
- *(derive)* Reorganize modules
- *(derive)* `add_txs` function
- *(derive)* Derive raw batches, mocks
- *(derive)* Refactor serialization; `SpanBatchPayload` WIP
- *(derive)* fixed bytes and encoding
- *(derive)* raw span type refactoring
- *(types)* span batches
- *(derive)* Channel Reader Implementation ([#65](https://github.com/ethereum-optimism/kona/pull/65))
- *(derive)* share the rollup config across stages using an arc
- *(derive)* Test Utilities ([#62](https://github.com/ethereum-optimism/kona/pull/62))
- Single batch type ([#43](https://github.com/ethereum-optimism/kona/pull/43))
- *(derive)* channel bank ([#46](https://github.com/ethereum-optimism/kona/pull/46))
- Frame queue stage ([#45](https://github.com/ethereum-optimism/kona/pull/45))
- L1 retrieval ([#44](https://github.com/ethereum-optimism/kona/pull/44))
- System config update event parsing ([#42](https://github.com/ethereum-optimism/kona/pull/42))
- Add OP receipt fields ([#41](https://github.com/ethereum-optimism/kona/pull/41))
- Add `TxDeposit` type ([#40](https://github.com/ethereum-optimism/kona/pull/40))
- L1 traversal ([#39](https://github.com/ethereum-optimism/kona/pull/39))

### Fixed
- *(derive)* Stage Decoupling ([#88](https://github.com/ethereum-optimism/kona/pull/88))
- *(derive)* add back removed test
- *(derive)* lints
- *(derive)* extend attributes queue unit test
- *(derive)* successful payload attributes building tests
- *(derive)* error equality fixes and tests
- *(derive)* rework abstractions and attributes queue testing
- *(derive)* attributes queue
- *(derive)* hoist params
- *(derive)* merge upstream changes
- *(derive)* fix bricked arc stage param construction ([#84](https://github.com/ethereum-optimism/kona/pull/84))
- *(derive)* l1 retrieval docs ([#80](https://github.com/ethereum-optimism/kona/pull/80))
- *(derive)* clean up frame queue docs
- *(derive)* frame queue error bubbling and docs
- *(derive)* rebase
- *(derive)* merge upstream changes
- *(derive)* refactor tx enveloped
- *(derive)* refactor span batch tx types
- *(derive)* bitlist alignment
- *(derive)* span batch tx rlp
- *(derive)* span type encodings and decodings
- *(derive)* more types
- *(derive)* small l1 retrieval doc comment fix ([#61](https://github.com/ethereum-optimism/kona/pull/61))

### Other
- Merge branch 'main' into refcell/data-sources
- Merge pull request [#87](https://github.com/ethereum-optimism/kona/pull/87) from ethereum-optimism/refcell/origin-providers
- Merge branch 'main' into refcell/channel-bank-tests
- Merge branch 'main' into refcell/payload-queue
- *(derive)* L1Traversal Doc and Test Cleanup ([#79](https://github.com/ethereum-optimism/kona/pull/79))
- Merge pull request [#67](https://github.com/ethereum-optimism/kona/pull/67) from ethereum-optimism/refcell/batch-queue
- *(derive)* Channel reader tests + fixes, batch type fixes
- *(derive)* `RawSpanBatch` diff decoding/encoding test
- *(derive)* rebase + move `alloy` module
- *(derive)* Clean up RLP encoding + use `TxType` rather than ints
- Update `derive` lint rules ([#47](https://github.com/ethereum-optimism/kona/pull/47))
- scaffold ([#37](https://github.com/ethereum-optimism/kona/pull/37))
- Make versions of packages independent ([#36](https://github.com/ethereum-optimism/kona/pull/36))
