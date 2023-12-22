---
name: CreditTransferFailed
summary: "Indicates that a credit transfer process failed"
version: 0.0.1
consumers: 
    - 'Credit Transfer Process Manager'
tags:
    - label: 'event'
    - label: 'external'
externalLinks: []
badges: []
---
This event is published from an external source to indicate that the credit transfer process failed.
Note that this event doesn't have any internal information like entity ID because it originates from outside the system.

<Mermaid />

## Schema
<SchemaViewer />