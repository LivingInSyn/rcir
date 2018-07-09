# RCIR
RCIR is a ranked choice instant runoff library written in rust. It's a work in progress. The library is in the rcir directory, and a sample binary is in the rcir_csv directory.

In the result of a tie for last place and no winner, all the last place candidates are removed

## Example
| Voter A | Voter B | Voter C | Voter D | Voter E |
|---------|---------|---------|---------|---------|
| Bob     | Sue     | Bill    | Bob     | Sue     |
| Bill    | Bob     | Sue     | Bill    | Bob     |
| Sue     | Bill    | Bob     | Sue     | Bill    |

This would result in an ElectionResult::Winner("Sue")

## Tie Example

| Voter A | Voter B |
|---------|---------|
| Bob     | Sue     |

This would result in an ElectionResult::Tie(vec!["Bob", "Sue"])

