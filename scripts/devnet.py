# /// script
# requires-python = ">=3.11,<3.14"
# dependencies = ["starknet-py==0.30.0"]
# ///
"""Déclare et vérifie uniquement sur un devnet isolé que ce script démarre lui-même."""
import asyncio
import json
import pathlib
import shutil
import socket
import subprocess
import tempfile
import time
import urllib.request

from starknet_py.common import create_sierra_compiled_contract
from starknet_py.contract import Contract
from starknet_py.hash.sierra_class_hash import compute_sierra_class_hash
from starknet_py.net.account.account import Account
from starknet_py.net.client_models import Call
from starknet_py.net.client_errors import ClientError
from starknet_py.net.full_node_client import FullNodeClient
from starknet_py.net.signer.key_pair import KeyPair
from starknet_py.hash.selector import get_selector_from_name

ROOT = pathlib.Path(__file__).resolve().parents[1]


def artifacts(directory, name):
    sierra_path = next(directory.glob(f"*_{name}.contract_class.json"))
    casm_path = sierra_path.with_name(sierra_path.name.replace(".contract_class", ".compiled_contract_class"))
    sierra = sierra_path.read_text()
    casm = json.loads(casm_path.read_text())
    # Les hints ne participent pas au hash; le parseur rejette certains immédiats.
    casm["hints"] = []
    return sierra, json.dumps(casm)


def check_hashes():
    config = json.loads((ROOT / "integration.json").read_text())
    directory = ROOT / "contracts/verifier/target/release"
    for name, key in [("SP1Groth16VerifierBN254", "sp1_verifier_class_hash"), ("UniversalECIP", "ecip_class_hash")]:
        sierra, _ = artifacts(directory, name)
        actual = compute_sierra_class_hash(create_sierra_compiled_contract(sierra))
        assert actual == int(config[key], 16), (name, hex(actual), config[key])
        print(f"{name}: {hex(actual)}", flush=True)


async def verify(url):
    request = urllib.request.Request(url + "/rpc", data=json.dumps({
        "jsonrpc": "2.0", "id": 1, "method": "devnet_getPredeployedAccounts", "params": {},
    }).encode(), headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(request, timeout=5) as response:
        details = json.load(response)["result"][0]
    client = FullNodeClient(node_url=url + "/rpc")
    account = Account(
        address=details["address"], client=client,
        key_pair=KeyPair.from_private_key(int(details["private_key"], 16)),
        chain=int(await client.get_chain_id(), 16),
    )
    declarations = {}
    for name, directory in [
        ("UniversalECIP", ROOT / "contracts/verifier/target/release"),
        ("SP1Groth16VerifierBN254", ROOT / "contracts/verifier/target/release"),
        ("HelloStarknet", ROOT / "contracts/target/release"),
    ]:
        sierra, casm = artifacts(directory, name)
        declaration = await Contract.declare_v3(
            account=account, compiled_contract=sierra, compiled_contract_casm=casm, auto_estimate=True,
        )
        await declaration.wait_for_acceptance()
        declarations[name] = declaration
        print(f"Declared {name}: {hex(declaration.class_hash)}", flush=True)
    fixture = json.loads((ROOT / "contracts/src/fixtures/groth16-fixture.json").read_text())
    deployed = await declarations["HelloStarknet"].deploy_v3(
        constructor_args={
            "verifier": declarations["SP1Groth16VerifierBN254"].class_hash,
            "program_vkey": int(fixture["vkey"], 16),
        }, auto_estimate=True,
    )
    await deployed.wait_for_acceptance()
    calldata = [int(word, 16) for word in (ROOT / "contracts/src/fixtures/groth16-calldata.txt").read_text().split()]
    result = await client.call_contract(Call(
        to_addr=deployed.deployed_contract.address,
        selector=get_selector_from_name("verify_sp1_proof"), calldata=[len(calldata), *calldata],
    ))
    assert result == [0, 3, 3, 0, 2, 0, 3, 0], result
    print("Devnet application verified the stored receipt and returned (3, 2, 3)", flush=True)
    changed = calldata.copy()
    changed[48] = 4
    try:
        rejected = await client.call_contract(Call(
            to_addr=deployed.deployed_contract.address,
            selector=get_selector_from_name("verify_sp1_proof"), calldata=[len(changed), *changed],
        ))
    except ClientError as error:
        assert error.code == 40, error
    else:
        assert rejected[0] == 1, rejected
    print("Devnet application rejected changed public values", flush=True)


def main():
    for manifest in ["contracts/Scarb.toml", "contracts/verifier/Scarb.toml"]:
        subprocess.run(["scarb", "--manifest-path", str(ROOT / manifest), "--release", "build"], check=True)
    check_hashes()
    binary = shutil.which("starknet-devnet")
    assert binary, "Install starknet-devnet 0.10.1 and add it to PATH"
    version = subprocess.check_output([binary, "--version"], text=True)
    assert version.strip() == "starknet-devnet 0.10.1", version
    with socket.socket() as listener:
        listener.bind(("127.0.0.1", 0))
        port = listener.getsockname()[1]
    url = f"http://127.0.0.1:{port}"
    with tempfile.TemporaryFile(mode="w+") as log:
        process = subprocess.Popen([
            binary, "--host", "127.0.0.1", "--port", str(port), "--seed", "1", "--accounts", "1",
            "--initial-balance", "1000000000000000000000000000000",
        ], stdout=log, stderr=subprocess.STDOUT)
        try:
            for _ in range(150):
                try:
                    with urllib.request.urlopen(url + "/is_alive", timeout=1):
                        break
                except OSError:
                    if process.poll() is not None:
                        raise RuntimeError("Devnet exited before startup")
                    time.sleep(0.2)
            else:
                raise RuntimeError("Devnet startup timed out")
            asyncio.run(verify(url))
        except BaseException:
            log.seek(0)
            print(log.read())
            raise
        finally:
            process.terminate()
            try:
                process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()


if __name__ == "__main__":
    main()
