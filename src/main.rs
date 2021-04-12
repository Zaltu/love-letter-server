use game::objects::Action;

pub fn main() -> anyhow::Result<()> {
    let config = game::config::Config::load("config.toml")?;
    
    let mut game = game::Game::new(config);
    game.setup()?;

    let _player_id1 = game.add_player()?;
    let _player_id2 = game.add_player()?;
    let _player_id3 = game.add_player()?;
    let _player_id4 = game.add_player()?;

    loop {
        let action = game.draw();
        if let Action::EndRound = action {
            
            break;
        }
    }
    Ok(())
}
