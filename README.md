## Dark Trades: The Zero-MEV TEE Dark Pool on Solana
A high-frequency, privacy-first Decentralized Exchange powered by MagicBlock Private Ephemeral Rollups (PERs) and Intel TDX.

### The Problem: On-Chain Intent Leakage & Leader-Level MEV
While Solana does not have a traditional mempool—instead utilizing Gulf Stream to forward transactions directly to block leaders—the problem of value extraction remains. Standard Decentralized Exchanges (DEXs) rely on transparent order books or public AMM curves. When a user places a large resting limit order, it sits publicly on the base layer (L1). This guarantees two negative outcomes:

Jito & Leader-Level MEV: Searchers monitoring L1 state changes see the order, bundle transactions to manipulate the price, and extract arbitrage value via blockspace auctions before the user's order is naturally filled.

Information Leakage: Institutional or whale traders broadcast their market intent to the entire network, artificially shifting asset prices against themselves before their trade even executes.

### 💡 The Solution: Shielded Intents & Ephemeral Execution
Dark Trades is a true Dark Pool built natively on Solana. By leveraging MagicBlock's Private Ephemeral Rollups (PERs) backed by Trusted Execution Environments (TEEs), Dark Trades moves the order-matching engine entirely off-chain while maintaining L1 cryptographic settlement.

### 🏗 Dark Trades Architecture: 
The L1 $\to$ TEE $\to$ L1 LifecycleDark Trades is designed around a strict, atomically secure lifecycle.

🔐 1. Permission & Delegation First (Security Boundary)Before any trading begins, a secure boundary is established. Permission accounts are created for the DepositAccount PDAs, IntentAccount PDAs, and Vault Token Accounts (USDC ATAs owned by the DepositAccount). These are immediately delegated to the MagicBlock PER.The Result: Solana L1 transfers mutation rights to the MagicBlock TEE. Accounts are empty and intents are hidden, but the execution environment is secured.

📝 2. place_intent: Atomic Setup + DelegationWhen a user places an intent, the following happens in a single, atomic transaction:Create Deposit PDA.Create Vault ATA.Create Intent PDA.Deposit assets.Delegate All to TEE.Why this matters: There is never a moment where a funded order sits publicly on L1 awaiting matching. Funds move directly into TEE custody, preventing any public orderbook from forming.

🧠 3. Off-Chain DiscoveryThe matcher bot observes delegated IntentAccounts and matches compatible buy/sell intents off-chain, determining the midpoint price using a Pyth oracle. This stage is purely off-chain discovery; no execution happens yet, meaning zero MEV surface.

🔴 4. match_orders: Private Execution in TEEThe matcher bot calls match_orders, executing exclusively inside the MagicBlock PER.Inside the TEE, the program validates intents, expiry, and price crossing. Then, the magic happens:SOL is moved between Deposit PDAs (direct lamport mutation).USDC balances are updated in vault token accounts.IntentAccount.is_matched is updated.All mutations occur without CPIs and without emitting token transfer logs. No searcher can observe the matching price logic, execution sequence, or vault rebalancing.

🟣 5. settle_and_undelegate: Commit Back to L1After matching is complete (is_matched == true), the matcher bot triggers settlement. This instruction commits the modified state to Solana L1, removes TEE permissions, and undelegates the PDAs. Control is returned to the base layer with the trade completed.

💰 6. Withdrawal Phase (L1)Once undelegated, users can withdraw SOL and USDC from their respective PDAs and close the intent/deposit accounts. All settlement is now transparent and final. No private state remains.

### 🔁 Full Lifecycle FlowPlaintext
```
User → place_intent
       ├── Create Deposit PDA
       ├── Create Vault ATA
       ├── Create Intent PDA
       └── Delegate to PER

Matcher → match_orders (Inside TEE)
           ├── Validate intents & Compute trade
           ├── Move SOL (lamport mutation)
           ├── Move USDC (vault byte mutation)
           └── Mark intent matched

Matcher → settle_and_undelegate
           ├── Commit state to L1
           ├── Remove permission
           └── Undelegate accounts

User → withdraw
       └── Assets return to user wallet
```

### 🛡 Why This Design is Truly Zero-MEVFeature
Traditional Solana DEXDark Trades (PER + TEE)Order VisibilityPublicly visible resting liquidityDelegated & shielded inside TEEPrice ExecutionVisible AMM curves / OrderbooksPrivate endpoint crossingMEV VulnerabilityHigh (Frontrunning, Sandwiching)Zero (No transaction bundle surface)Searchers cannot frontrun what they cannot see. Block leaders cannot reorder what never touches L1 until final settlement.

### ⚙️ Technical Properties & Trust Model
Technical Highlights:
- PDA-owned vault architecture.
- Direct lamport mutation inside the TEE.
- Direct token balance mutation (Zero CPI overhead).
- Deterministic PDA seeds for deposits and intents.
- Permission-based mutation control.

### Trust & Privacy Model:
Dark Trades relies on the MagicBlock PER and Intel TDX attested execution. Privacy is derived from hardware isolation, not cryptographic obfuscation. However, settlement remains deterministic and fully verifiable on the Solana L1.

### 🎯 Final Positioning
Dark Trades is not a standard on-chain orderbook. It is not an AMM. It is not a zk-Rollup. It is a TEE-native, zero-MEV, institutional-grade dark pool built directly on Solana.