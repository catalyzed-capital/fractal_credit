---
name: CreditCommitted
summary: "Indicates that reserved credit were committed and withdrawn"
version: 0.0.1
consumers:
    - 'Mutual Credit Account Aggregate'
    - 'Credit Transfer Process Manager'
producers:
    - 'Mutual Credit Account Aggregate'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
Indicates that previously held credit were withdrawn from the account. In the interest of simplicity, this example doesn't support partially committed credit or credit that are required to clear in increments of some small value.

<Mermaid />

## Schema
<SchemaViewer />