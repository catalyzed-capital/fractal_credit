---
name: CreditReserved
summary: "Indicates credit have been placed on hold for a credit transfer"
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
Indicates that the credit to be used in a credit transfer have been reserved/placed on hold.

<Mermaid />

## Schema
<SchemaViewer />