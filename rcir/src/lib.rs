use std::error;
use std::fmt;
use std::collections::HashMap;

pub fn run_election<'a, Iter1, SubIter: std::iter::IntoIterator + 'a, Vobj: 'a + std::cmp::Eq + std::hash::Hash>(voters: &'a Iter1) -> Result<ElectionResult<Vobj>>
    where &'a Iter1: IntoIterator<Item = &'a SubIter>, &'a SubIter: IntoIterator<Item = &'a Vobj>
{
    //contains the list of eliminated candidates
    let mut eliminated: HashMap<&Vobj, bool> = HashMap::new();
    loop {
        // setup the vote data structure
        let mut round_votes: HashMap<&Vobj, u32> = HashMap::new();
        // number of voters
        let mut num_voters: u32 = 0;
        // count the votes
        for voter in voters {
            let (num_voters_checked, overflow) = num_voters.overflowing_add(1);
            if overflow {
                return Err(ElectionError::Overflow);
            }
            num_voters = num_voters_checked;
            for vote in voter {
                if !eliminated.contains_key(vote){
                    let mut vc = round_votes.entry(vote).or_insert(0);
                    *vc = *vc + 1;
                    break;
                }
            }
        }
        // ensure that we had voters
        if num_voters == 0 {
            return Err(ElectionError::EmptyVoteCollection);
        }
        // essure that we had votes
        if round_votes.is_empty() {
            return Err(ElectionError::VotersNoVotes)
        }
        // find the fifty percentile and see if we have a winner by majority
        // note: we need to make sure we don't overflow num_voters when getting
        // the fifty percentile
        let (fifty_numerator, overflow) = num_voters.overflowing_add(1);
        if overflow {
            return Err(ElectionError::Overflow);
        }
        let fifty_percent = (fifty_numerator) / 2;
        let mut winners = Vec::new();
        for (v, votecount) in round_votes.iter() {
            if *votecount >= fifty_percent {
                winners.push(*v);
            }
        }
        match winners.len() {
            0 => {}
            1 => {return Ok(ElectionResult::Winner(winners[0]));}
            _ => { return Ok(ElectionResult::Tie(winners)); }
        }
        // if we don't have a majority we need to eliminate the 
        // minimum vote getters
        let mut add_elim = Vec::new();
        let mut elim_count = u32::max_value();
        for (v, votecount) in round_votes.iter() {
            if *votecount < elim_count {
                add_elim.clear();
                add_elim.push(v);
                elim_count = *votecount;
            } else if *votecount == elim_count {
                add_elim.push(v);
            }
        }
        // add who to eliminate to the eliminated hashmap
        for elim in add_elim {
            eliminated.entry(elim).or_insert(true);
        }
    }
    //return Err(ElectionError::EmptyVoteCollection);
}

#[derive(Debug, PartialEq)]
pub enum ElectionResult<'a, T: 'a> {
    Winner(&'a T),
    Tie(Vec<&'a T>),
}

type Result<T> = std::result::Result<T, ElectionError>;
#[derive(Debug, Clone, PartialEq)]
pub enum ElectionError {
    EmptyVoteCollection,
    VotersNoVotes,
    Overflow,
}
impl fmt::Display for ElectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error with election input")
    }
}
impl error::Error for ElectionError {
    fn description(&self) -> &str {
        match &self {
            ElectionError::EmptyVoteCollection => {
                "Vote Collection is empty"
            },
            ElectionError::VotersNoVotes => {
                "There were voters, but no votes"
            }
            ElectionError::Overflow => {
                "An integer overflow occured"
            }
            // _ => {
            //     "Other Error"
            // }
        }        
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn basic_run() {
        let mut voter_a = VecDeque::new();
        voter_a.push_front("sue");
        voter_a.push_front("bill");
        voter_a.push_front("bob");

        let mut voter_b = VecDeque::new();
        voter_b.push_front("bill");
        voter_b.push_front("bob");
        voter_b.push_front("sue");

        let mut voter_c = VecDeque::new();
        voter_c.push_front("bob");
        voter_c.push_front("sue");
        voter_c.push_front("bill");

        let mut voter_d = VecDeque::new();
        voter_d.push_front("sue");
        voter_d.push_front("bill");
        voter_d.push_front("bob");

        let mut voter_e = VecDeque::new();
        voter_e.push_front("bill");
        voter_e.push_front("bob");
        voter_e.push_front("sue");

        let vec = vec![voter_a, voter_b, voter_c, voter_d, voter_e];
        let winner = run_election(&vec);
        //println!("{:?}", winner);
        if let Ok(winner) = winner {
            assert_eq!(ElectionResult::Winner(&"sue"), winner);
        } else {
            assert!(false);
        }
        //println!("winner is: {:?}", winner);
    }

    #[test]
    fn tie() {
        let mut voter_a = VecDeque::new();
        voter_a.push_front("sue");

        let mut voter_b = VecDeque::new();
        voter_b.push_front("bill");

        let vec = vec![voter_a, voter_b];
        let winner = run_election(&vec);
        if let Ok(ElectionResult::Tie(tie_res)) = winner {
            assert!(tie_res.contains(&&"bill"));
            assert!(tie_res.contains(&&"sue"));
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn tie2() {
        let mut voter_a = VecDeque::new();
        voter_a.push_front("sue");
        voter_a.push_front("bill");

        let mut voter_b = VecDeque::new();
        voter_b.push_front("bill");
        voter_b.push_front("sue");

        let vec = vec![voter_a, voter_b];
        let winner = run_election(&vec);
        if let Ok(ElectionResult::Tie(tie_res)) = winner {
            assert!(tie_res.contains(&&"bill"));
            assert!(tie_res.contains(&&"sue"));
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn voters_no_votes() {
        let voter_a: Vec<&str> = Vec::new();
        let voter_b: Vec<&str> = Vec::new();
        let voters = vec![voter_a, voter_b];
        let winner = run_election(&voters);
        if let Err(error_res) = winner {
            assert_eq!(error_res, ElectionError::VotersNoVotes);
        } else {
            assert!(false);
        }
    }
}
