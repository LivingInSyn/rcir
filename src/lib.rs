use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ElectionResult<T: std::cmp::Eq + std::hash::Hash + std::marker::Copy> {
    Winner(T),
    Tie(Vec<T>),
}

/// Runs an election returning a winner, or a tie. It expects a vector of VecDeque's which are a queue of peoples priorities
pub fn run_election<T: std::cmp::Eq + std::hash::Hash + std::marker::Copy>(mut vote_collection: Vec<VecDeque<T>>) -> ElectionResult<T> {
    //run passes    
    loop {
        //vector to hold this rounds votes
        let mut roundvotes: Vec<T> = Vec::new();
        for mut vote in vote_collection.clone() {
            if let Some(vote) = vote.front() {
                roundvotes.push(*vote);
            }
        }
        //count the votes
        let mut votecount: HashMap<T,u64> = HashMap::new();
        let mut totalvote_counter: u64 = 0;
        for vote in roundvotes {
            let vc = votecount.entry(vote).or_insert(0);
            *vc += 1;
            totalvote_counter += 1;
        }
        //see if there's a winner
        let fifty_percent = totalvote_counter / 2;
        //let mut to_remove: Option<&T> = None;
        let mut to_remove_vec: Vec<&T> = Vec::new();
        let mut winner: Option<&T> = None;
        let mut to_remove_count = u64::max_value();
        for (key, votes) in &votecount {
            if *votes > fifty_percent {
                winner = Some(key);
                break;
            }
            //if this is lower than the pervious, set the low vote count
            if *votes < to_remove_count {
                to_remove_count = *votes;
                to_remove_vec.clear();
                to_remove_vec.push(key);
            }
            else if *votes == to_remove_count {
                to_remove_vec.push(key);
            }

        }
        //if there's a winner, return
        if let Some(winner) = winner {
            return ElectionResult::Winner(*winner);
        }
        //detect if there is a tie. There is a tie if the to_remove_vec has the same
        //length as the votecount hashmap
        if to_remove_vec.len() == votecount.len() {
            let mut tievec = Vec::new();
            for tienetry in to_remove_vec {
                tievec.push(*tienetry);
            }
            return ElectionResult::Tie(tievec);
        }        
        //otherwise clear the loser(s) from the queue
        for removal in to_remove_vec {
            for vote in vote_collection.iter_mut() {
                //pop the front element
                if let Some(frontvote) = vote.pop_front() {
                    //if it's not the to_remove value, put it back
                    if frontvote != *removal {
                        vote.push_front(frontvote);
                    }
                }
            }
        }
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
        let winner = run_election(vec);
        println!("winner is: {:?}", winner);
        assert_eq!(ElectionResult::Winner("sue"), winner);

    }

    #[test]
    fn tie() {
        let mut voter_a = VecDeque::new();
        voter_a.push_front("sue");

        let mut voter_b = VecDeque::new();
        voter_b.push_front("bill");

        let vec = vec![voter_a, voter_b];
        let winner = run_election(vec);
        let _tievec: Vec<&str> = vec!["bill", "sue"];
        if let ElectionResult::Tie(tie_res) = winner {
            assert!(tie_res.contains(&"bill"));
            assert!(tie_res.contains(&"sue"));
        }
        else {
            assert!(false);
        }
    }
}
