# RCIR
RCIR is a ranked choice instant runoff library written in rust. It's a work in progress. The library is in the rcir directory, and a sample binary is in the rcir_csv directory.

In the result of a tie for last place and no winner, all the last place candidates are removed, so it is not a 'Spoiler Proof' method [Spoiler Effect + Resistance to tactical voting](https://en.wikipedia.org/wiki/Instant-runoff_voting#Resistance_to_tactical_voting) 

RCIR will not mutate the input and aims to have as few restrictions on data types as possible

## Example
| Voter A | Voter B | Voter C | Voter D | Voter E |
|---------|---------|---------|---------|---------|
| Bob     | Sue     | Bill    | Bob     | Sue     |
| Bill    | Bob     | Sue     | Bill    | Bob     |
| Sue     | Bill    | Bob     | Sue     | Bill    |

This would result in an `ElectionResult::Winner(&"sue")` under either operational mode

## Tie Example

| Voter A | Voter B |
|---------|---------|
| Bob     | Sue     |

This would result in an `ElectionResult::Tie([&"Bob", &"Sue"])` under either operational mode

## Modes of operation
RCIR has 2 modes of operation:

* Complete Majority
* Remaining Majority

The differences between the two can best be explained with this sample:

### Start

| Voter A | Voter B | Voter C | Voter D | Voter E |
|---------|---------|---------|---------|---------|
| Alice   | Bob     | Chris   | Sam     | Alice   |
| Sam     | Sam     | Bob     | Chris   | Chris   |

### After Round 1
Bob, Chris, and Sam are eliminated (lowest vote getters)

| Voter A | Voter B | Voter C | Voter D | Voter E |
|---------|---------|---------|---------|---------|
| Alice   | _       | _       | _       | Alice   |
| _       | _       | _       | _       | _       |

Under `CompleteMajority`, there is no winner and the result would be: `ElectionError::NoMajorityWinner` because Alice has 2/5 votes, which is below the 50% threshold for a complete majority.

Under `RemainingMajority`, the result would be an `ElectionResult::Winner(&"Alice")` because Alice has 100% (2/2) of the remaining ballots of voters who still have candidates left.

