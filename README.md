# anything

[<img alt="github" src="https://img.shields.io/badge/github-udoprog/anything-8da0cb?style=for-the-badge&logo=github" height="26">](https://github.com/udoprog/anything)
[<img alt="crates.io" src="https://img.shields.io/crates/v/anything.svg?style=for-the-badge&color=fc8d62&logo=rust" height="26">](https://crates.io/crates/anything)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-anything-66c2a5?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="26">](https://docs.rs/anything)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/udoprog/anything/CI/master?style=for-the-badge" height="26">](https://github.com/udoprog/anything/actions?query=branch%3Amaster)

Calculate everything and nothing with perfect precision.

Anything provides the `any` binary, which is a small flexible program
intended to do calculations and query for numerical facts. Among other
things it can do:

* Unit conversions `3floz to dl` (this is mostly why I use this).
* Fact queries `population finland / population world`.
* Basic math `100l / round(population finland)`.
* Unit-aware calculations `3N / 10kg` gives us `0.3 m/s²`.

You can thing of `any` is a much simpler and local wolfram engine. Which I'm
hoping to extend it as I go along.

License: MIT/Apache-2.0
