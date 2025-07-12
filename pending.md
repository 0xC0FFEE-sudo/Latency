### **Project: Latency-X - Implementation and Finalization Plan**

### 1. Project Overview

`Latency-X` is a high-frequency trading application designed for cryptocurrency markets. Its architecture includes:

*   **Core Logic (`latency-x-core`):** A Rust-based backend that handles connections to exchanges (like Binance and Kraken), implements various trading strategies, and manages data flow.
*   **Connectors:** Modules for integrating with different exchange APIs via WebSockets to receive real-time market data.
*   **Strategies:** Pluggable trading algorithms (`arbitrage`, `market_maker`, etc.).
*   **Dashboard:** A web-based frontend (likely using a framework like Svelte or React, powered by Axum on the backend) for monitoring, logging, and data visualization.

### 2. Current Status

The project is currently in a stabilization phase. A major effort was undertaken to refactor dependencies and fix a large number of resulting compilation errors. The backend logging framework (`tracing`) has been instrumented.

However, the application **does not yet compile**. Several errors remain within the trading strategy modules. The immediate priority is to achieve a stable, compilable `main` branch.

### 3. Phase 1: Code Compilation and Stabilization

This is the highest priority. The goal is to resolve all remaining compilation errors. The errors are concentrated in the strategy files, which are still using outdated data structures and function signatures.

The core issue is the incorrect creation of `Order` objects. The `Order` struct in `latency-x-core/src/models.rs` has been updated, and the strategy files need to be brought into compliance.

**Key changes to be aware of:**
*   The `Order` struct now uses a field named `amount` instead of `quantity`.
*   A constructor `Order::market(side: OrderSide, amount: f64)` is available to create market orders.

Here is the file-by-file plan to fix the codebase:

#### 3.1. `latency-x-core/src/strategies/arbitrage.rs`

*   **Task:** Update `Order` creation to use the correct field names and constructor.
*   **Implementation:**

```rust:latency-x-core/src/strategies/arbitrage.rs
// ... existing code ...
        let order_leg1 = Order {
            order_type: OrderType::Market,
            side: OrderSide::Buy,
            amount: 1.0, //
            price: None,
        };

        let order_leg2 = Order {
            order_type: OrderType::Market,
            side: OrderSide::Sell,
            amount: 1.0,
            price: None,
        };

        (order_leg1, order_leg2)
    }
}
// ... existing code ...
```

#### 3.2. `latency-x-core/src/strategies/buy_new_token.rs`

*   **Task:** Update `Order` creation to use the `amount` field.
*   **Implementation:**

```rust:latency-x-core/src/strategies/buy_new_token.rs
// ... existing code ...
    fn on_fill(&mut self, fill: Fill) {
        if fill.source == self.source {
            let order = Order {
                order_type: OrderType::Market,
                side: OrderSide::Buy,
                amount: 1.0, // Use 'amount' instead of 'quantity'
                price: None,
            };
            self.sender.send(DashboardEvent::Trade(Trade {
// ... existing code ...
```

#### 3.3. `latency-x-core/src/strategies/market_maker.rs`

*   **Task:** Update `Order` creation to use the `amount` field.
*   **Implementation:**

```rust:latency-x-core/src/strategies/market_maker.rs
// ... existing code ...
    fn create_orders(&self) -> (Order, Order) {
        let order_buy = Order {
            order_type: OrderType::Limit,
            side: OrderSide::Buy,
            amount: 1.0, // Use 'amount' instead of 'quantity'
            price: Some(99.0),
        };

        let order_sell = Order {
            order_type: OrderType::Limit,
            side: OrderSide::Sell,
            amount: 1.0, // Use 'amount' instead of 'quantity'
            price: Some(101.0),
        };

        (order_buy, order_sell)
// ... existing code ...
```

#### 3.4. `latency-x-core/src/strategies/mev.rs`

*   **Task:** Update the `Order::market` call to match the correct function signature.
*   **Implementation:**

```rust:latency-x-core/src/strategies/mev.rs
// ... existing code ...
    fn on_trade(&mut self, trade: Trade) {
        if trade.source == self.source {
            // TODO: implement MEV logic
            // For now, just buy 1.0 of the token
            let order = Order::market(OrderSide::Buy, 1.0);
            self.sender.send(DashboardEvent::Trade(Trade {
                source: self.source.clone(),
                side: order.side.clone(),
// ... existing code ...
```

After applying these changes, run `cargo check --workspace` from the root directory to confirm that all compilation errors are resolved.

### 4. Phase 2: High-Level Feature Implementation

Once the codebase is stable, work can begin on the features outlined in the project's TODO list.

*   **`pending (frontend_logging)`**: Implement the Logs page to display real-time logs from the backend.
    *   **Task:** The backend websocket server at `src/dashboard/server.rs` already broadcasts logs. The frontend needs to establish a WebSocket connection to the `/ws/logs` endpoint and display the incoming `LogEntry` JSON objects in a user-friendly format (e.g., a scrolling, filterable table).
*   **`pending (backend_persistence)`**: Add a SQLite database to persist trades and other events.
    *   **Task:** Integrate `sqlx` with the `sqlite` feature.
    *   **Implementation:**
        1.  Add `sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }` to `latency-x-core/Cargo.toml`.
        2.  Create a database manager (e.g., `src/db.rs`).
        3.  Define a database schema for `trades`, `fills`, and `orders`.
        4.  In `main.rs`, initialize the database connection and pass it to the components that need it (e.g., the main event loop).
        5.  Modify the main event loop to write trade and fill events to the database.
*   **`pending (frontend_historical_data)`**: Enhance the Trades page to show historical data.
    *   **Task:** This depends on backend persistence.
    *   **Implementation:**
        1.  Create a new Axum route in the backend (e.g., `GET /api/trades`) that queries the SQLite database for historical trades.
        2.  The frontend will call this endpoint upon loading the "Trades" page to populate a historical data table. Implement features like pagination and filtering.
*   **`pending (frontend_settings_theme)`**: Implement theme switching (light/dark) in the Settings page.
    *   **Task:** This is a pure frontend task.
    *   **Implementation:** Use CSS variables for colors. A toggle in the UI will switch a class on the root HTML element (e.g., `data-theme="dark"`), which will apply a different set of color variables. Store the user's preference in `localStorage`.

### 5. How to Leverage the Development Assistant

The AI assistant has a powerful set of tools to accelerate development. Here is how to use them effectively:

*   **Reading and Understanding Code (`read_file`):** When you encounter an error or need to understand a module, ask the assistant to read the relevant file. For example: *"Read the contents of `latency-x-core/src/models.rs` so I can see the `Order` struct definition."*
*   **Searching for Code (`grep_search`):** To find where a function is used or a struct is defined, use a search. For example: *"Search for all usages of `Order::market` in the `src/strategies` directory."*
*   **Making Changes (`edit_file`):** For implementing the changes described in this plan, you can instruct the assistant directly. For example: *"Apply the fix for `arbitrage.rs` as described in the plan."* The assistant can generate and apply the code diffs for you.
*   **Finding Files (`file_search`):** If you are unsure of a file's exact location, ask the assistant to find it. For example: *"Find the `Cargo.toml` file for the core project."*
*   **External Knowledge (`web_search`):** For questions about dependencies, Rust language features, or best practices, the assistant can search the web. For example: *"Search for the best way to handle SQLite database connections in an Axum application using `sqlx`."*
*   **Using Pull Request Information (`fetch_pull_request`):** If the project used GitHub and had pull requests, this tool would be invaluable for understanding the history and context of large changes. For this project, since it is being developed locally, this tool might be less relevant unless you start using a git hosting service.

By following this plan and leveraging the assistant's capabilities, you can efficiently bring `Latency-X` to a stable, feature-complete state. 