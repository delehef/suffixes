#![feature(test)]
use test::Bencher;

extern crate time;
use time::PreciseTime;


use suffix::SuffixTable;
mod divsufsort64;

static GENOME: &'static str = include_str!("../msy.fasta");
static NEEDLE: &'static str = "ATGCCGTA";

fn divsufsort() {
    fn search(dna: &[u8], sa: &[divsufsort64::idx], pattern: &[u8]) -> Vec<divsufsort64::idx> {
        let mut out = 0;
        let count = unsafe {
            divsufsort64::sa_search64(dna.as_ptr(),
                                      dna.len() as i64,
                                      pattern.as_ptr(),
                                      pattern.len() as i64,
                                      sa.as_ptr(),
                                      sa.len() as i64,
                                      &mut out)
        };

        sa.iter()
            .cloned()
            .skip(out as usize)
            .take(count as usize)
            .collect::<Vec<_>>()
    }




    let mut sa = Vec::with_capacity(GENOME.len());
    sa.resize(GENOME.len(), 0);
    unsafe {divsufsort64::divsufsort64(GENOME.as_ptr(), sa.as_mut_ptr(), GENOME.len() as i64);}
    println!("{:?} matches", search(GENOME.as_bytes(), &sa, NEEDLE.as_bytes()).len());
}
#[bench]
fn bench_divsufsort(b: &mut Bencher) { b.iter(|| divsufsort()) }


fn suffix() {
    let st = SuffixTable::new(GENOME);
    println!("{:?} matches", st.positions(NEEDLE).len());
}
#[bench]
fn bench_suffix(b: &mut Bencher) { b.iter(|| suffix()) }

fn main() {
    let start = PreciseTime::now();
    divsufsort();
    let end = PreciseTime::now();
    println!("{} seconds for divsufsort.", start.to(end));

    let start = PreciseTime::now();
    suffix();
    let end = PreciseTime::now();
    println!("{} seconds for suffix.", start.to(end));
}
