const connectBtn = document.querySelector("#connectBtn");
const form = document.querySelector("#verifyForm");
const learnerInput = document.querySelector("#learner");
const courseInput = document.querySelector("#courseId");
const evidenceInput = document.querySelector("#evidence");
const preview = document.querySelector("#invokePreview");

const CONTRACT_ID = "CACQPBXHOFOIXNVW22Z5XB43D6U3RXC3CRR7ZQQLVR5UMQ3J5Y3RBXKX";
const RPC_URL = "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE = "Test SDF Network ; September 2015";

let walletAddress = "";

async function loadStellarTools() {
  const [stellar, freighter] = await Promise.all([
    import("https://esm.sh/@stellar/stellar-sdk@13"),
    import("https://esm.sh/@stellar/freighter-api@4"),
  ]);

  return { stellar, freighter };
}

function readAccessAddress(accessResult) {
  if (typeof accessResult === "string") {
    return accessResult;
  }

  return accessResult?.address ?? accessResult?.publicKey ?? "";
}

connectBtn.addEventListener("click", async () => {
  connectBtn.textContent = "Connecting...";

  try {
    const { freighter, stellar } = await loadStellarTools();
    const connection = await freighter.isConnected();
    if (connection?.error || connection === false) {
      throw new Error("Freighter is not available");
    }

    const access = await freighter.requestAccess();
    walletAddress = readAccessAddress(access);
    if (!walletAddress) {
      throw new Error("Wallet access was not approved");
    }

    const server = new stellar.rpc.Server(RPC_URL);
    await server.getLatestLedger();
    connectBtn.textContent = `${walletAddress.slice(0, 6)}...${walletAddress.slice(-4)}`;
  } catch (error) {
    walletAddress = "";
    connectBtn.textContent = "Freighter not connected";
    preview.textContent = `Connect wallet failed

${error.message}`;
  }
});

form.addEventListener("submit", (event) => {
  event.preventDefault();

  const learner = learnerInput.value.trim() || walletAddress || "G...";
  const courseId = courseInput.value || "1";
  const evidence = evidenceInput.value.trim() || "https://github.com/...";

  preview.textContent = `verify(
  contract = "${CONTRACT_ID}",
  learner = "${learner}",
  course_id = ${courseId},
  evidence = "${evidence}",
  network = "${NETWORK_PASSPHRASE}"
)

Stellar SDK call:
new Contract(CONTRACT_ID).call("verify", learner, course_id, evidence)`;
});
