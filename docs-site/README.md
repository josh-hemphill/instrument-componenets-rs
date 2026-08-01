# Docs site (MkDocs Material)

Public documentation for [instrument-components](https://josh-hemphill.github.io/instrument-components-rs/).

## Local serve

From this directory:

```bash
pip install -r requirements.txt && mkdocs serve
```

Then open <http://127.0.0.1:8000>.

## Build

```bash
pip install -r requirements.txt
mkdocs build --strict
```

Output lands in `site/`. CI deploys on push to `latest` via `.github/workflows/docs.yml`.

## Layout

- `docs/` — published pages (Rust | C# tabs on shared guides)
- `snippets/` — reusable Markdown fragments included with `--8<--`
- Agent-oriented notes (roadmap, parity checklist) stay in the repo root `docs/` folder
