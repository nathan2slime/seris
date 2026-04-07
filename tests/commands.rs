use seris::commands::commands;

#[test]
fn registers_expected_commands() {
    let names: Vec<_> = commands().into_iter().map(|command| command.name).collect();

    assert_eq!(
        names,
        vec![
            "ping",
            "clear",
            "apod",
            "get_random_anime",
            "get_random_manga",
            "about",
            "uptime",
            "stats"
        ]
    );
}
