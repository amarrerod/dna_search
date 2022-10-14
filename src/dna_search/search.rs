use super::genes::{Codon, Gene};

pub fn linear_search(gene: &Gene, codon: &Codon) -> bool {
    gene.contains(&codon)
}
