use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use aoc2024_common::file::read_input_lines;

fn main() {
    let cxns = read_input_lines(23).into_iter()
        .map(|line| {
            let spl = line.split_once("-").unwrap();
            (spl.0.to_string(), spl.1.to_string())
        })
        .map(|(a, b)| [(a.to_string(), b.to_string()), (b.to_string(), a.to_string())])
        .flatten()
        .into_grouping_map()
        .collect::<HashSet<_>>();

    println!("Part 1: {}", solve_p1(&cxns));
    println!("Part 2: {}", solve_p2(&cxns));
}

fn solve_p1(cxns: &HashMap<String, HashSet<String>>) -> u64 {
    let mut triplets = HashSet::new();
    for (src, dests) in cxns {
        let mut seen = HashSet::new();
        for dest in dests {
            for dest2 in &cxns[dest] {
                let needle_cxn = (dest.clone(), dest2.clone());
                if seen.contains(&needle_cxn) {
                    continue;
                }
                if dests.contains(dest2.as_str()) {
                    let mut triplet = vec![src.clone(), dest.clone(), dest2.clone()];
                    triplet.sort();
                    triplets.insert(triplet);
                    seen.insert(needle_cxn.clone());
                    seen.insert((needle_cxn.1.clone(), needle_cxn.0.clone()));
                }
            }
        }
    }

    triplets.iter()
        .filter(|&t| t.iter().any(|c| c.chars().nth(0).unwrap() == 't'))
        .collect::<Vec<_>>()
        .len() as u64
}

fn solve_p2(cxns: &HashMap<String, HashSet<String>>) -> String {
    let mut res = Vec::new();
    bk(
        cxns,
        HashSet::new(),
        cxns.keys().cloned().collect(),
        HashSet::new(),
        &mut res,
    );
    let maximal = res.iter().max_by_key(|set| set.len()).unwrap();
    maximal.iter().sorted().join(",")
}

fn bk(
    graph: &HashMap<String, HashSet<String>>,
    r: HashSet<String>,
    p: HashSet<String>,
    x: HashSet<String>,
    result: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        result.push(r.clone());
        return;
    }
    let mut cur_p = p;
    let mut cur_x = x;
    let u = cur_p.union(&cur_x).nth(0).unwrap();
    let nu = graph.get(u).unwrap();
    let mut to_remove = HashSet::new();
    for v in cur_p.difference(nu) {
        let nv = graph.get(v).unwrap();
        let mut rec_r = r.clone();
        rec_r.insert(v.clone());
        let rec_p = cur_p.intersection(&nv).cloned().collect::<HashSet<_>>();
        let rec_x = cur_x.intersection(&nv).cloned().collect::<HashSet<_>>();
        bk(graph, rec_r, rec_p, rec_x, result);
        to_remove.insert(v.clone());
        cur_x.insert(v.clone());
    }
    cur_p.retain(|v| !to_remove.contains(v));
}
