# Cryptocurrencies

[![cat~cryptography::cryptocurrencies][cat~cryptography::cryptocurrencies~badge]][cat~cryptography::cryptocurrencies]{{hi:Cryptocurrencies}}

Cryptocurrencies are digital or virtual currencies designed to work as a medium of exchange. They use cryptography to secure and verify transactions as well as to control the creation of new units of a particular cryptocurrency. Unlike traditional currencies issued by central banks, cryptocurrencies operate independently of any central authority, relying on a decentralized system.

Cryptocurrencies are typically based on a decentralized network, meaning no single entity controls them. This is often achieved through blockchain technology. A blockchain is a distributed, immutable ledger that records transactions in "blocks" that are linked together chronologically. This creates a transparent and secure record of all transactions.

Many (but not all) cryptocurrencies use a process called "mining" to verify transactions and add them to the blockchain. Miners use powerful computers to solve complex mathematical problems, and the first to solve the problem gets to add the next block to the chain and is rewarded with newly created cryptocurrency.

Rust has become popular in the cryptocurrency space, due to its performance, security, and developer-friendly features.

BEWARE: Be wary of scams.

- Cryptocurrency prices are notoriously volatile, experiencing dramatic swings in value in short periods. This makes them highly speculative "investments", with the potential for significant losses.
- The cryptocurrency space attracts scammers, who may try to defraud users through phishing, fake ICOs, or other schemes.
- The relatively unregulated nature of cryptocurrency markets makes them susceptible to manipulation, such as "pump and dump" schemes.
- Cryptocurrency exchanges and wallets can be vulnerable to hacking attempts.

## Cryptocurrencies

{{#include cryptocurrencies.incl.md}}

- Bitcoin: [`bitcoin`][c~bitcoin~docs]↗{{hi:bitcoin}}.
- Ethereum: [`web3`][c~web3~docs]↗{{hi:web3}} (for interacting with Ethereum), [`ethers`][c~ethers~docs]↗{{hi:ethers}}.
- Other Cryptocurrencies: many cryptocurrencies have their own Rust libraries.
- Wallet Development: often involves a combination of the above crates and custom logic.
- Smart Contracts: primarily a [`Solidity`][soliditylang~website]↗{{hi:Solidity}} ecosystem, but Rust may be used for [testing][p~testing] or interacting with contracts via crates like [`ethers`][c~ethers~docs]↗{{hi:ethers}}.

## Related Topics

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[cryptocurrencies: write](https://github.com/john-cd/rust_howto/issues/278)
</div>
