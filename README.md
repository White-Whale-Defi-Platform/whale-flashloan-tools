# whale-flashloan-tools

This repository contains a bunch of different assets to enable flash loans on Terra as a composable piece in a grander Defi strategy.

Initially starting with just one flash loan starter package the hope is this repo will grow with community input to encompass many different open-source tools to enable cashflow generation using flash loan capability as a source of liquidity.

We have other contracts and script internally which aren't quite ready for public consumption yet so as time goes on the team will contribute more of our internal thought up strategies for the various protocols.

### What are Flashloans

Flash Loans allow you to borrow any available amount of assets without putting up any collateral, as long as the liquidity is returned to the protocol within one block transaction. - [Source: AAVE](https://docs.aave.com/faq/flash-loans)

### Contract resources

At the time of writing these are the relevant contracts for flash loans that should be called to request funds on each network: 

# Columbus 5
| Contract Name | Address | Description |
| :--- | :--- | :--- |
| UST Vault /w Flash Loans | `terra1ec3r2esp9cqekqqvn0wd6nwrjslnwxm7fh8egy` | Address of the UST Stablecoin Vault with Flash Loan capabilities |

# Bombay-12

| Contract Name | Address | Description |
| :--- | :--- | :--- |
| UST Vault /w Flash Loans | `terra1zljypewdglfl5f6ntfl2r3epgxjmzh05qnjknv` | Address of the UST Stablecoin Vault with Flash Loan capabilities |


It is advised to do strategy testing first on bombay to protect gas spending but with the right denom setting this can be rather low. 
### Acknowledgements 

A particular massive shout-out to CyberHoward and Astromartian for their internal contributions on flash loans. Both could easily be considered thought leaders in addition to the rest of the Smart Contract team.