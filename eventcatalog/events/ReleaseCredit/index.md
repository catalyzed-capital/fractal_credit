---
name: ReleaseCredit
summary: "A request to release credit from a credit transfer"
version: 0.0.1
consumers:
    - 'Mutual Credit Account Aggregate'
producers:
    - 'Credit Transfer Process Manager'
tags:
    - label: 'command'
externalLinks: []
badges: []
---
Requests that credit held for a given credit transfer are to be released. Note that this command can be rejected if no such 
credit transfer is known to the aggregate.

<Mermaid />

## Schema
<SchemaViewer />