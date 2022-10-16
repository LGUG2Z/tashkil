# Tashkil
A lightweight Rust library for removing Arabic diacritics (تَشْكِيل)

This library exposes a single function, `tashkil::remove()`, which removes from a `&str` all diacritics in the [unicode specification for the Arabic alphabet and its variants](https://www.unicode.org/charts/PDF/U0600.pdf).

It is my hope that this library can be used to improve search results in [Meilisearch](https://github.com/meilisearch/MeiliSearch/) for languages using the Arabic alphabet and its variants, similarly to how [`niqqud`](https://github.com/benny-n/niqqud) has been used to [improve search results for Hebrew](https://docs.meilisearch.com/learn/advanced/tokenization.html#deep-dive-the-meilisearch-tokenizer).
