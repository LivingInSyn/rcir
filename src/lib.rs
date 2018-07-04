use std::collections::VecDeque;
use std::collections::HashMap;

pub extern fn run_election<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone + std::marker::Copy>(mut vote_collection: Vec<VecDeque<T>>) -> T {
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
        let mut to_remove: Option<&T> = None;
        let mut winner: Option<&T> = None;
        let mut to_remove_count = u64::max_value();
        for (key, votes) in &votecount {
            if *votes >= fifty_percent {
                winner = Some(key);
                break;
            }
            if *votes < to_remove_count {
                to_remove = Some(key);
                to_remove_count = *votes;
            } 
        }
        //if there's a winner, return
        if let Some(winner) = winner {
            return *winner;
        }
        //otherwise clear the loser from the queue
        if let Some(to_remove) = to_remove {
            for vote in vote_collection.iter_mut() {
                //pop the front element
                if let Some(frontvote) = vote.pop_front() {
                    //if it's not the to_remove value, put it back
                    if frontvote != *to_remove {
                        vote.push_front(frontvote);
                    }
                }
            }
        } else {
            panic!("There's a tie!")
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

        let mut vec = vec![voter_a, voter_b, voter_c, voter_d, voter_e];
        let winner = run_election(vec);
        println!("winner is: {}", winner);
        assert_eq!("sue", winner);

    }
}
