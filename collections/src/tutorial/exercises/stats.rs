use std::collections::hash_map::Entry;



pub fn median(vector: &Vec<i32>) -> Option<f64> {
    // Ok, to find the median, we must first sort vector
    let mut sorted: Vec<i32> = vector
        .iter()
        .cloned()
        .collect();
    sorted.sort();// Bottleneck happens here
    // O(n log n)
    match sorted.len() % 2 {
        0 => even_length_median(sorted),
        _ => odd_length_median(sorted),
    } 
}

pub fn mode(vector: &Vec<i32>) -> Option<(i32,usize)> {
    use priority_queue::PriorityQueue;
    use std::collections::HashMap;
    let mut multiset: HashMap<i32, usize> = HashMap::new();
    for &x in vector {
        match multiset.entry(x) {
            Entry::Occupied(mut occ) => *occ.get_mut() += 1,
            Entry::Vacant(v) => {
                v.insert(1);
            }
        }
    }
    let mut pq = PriorityQueue::new();
    for (elem, mult) in multiset {
        pq.push(elem, mult);
    }
    pq.pop()    
}

fn odd_length_median(sorted_owned: Vec<i32>) -> Option<f64>{
    let mid: usize = sorted_owned.len() / 2;
    match sorted_owned.get(mid) {
        Some(n) => Some(*n as f64),
        None => None,
    }
}

fn even_length_median(sorted_owned: Vec<i32>) -> Option<f64> {
    if sorted_owned.len() == 0 {
        return None
    }
    let right_index = sorted_owned.len() / 2;
    let left_index = right_index - 1;

    match sorted_owned.get(right_index) {
        Some(left) => {
            match sorted_owned.get(left_index) {
                Some(right) => {
                    let left_dec = *left as f64;
                    let right_dec = *right as f64;
                    Some((left_dec + right_dec)/2.0)
                },
                _ => None
            }
        },
        _ => None
    }
}

