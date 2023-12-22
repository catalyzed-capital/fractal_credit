---
name: TransferCredit
summary: "A request to transfer credit to another account at another mutualcredit"
version: 0.0.1
consumers:
    - 'Mutual Credit Account Aggregate'
tags:
    - label: 'command'
externalLinks: []
badges: []
---
Requests the wiring of a specified amount to another account at another mutual credit account. This command can fail to process if the parameters are invalid or if the source account does not have sufficient credit. This will result in the _holding_ of the credit until the credit is completed or cancelled.

<Mermaid />

## Schema
<SchemaViewer />