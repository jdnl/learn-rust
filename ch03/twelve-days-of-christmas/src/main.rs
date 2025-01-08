fn main() {
    let days_of_christmas = 12;

    let christmas_days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    let gifts = [
        "a partridge in a pear tree.",
        "Two turtle doves,",
        "Three french hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve dummers drumming,",
    ];


    for days in 0..days_of_christmas {
        println!("On the {} day of Christmas,", christmas_days[days]);
        println!("My true love gave to me:");

        for c in (0..days+1).rev() {
            println!("{}", gifts[c]);
        }
        println!();
    }

}
