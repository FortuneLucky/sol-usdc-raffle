# Get Start

##Build Contract

  anchor build

## Deploy Contract

  anchor deploy

## Test Contract

  anchor run test

## Contract Description

### Ticket Purchase:

Each ticket costs $1 USDC.
Users can buy as many tickets as they want and assign them to specific lotteries.
Upon each ticket purchase, the USDC is immediately distributed to the proper wallets based on the following percentages:
5% goes to the smart contract wallet if a referrer is set, otherwise 10% goes to the smart contract wallet.
If a referrer is set, they receive 5% of the ticket purchase.
The remaining amount (90%) stays in the raffle pool.

### Automatic raffle:

The initial raffle starts with a prize of $69 USDC.
Once the number of tickets purchased is 20% greater than the prize amount, a random winner is selected.
After a winner is selected, a new raffle is automatically generated with a prize amount 1.3X higher than the previous one. (It does this forever)

### Create Raffles:

Admins can create raffles.
Admins can set the ticket value, max tickets, and the USDC prize amount for the raffle.

### Referral System:

If a referrer's wallet address is provided during ticket purchase, they receive 5% of the ticket value (half of the 10% to smart contract).

### Raffle Information:

For each raffle, display the following information:
Total value of the prize pool
Total number of tickets bought
Total number of tickets remaining before the raffle/raffle ends
Total Tickets Purchased By The Wallet with the Ticket IDs
Automatic raffle Recreation (On / Off) - Determines if the raffle recreates at 1.3x after completion or ends after final distribution.

### Fund Distribution:

With every ticket purchase transaction, the USDC should be automatically distributed to the proper wallets based on the defined percentages.
The smart contract should keep track of the balances and ensure accurate distribution.
The random winner is selected and sent the USDC when the max tickets value is reached.

### Ticket Purchase Distribution:

#### If referrer is set:

Smart contract admin wallet receives 5% of the ticket value
Referrer wallet receives 5% of the ticket value
Remaining 90% stays in the raffle pool

#### If no referrer:

Smart contract admin wallet receives 10% of the ticket value
Remaining 90% stays in the raffle pool

### Automatic raffle Winner Selection:
When the number of tickets purchased is 20% greater than the prize amount:
Randomly with a VRF  select a winner from the pool of purchased tickets
Transfer the prize amount to the winner's wallet
Generate a new raffle with a prize amount 1.3X higher than the previous one

A raffle status is either Active or Completed

Requirement: A VRF is required for the random number, not slot hashes
