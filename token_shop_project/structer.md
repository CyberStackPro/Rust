Nice! Let’s plan your **Token Shop v2** roadmap with clear modules/files and a diagram showing how components talk to each other. This will keep your code clean, scalable, and easy to maintain — plus it aligns perfectly with Rust best practices.

---

# Project Expansion Plan — What to Add & Where

| Feature                 | Description                                  | File/Module Suggestion         |
| ----------------------- | -------------------------------------------- | ------------------------------ |
| **User Authentication** | Login system with username & hashed password | `auth.rs`                      |
| **Item Inventory**      | Catalog with item names and prices           | `inventory.rs`                 |
| **Token Management**    | Token balance & top-up logic                 | `tokens.rs`                    |
| **Purchase Logic**      | Buy items, apply discounts, update balances  | `purchase.rs`                  |
| **Transaction History** | Record purchases, top-ups, timestamps        | `history.rs`                   |
| **Custom Errors**       | Define detailed error enums & handling       | `error.rs` (refine existing)   |
| **Main CLI Interface**  | Input/output & command routing               | `main.rs`                      |
| **Tests**               | Unit and integration tests                   | `tests/` folder or `*_test.rs` |

---

# Suggested Folder Structure

```
token_shop_project/
│
├── src/
│   ├── main.rs             # Entry point, CLI interface
│   ├── auth.rs             # User authentication logic
│   ├── inventory.rs        # Item catalog & prices
│   ├── tokens.rs           # Token balance & top-up functions
│   ├── purchase.rs         # Buying logic, discounts, token deduction
│   ├── history.rs          # Transaction log & history management
│   ├── error.rs            # Custom error types & conversions
│   └── cost.rs             # Cost calculation (maybe merged with purchase.rs)
│
├── Cargo.toml
└── README.md
```

---

# High-Level Architecture Diagram

```plaintext
+--------------------+      +-------------------+
|     main.rs        | ---> |     auth.rs       |  <-- User login & validation
| (CLI interface)    |      +-------------------+
|                    |            |
|  - Reads commands  |            |
|  - Routes actions  |            v
|                    |      +-------------------+
|                    | ---> |  inventory.rs     |  <-- Item catalog & prices
|                    |      +-------------------+
|                    |            |
|                    |            v
|                    |      +-------------------+
|                    | ---> |  purchase.rs      |  <-- Handles buying, discounts,
|                    |      |                   |      balance checks, updates tokens
|                    |      +-------------------+
|                    |            |
|                    |            v
|                    |      +-------------------+
|                    | ---> |   tokens.rs       |  <-- Token balance, top-ups
|                    |      +-------------------+
|                    |            |
|                    |            v
|                    |      +-------------------+
|                    | ---> |   history.rs      |  <-- Transaction logs & timestamps
|                    |      +-------------------+
|                    |
|                    | <----------------------------+
|                    |  (Handles errors through error.rs)
+--------------------+
          |
          v
    +-------------+
    |  error.rs   |  <-- Custom error enums & Result types
    +-------------+
```

---

# How Data Flows

- **User runs app (main.rs) → logs in (auth.rs)**
- **Views inventory (inventory.rs)**
- **Chooses item to buy → purchase.rs calculates cost, applies discounts**
- **Checks tokens with tokens.rs**
- **If enough tokens → deducts tokens and updates balance**
- **Transaction details saved in history.rs**
- **Errors anywhere handled via error.rs with meaningful messages**

---

# Next Steps

1. **Start with `auth.rs`**

   - Define user struct
   - Implement login with password hash stub (can be simple at first)

2. **Add `inventory.rs`**

   - Define some items & prices
   - Function to list items

3. **Build `tokens.rs`**

   - Balance & top-up functions

4. **Create `purchase.rs`**

   - Buying, discount logic, token deduction

5. **Implement `history.rs`**

   - In-memory or file-based transaction logging

6. **Refine `error.rs`**

   - Add errors for auth failures, out of stock, etc.

7. **Integrate all in `main.rs`**

   - Route commands, show menus

8. **Write tests for each module**

---

Ready to pick a file to start?
Or want me to scaffold one of these files with comments and function stubs to get you going?
