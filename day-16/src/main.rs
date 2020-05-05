mod fft;
mod pattern;

// const DATA: &str = "59715091976660977847686180472178988274868874248912891927881770506416128667679122958792624406231072013221126623881489317912309763385182133601840446469164152094801911846572235367585363091944153574934709408511688568362508877043643569519630950836699246046286262479407806494008328068607275931633094949344281398150800187971317684501113191184838118850287189830872128812188237680673513745269645219228183633986701871488467284716433953663498444829748364402022393727938781357664034739772457855166471802886565257858813291667525635001823584650420815316132943869499800374997777130755842319153463895364409226260937941771665247483191282218355610246363741092810592458";
const DATA: &str = "12345678";

fn main() {
    let data = DATA.chars().map(|c| c.to_digit(10).unwrap() as i64);
    let mut data = data.cycle().take(DATA.len() * 2).collect::<Vec<_>>();

    for _ in 0..100 {
        println!("{:?}", data);
        data = fft::flawed_ft(&data);
    }
}
