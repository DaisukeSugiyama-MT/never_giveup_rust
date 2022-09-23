use rand::{distributions::Alphanumeric, Rng};

pub fn genarete_rnd_str(len: usize) -> String {
    //
    let mut rng = rand::thread_rng();

    let chars: String = (0..len).map(|_| rng.sample(Alphanumeric) as char).collect();

    /* for loopでpush */
    /*
        let mut s = String::with_capacity(len);
        for _ in 0..len {
            // 英数字をランダムに生成
            s.push(rng.sample(Alphanumeric) as char);
        }
    */
    chars
}
