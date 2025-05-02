# Cabal RPC API Documentation

## Authentication
All API requests must include an authentication header:
```plaintext
"auth": "YOUR_API_KEY"
```
Ensure that this header is present in every request to successfully authenticate with the Cabal RPC service.

## Stream Dependencies
- The `UserActivity` stream **must be opened before** the `Trades` stream.
- Failure to follow this order may result in connection issues or incomplete data streaming.

## Keep-Alive Requirement
- All active streams **must be pinged** at least **once every 10 seconds** to maintain connection stability.
- Use the `TradePing` or `UserPing` RPC methods for this purpose:
  
  ```proto
  rpc TradePing(Ping) returns (Pong);
  rpc UserPing(Ping) returns (Pong);
  ```

## Stream Management Best Practices
- **Do not** close and reopen streams for switching tokens. Instead, use the `SubscribeToken` request.
- This approach minimizes disruptions and improves efficiency in receiving token price updates.

### Switching to a New Token Price Feed
To subscribe to a new token feed, send a `SubscribeToken` request:
```proto
rpc SubscribeToken(TokenTradeEventSub) returns (TradeResponse);
```
- After a new `SubscribeToken` request, you will **first receive**:
  1. `TokenStats` in the `Trades` stream
  2. `UserTokenTradeStats` in the `UserActivity` stream
- These initial messages provide essential details about the newly subscribed token.

## Available RPC Methods
Below are the key RPC methods supported by Cabal RPC:

### User Activity Stream
```proto
rpc UserActivityUni(UserActionSub) returns (stream UserResponse);
```
- Streams user-specific trading actions and responses.
- Must be opened **before** the `Trades` stream.

### Trades Stream
```proto
rpc TradesUni(TradeEventSub) returns (stream TradeEventResponse);
```
- Streams market trade events.
- Can only be opened after the `UserActivity` stream.

### Market Orders
```proto
rpc MarketSell(Sell) returns (TradeResponse);
rpc MarketBuy(Buy) returns (TradeResponse);
```
- Execute immediate buy/sell market orders.

### Limit Orders
```proto
rpc PlaceLimitOrders(orders.TokenLimitOrders) returns (orders.PlaceLimitOrdersResponse);
rpc DeleteLimitOrders(orders.DeleteOrders) returns (orders.PlaceLimitOrdersResponse);
```
- Manage limit orders by placing or deleting them.


## Messages Overview

### Token Subscription Request
```proto
message TokenTradeEventSub {
  string mint = 1;
}
```
- Used in `SubscribeToken` to receive updates for a specific token.

### User Activity Response
```proto
message UserResponse {
  oneof user_response_kind {
    TradeResponse trade_status = 1;
    TokenTradeStats trade_stats = 2;
    txncb.LandedTxnState txn_cb = 3;
    Ping ping = 4;
    Pong pong = 5;
  }
}
```
- Provides feedback on user actions and includes trade statistics.

### Trade Events Response
```proto
message TradeEventResponse {
  oneof trade_event_response_kind {
    TradeEvent trade_event = 1;
    TokenStatus token_status = 2;
    Ping ping = 3;
    Pong pong = 4;
  }
}
```
- Delivers real-time trade event updates.


