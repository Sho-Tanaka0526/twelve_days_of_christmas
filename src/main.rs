fn main() {
    //配列を作成する
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    //歌詞を表示する
    for number in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me",days[number]);
        if number >= 11 {println!("Twelve drummers drumming");}
        if number >= 10 {println!("Eleven pipers piping");}
        if number >= 9 {println!("Ten lords a-leaping");}
        if number >= 8 {println!("Nine ladies dancing");}
        if number >= 7 {println!("Eight maids a-milking");}
        if number >= 6 {println!("Seven swans a-swimming");}
        if number >= 5 {println!("Six geese a-laying");}
        if number >= 4 {println!("Five golden rings");}
        if number >= 3 {println!("Four calling birds");}
        if number >= 2 {println!("Three French hens");}
        if number >= 1 {println!("Two turtledoves");}
        println!("And a partridge in a pear tree\n");
    }
    println!("And a partridge in a pear tree\n");
}
