# Mutual Credit Account Aggregate
This aggregate represents the sum of events on the `mutualcreditaccount` stream, which is keyed by the MCA Id on the commands and events in this logical stream.

# Configuration
The following configuration values should be set for this aggregate to work properly.
* `ROLE` - `aggregate`
* `INTEREST` - `mutualcreditaccount`
* `NAME` - `mutualcreditaccount`
* `KEY` - `account_id`

# Manual Testing
You can send the following commands manually to watch the aggregate perform its tasks:

## Creating an Account
You can use the following `nats req` command (edit the data as you see fit) to create a new account by submitting a new `create_mca` command:
```
nats req cc.commands.mutualcreditaccount '{"command_type": "create_account", "key": "ABC123", "data": {"account_number": "ABC123", "initial_balance": 4000, "min_balance": 100, "entity_id": "CUSTBOB"}}'
```
You should receive a reply that looks something like this:
```
11:25:05 Sending request on "cc.commands.mutualcreditaccount"
11:25:05 Received with rtt 281.083µs
{"stream":"CC_COMMANDS", "seq":2}
```

And now you can verify that you have indeed created the `ABC123` account (note the key is account number and not entity ID).
```
nats kv get CC_STATE agg.mutualcreditaccount.ABC123
CC_STATE > agg.mutualcreditaccount.ABC123 created @ 20 Mar 23 15:25 UTC

{"balance":4000,"min_balance":100,"account_number":"ABC123"}
```

