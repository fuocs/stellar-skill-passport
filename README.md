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

Contract link: I prepared the contract and repository for Soroban Studio. After the testnet deployment is signed with Freighter, this section should be updated with the Stellar Expert contract URL

Example format:

`https://stellar.expert/explorer/testnet/contract/YOUR_CONTRACT_ID?filter=history`

Deployment screenshot checklist:

- The contract deployment transaction
- One successful invoke transaction, such as `verify` or `passport`

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
|-- info.md
|-- lib.rs
`-- README.md
```

The root `lib.rs` is included because the bootcamp guide asks us to paste code into a `lib.rs` file in Soroban Studio. The full Rust project is inside `contracts/skill-passport/`

## Build and Test

If Stellar CLI and Rust are installed:

```bash
cargo test
stellar contract build --package skill-passport
```

If the local Stellar/Rust setup is not installed yet, the same contract can still be tested through Soroban Studio

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

## Deploy and Invoke on Testnet

1. Open Soroban Studio: https://soroban.studio/
2. Connect GitHub and Freighter
3. Switch Freighter to Stellar Testnet and fund the wallet
4. Paste the root `lib.rs` content into the Studio contract file
5. Build the contract
6. Deploy to testnet
7. Invoke in this order:

```text
init(admin = your Freighter testnet address)
create_course(title = "Soroban Smart Contract Basics", verifier = your Freighter testnet address, reward = 75)
verify(learner = your Freighter testnet address, course_id = 1, evidence = "https://github.com/fuocs/stellar-skill-passport")
passport(learner = your Freighter testnet address)
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
