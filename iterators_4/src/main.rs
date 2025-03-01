use std::{any, char};

#[derive(Debug,PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials
}
#[derive(Debug)]
struct TvChannel {
    name: String,
    channel_type: ChannelType
}


fn main() {
    let th_channel = [TvChannel {
        name: "TH Channel".to_string(),
        channel_type: ChannelType::Comedy
    },
    TvChannel {
        name: "News Channel".to_string(),
        channel_type: ChannelType::News
    },
    TvChannel {
        name: "Rust Live Channel".to_string(),
        channel_type: ChannelType::ProgrammingTutorials
    },
    TvChannel {
        name: "Rust Channel".to_string(),
        channel_type: ChannelType::ProgrammingTutorials
    },
    TvChannel {
        name: "Python Programming Channel".to_string(),
        channel_type: ChannelType::ProgrammingTutorials
    },
    ];


    let good_channels : Vec<&TvChannel>= th_channel
            .iter()
            .filter(|channel| channel.name.to_lowercase().contains("rust"))
            .collect();

    println!("Good Channels: {:?}", good_channels);
    println!("normal channels: {:?}", th_channel);


    let all_are_rust = th_channel
        .iter()
        .all(|channel| {channel.name.to_lowercase().contains("rust")});

    println!("All are rust: {}", all_are_rust);

    let any_is_rust = th_channel
        .iter()
        .any(|channel| {channel.name.to_lowercase().contains("rust")});

    println!("Any is rust: {}", any_is_rust);

}
