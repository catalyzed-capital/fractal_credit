---
name: Credit Transfer Process Manager
summary: |
  The process manager for managing credit transfer processes
tags:
    - label: 'procman'
---

This process manager is responsible for managing the process of credit transfers. It listens for the `CreditTransferInitiated` event and then emits the appropriate commands to continue the process

<Mermaid/>

## Interest
The following indicates the sequential flow of the process manager's interest, which is required for defining link definitions. It's important to note that the process doesn't complete when it receives the fail/succeed events from the outside world. The process is only considered completed when the credit held by the credit transfer are released or committed.

* `start` - [CreditTransferInitiated](../../events/CreditTransferInitiated)
* `advance` - [CreditReserved](../../events/CreditReserved), [CreditTransferSucceeded](../../events/CreditTransferSucceeded), [CreditTransferFailed](../../events/CreditTransferFailed)
* `end` - [CreditCommitted](../../events/CreditCommitted), [CreditReleased](../../events/CreditReleased)