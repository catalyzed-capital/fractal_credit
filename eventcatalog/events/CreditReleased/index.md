---
name: CreditReleased
summary: "Indicates that reserved credit were released"
version: 0.0.1
consumers:
    - 'Mutual Credit Account Aggregate'
    - 'Credit Transfer Process Manager'
    - 'Mutual Credit Account Projector'
producers:
    - 'Mutual Credit Account Aggregate'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
Indicates that held credit were released as part of a failed or canceled transfer.

<Mermaid />

## Schema
<SchemaViewer />