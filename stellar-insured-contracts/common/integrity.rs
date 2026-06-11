use soroban_sdk::{Env, Address, Map, BytesN};

/// Run invariant checks to detect corruption or unauthorized modifications
pub fn verify_invariants(
    env: &Env,
    balances: &Map<Address, i128>,
    total_supply: i128,
    escrow_totals: i128,
    deposits_sum: i128,
) -> bool {
    // REVIEW: `env` is currently unused. Consider removing it or renaming
    // it to `_env` to make the intent explicit and avoid warnings.

    // Bounded iteration to prevent unbounded looping and excessive gas usage.
    let mut sum_balances: i128 = 0;
    let mut count: u32 = 0;
    const MAX_ITER: u32 = 100;

    for (_, balance) in balances.iter() {
        if count >= MAX_ITER {
            break;
        }

        // REVIEW: Consider using checked_add() to guard against potential
        // integer overflow when summing large balance values.
        sum_balances += balance;
        count += 1;
    }

    // NOTE:
    // If balances exceed MAX_ITER, this is only a partial validation.
    // REVIEW: For large datasets, consider maintaining aggregate balances
    // during state updates instead of recalculating them during verification.
    // This would provide stronger guarantees and better scalability.

    // REVIEW: Good integrity check. Ensures the recorded total supply matches
    // the sum of tracked account balances.
    if sum_balances != total_supply {
        // REVIEW: Consider returning Result<(), Error> instead of bool
        // so callers can identify which invariant failed.
        return false;
    }

    // Check that escrow totals match deposits.
    // REVIEW: Assumes deposits_sum is accurately maintained elsewhere.
    // Consider documenting how deposits_sum is derived for easier auditing.
    if escrow_totals != deposits_sum {
        return false;
    }

    true
}

/// Verify contract code hash matches expected value
pub fn verify_code_hash(env: &Env, expected_hash: &BytesN<32>) -> bool {
    // REVIEW: Excellent security safeguard. Helps detect unauthorized
    // upgrades, deployment mismatches, or contract tampering.
    let current_hash = env.contract_data().code_hash();

    // REVIEW: Consider logging or emitting an event when verification fails
    // to improve observability and incident response.
    &current_hash == expected_hash
}