---
name: CreditTransferInitiated
summary: "Indicates that a credit transfer process has begun"
version: 0.0.1
consumers:    
    - 'Mutual Credit Account Projector'
    - 'Credit Transfer Process Manager'
    - 'Mutual Credit Account Aggregate'
producers:
    - 'Mutual Credit Account Aggregate'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
Indicates that the **process** of a credit transfer has been initiated. External stimuli from a gateway can then emit events to indicate the completion (successful or otherwise) of this process. Credit involved in the transfer are _reserved_ from the account, but not yet _withdrawn_. The credit will either be released or fully withdrawn pending the outcome of the transfer.

<Mermaid />

## Schema
<SchemaViewer />