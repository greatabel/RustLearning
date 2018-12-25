fn main() {
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);

    println!("{:?}", vec);

    // 只写一部分类型，剩下的部分让编译器去推导
    let player_scores = [
        ("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19),
    ];
    let players: Vec<_> = player_scores
        .iter()
        .map(|&(player, _score)|{
            player
        })
        .collect();

    println!("{:?}", players);


}