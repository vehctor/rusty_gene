use rand::Rng;
use rand::prelude::SliceRandom;

fn generate_parent(length: usize,gene_set: &str) -> String {
    let mut rng = rand::thread_rng();
   
    let mut genes: Vec<char> = Vec::new();
    for _ in 0..length {
        let gene: char = gene_set.chars().nth(rng.gen_range(0..gene_set.len())).unwrap();
        genes.push(gene);
    }
    genes.into_iter().collect()
}


fn mutate(parent: &str, gene_set: &str) -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..parent.len());
    let mut child_genes: Vec<char> = parent.chars().collect();
    let genes: Vec<char> = gene_set.chars().collect();
    let mut sampled_genes = genes.clone();
    sampled_genes.shuffle(&mut rng);
    let (new_gene, alternate) = (sampled_genes[0], sampled_genes[1]);
    child_genes[index] = if new_gene != child_genes[index] {
        new_gene
    } else {
        alternate
    };
    child_genes.into_iter().collect()
}

fn get_fitness(guess: &str, target: &str) -> usize {
    let mut fitness: usize = 0;
    for (a, b) in guess.chars().zip(target.chars()) {
        
        if a == b {
            fitness += 1;
        }
    }
    
    fitness
}


fn get_best(get_fitness: fn(&str, &str) -> usize, target_len: usize, optimal_fitness: usize, gene_set: &str, target: &str) -> String {
    let mut best_parent = generate_parent(target_len, gene_set);
    let mut best_fitness = get_fitness(&best_parent, &target);
    
    if best_fitness >= optimal_fitness {
        return best_parent;
    }
    loop {
        let child = mutate(&best_parent, gene_set);
        let child_fitness = get_fitness(&child, &target);
        
        if best_fitness >= child_fitness {
            continue;
        }
        
        if child_fitness >= optimal_fitness {
            return child;
        }
        best_fitness = child_fitness;
        best_parent = child;
    }
}
fn main() {
    let gene_set: &str  = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!.";
    
    let target: &str = "Methinks it is like a weasel";
    let target_len: usize = target.len();
    let optimal_fitness: usize = target_len;
    let best_parent = get_best(get_fitness, target_len, optimal_fitness, gene_set, target);
    println!("{}", best_parent);
}
