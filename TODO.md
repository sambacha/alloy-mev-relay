# DEVELOPMENT TODO

## NORMAL CASES 
RelayClients initializes correctly with given URLs

bid_manager_receiver subscribes successfully to top bids

tokio::spawn task handles received messages from bid manager

AlloyProvider connects to WebSocket provider successfully

block_stream subscribes to new blocks without errors

New blocks are processed as they come in

relay_clients polls for each new block correctly

Program completes execution without errors

### EDGE CASES

RelayClients initialization fails due to invalid URLs

bid_manager_receiver fails to subscribe to top bids

tokio::spawn task fails to handle received messages

AlloyProvider fails to connect to WebSocket provider

block_stream fails to subscribe to new blocks

New block does not contain a block number

relay_clients fails to poll for new blocks

WebSocket connection drops unexpectedly

Handling of empty or null URLs in RelayClients initialization

Handling of unexpected data format in bid_manager_receiver

Handling of network latency or timeouts during WebSocket connection

Handling of duplicate block numbers in block_stream