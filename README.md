# dendryform

> Declarative software architecture diagrams — beautiful, dark-themed, with a simple schema.

Named for the 23 [dendriform models](https://en.wikipedia.org/wiki/Plant_architecture) of tree architecture (Hallé & Oldeman, 1970), because every system has a branching pattern worth revealing.

## Status

🌱 **Early development** — schema design and HTML renderer in progress.

## What This Will Be

`dendryform` takes a declarative description of a software system — nodes, edges, containment, tiers — and renders it as a beautiful, interactive HTML architecture diagram.

```yaml
title: "myproject · system architecture"
theme: dark

tiers:
  - label: Client Layer
    layout: single
    nodes:
      - id: web-app
        label: Web Application
        kind: system
        tech: [React, TypeScript]
```

```bash
dendryform render architecture.yaml -o architecture.html
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
