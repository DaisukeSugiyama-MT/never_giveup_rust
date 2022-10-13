fn main() {
    let range = (1..=100);
    let list = range.map(|x| x).collect::<Vec<i32>>();

    let length = list.len();
    //
    println!("length: {}", length);

    // listを10で分割する
    let chunked_list = list.chunks(10);

    let chunked_list_len = chunked_list.len();

    //
    println!("chunked_list_len: {}", chunked_list_len);
}
