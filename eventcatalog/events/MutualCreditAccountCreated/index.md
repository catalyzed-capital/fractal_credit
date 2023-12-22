---
name: AccountCreated
summary: "Indicates the creation of a new mutualcredit account"
version: 0.0.1
consumers:
    - 'Mutual Credit Account Aggregate'
    - 'Mutual Credit Account Projector'
producers:
    - 'Mutual Credit Account Aggregate'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
Indicates that a mutualcredit account has been created. As with all events, this is immutable truth.

<Mermaid />

## Schema
<SchemaViewer />