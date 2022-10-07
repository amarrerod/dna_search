use crate::search::Nucleotide;

extern crate itertools;

mod search;

use search::Codon;

fn main() {
    println!("Hello, world!");
    let gene_str: String = String::from("ACGTGGCTCTCTAACGTACGTACGTACGGGGTTTATATATACCCTAGGACTCCCTTT");
    let gene = search::string_to_gene(&gene_str);

    let codon1 : Codon = (Nucleotide::A, Nucleotide::C, Nucleotide::G);
    let codon2 : Codon = (Nucleotide::G, Nucleotide::A, Nucleotide::T);
    println!("El codon: {:?} esta en la cadena genetica : {:?}", codon1, search::linear_search(&gene, &codon1));
    println!("El codon: {:?} esta en la cadena genetica : {:?}", codon2, search::linear_search(&gene, &codon2));

}
