
use super::genes::{Gene, Codon};

pub fn linear_search(gene: &Gene, codon: &Codon) -> bool {
    gene.contains(&codon)
}