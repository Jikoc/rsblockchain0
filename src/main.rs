mod models;
fn main(){
    let diffulty = 4;
    let mut blockchain = models::blockchain::Blockchain::new(diffulty);
    for _ in 0..3{
    models::blockchain::Blockchain::add_block(&mut blockchain);
    }
}
