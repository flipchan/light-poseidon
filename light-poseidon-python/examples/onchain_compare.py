"""
Compare local Poseidon hash against the on-chain contract on Paseo Asset Hub.

Requires: requests (pip install requests)
"""

import requests
from light_poseidon_python import poseidon_hash

CONTRACT = "0x1d165f6fE5A30422E0E2140e91C8A9B800380637"
RPC_URL = "https://paseo-assethub-rpc.laissez-faire.trade"
SELECTOR = "561558fe"


def call_on_chain(a: int, b: int) -> str:
    """Call hash(uint256[2]) on the deployed contract."""
    calldata = f"0x{SELECTOR}{a:064x}{b:064x}"
    payload = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_call",
        "params": [{"to": CONTRACT, "data": calldata}, "latest"],
    }
    resp = requests.post(RPC_URL, json=payload, timeout=30)
    resp.raise_for_status()
    result = resp.json()
    if "error" in result:
        raise RuntimeError(f"RPC error: {result['error']}")
    return result["result"]


def main():
    test_cases = [
        (0, 0),
        (1, 2),
        (123, 456),
        (255, 256),
        (1000000, 999999),
    ]

    print(f"Contract: {CONTRACT}")
    print(f"RPC:      {RPC_URL}")
    print()

    all_pass = True
    for a, b in test_cases:
        local = poseidon_hash([a, b])
        on_chain = call_on_chain(a, b)
        passed = local.lower() == on_chain.lower()
        status = "PASS" if passed else "FAIL"
        if not passed:
            all_pass = False

        print(f"hash([{a}, {b}])")
        print(f"  local:     {local}")
        print(f"  on-chain:  {on_chain}")
        print(f"  {status}")
        print()

    if all_pass:
        print("All tests passed - local matches on-chain!")
    else:
        print("MISMATCH detected!")
        exit(1)


if __name__ == "__main__":
    main()