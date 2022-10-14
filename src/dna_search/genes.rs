use itertools::Itertools;

/// Nucleotide enum which takes the values A, C, G and T.
#[derive(Debug, PartialEq)]
pub enum Nucleotide {
    A,
    C,
    G,
    T,
}

impl Nucleotide {
    pub fn new(s: &char) -> Option<Nucleotide> {
        match s {
            'A' => Some(Nucleotide::A),
            'C' => Some(Nucleotide::C),
            'G' => Some(Nucleotide::G),
            'T' => Some(Nucleotide::T),
            _ => None,
        }
    }
}

/// Codon type, a combination of three nucleotides
pub type Codon = (Nucleotide, Nucleotide, Nucleotide);
// Gene type, a list of nucleotides
pub type Gene = Vec<Codon>;

pub fn string_to_gene(s: &str) -> Gene {
    let mut gene: Gene = vec![];
    for mut chunk in &s.chars().chunks(3) {
        let (a, b, c) = chunk.next_tuple().unwrap();
        gene.push((
            Nucleotide::new(&a).unwrap(),
            Nucleotide::new(&b).unwrap(),
            Nucleotide::new(&c).unwrap(),
        ));
    }

    gene
}
