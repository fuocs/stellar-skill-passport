# Skill Passport on Stellar

## Project Description

Skill Passport is my Stellar Soroban project for the Vietnam Stellar Unitour. The idea is simple: when a student finishes a course, workshop, or project milestone, a trusted verifier can record that achievement on-chain

I chose this idea because many learning records are still kept in private forms, screenshots, or spreadsheets. With this contract, a learner can have a small public "passport" that shows completed milestones, earned points, and a link to proof such as a GitHub repository or demo

## Project Vision

My vision is to make learning progress easier to prove and reuse. A student should not have to explain the same achievements again and again when joining another bootcamp, hackathon, or student builder group. If the records are stored on Stellar, communities can check them quickly, and learners can keep building their reputation over time

## Key Features

- Admin can create learning milestones with a title, verifier, and point reward
- Only the selected verifier can confirm a learner's completion
- A learner can only complete the same course once, so duplicate credit is blocked
- The contract stores courses, completion proof, total points, and passport summaries
- Anyone can query a course, check whether a learner completed it, or view a learner passport
- Unit tests cover the main success flow and the duplicate-completion case
- The `frontend/` folder has a small UI prototype for the final dapp idea

## Contract Details

Repository: https://github.com/fuocs/stellar-skill-passport

Contract on Stellar Testnet:
https://stellar.expert/explorer/testnet/contract/CACQPBXHOFOIXNVW22Z5XB43D6U3RXC3CRR7ZQQLVR5UMQ3J5Y3RBXKX?filter=history

Contract ID:

```text
CACQPBXHOFOIXNVW22Z5XB43D6U3RXC3CRR7ZQQLVR5UMQ3J5Y3RBXKX
```

WASM hash:

```text
267a1be89fe4cb69d03b3381b570a0a6c2fb52b2839ce81449d0dfb58e903858
```

Testnet deployer:

```text
GCBZQEMRYZJGQVMJ5GI3ORDLT55UL7V4FCLUWUAEANKV5FBOJHMRZZBX
```

Main testnet transactions:

- Upload WASM: https://stellar.expert/explorer/testnet/tx/283e2a7690ad18886b8613e64e47b8b9260ecfe4bc2d8b15c25e665492055738
- Deploy contract: https://stellar.expert/explorer/testnet/tx/373e1a3315db1afaec93f07ec84a3c4877ea03060e7ddd5972dbed45841ac3b6
- `init`: https://stellar.expert/explorer/testnet/tx/f183a237703656a020722c85a4e00963138bfb9e45ab86f3741a002270cbe1a1
- `create_course`: https://stellar.expert/explorer/testnet/tx/52e02e0d4a9dc509ab2468617bb80a0762237a04180c0bc99673d2e33bfadde7
- `verify`: https://stellar.expert/explorer/testnet/tx/5b54b1d2fbf2900b020451de3e10bd98fcbf9767ca6c178649f685a847d60905

The `passport` call was read-only and returned:

```json
{"completed":1,"last_course":1,"learner":"GCBZQEMRYZJGQVMJ5GI3ORDLT55UL7V4FCLUWUAEANKV5FBOJHMRZZBX","points":75}
```

![Contract history on Stellar Expert](contract-history.png)

## Main Functions

| Function | Purpose |
| --- | --- |
| `init(admin)` | Set the first admin account |
| `create_course(title, verifier, reward)` | Add a new learning milestone |
| `set_course_active(course_id, active)` | Pause or reopen a course |
| `verify(learner, course_id, evidence)` | Record that a learner finished a milestone |
| `passport(learner)` | Show a learner's points, completed count, and latest course |
| `has_completed(learner, course_id)` | Check completion status |
| `get_course(course_id)` | Read course information |
| `get_completion(learner, course_id)` | Read the proof for one completion |

## Local Structure

```text
.
|-- Cargo.toml
|-- contracts
|   `-- skill-passport
|       |-- Cargo.toml
|       `-- src
|           |-- lib.rs
|           `-- test.rs
|-- frontend
|   |-- app.js
|   |-- index.html
|   |-- server.mjs
|   `-- styles.css
|-- contract-history.png
|-- frontend-preview.png
|-- info.md
|-- lib.rs
`-- README.md
```

The root `lib.rs` is included because the bootcamp guide asks us to paste code into a `lib.rs` file in Soroban Studio. The full Rust project is inside `contracts/skill-passport/`

## Build and Test

Commands I used to check the contract:

```bash
cargo test
stellar contract build --package skill-passport
```

The test suite currently covers the normal passport flow and the duplicate-completion guard

## Frontend Preview

The frontend is still a prototype, but it shows how the app could feel for a learner or verifier

Run it locally:

```bash
node frontend/server.mjs
```

Then open:

```text
http://127.0.0.1:5173/
```

## Deploy and Invoke

This is the flow I used on Stellar Testnet:

1. Build the contract with Stellar CLI
2. Create and fund a testnet deployer
3. Deploy `target/wasm32v1-none/release/skill_passport.wasm`
4. Invoke in this order:

```text
init(admin = deployer address)
create_course(title = "Soroban Smart Contract Basics", verifier = deployer address, reward = 75)
verify(learner = deployer address, course_id = 1, evidence = "https://github.com/fuocs/stellar-skill-passport")
passport(learner = deployer address)
```

For a better demo, I would use two wallets: one mentor wallet as the `verifier`, and one student wallet as the `learner`

## Future Scope

- Connect the frontend directly to Freighter and Stellar RPC
- Add more verifier roles for clubs, mentors, and partner events
- Add badge levels such as Beginner, Builder, and Hackathon Ready
- Add a way to revoke or dispute a wrong completion record
- Build a small dashboard where people can search learner passports

## About Me

Name: Tran Nguyen Huu Phuoc

I am a student learning how to move from normal web development into web3. Through this project, I practiced Rust smart contracts, Stellar/Soroban basics, GitHub project submission, and a small frontend prototype
