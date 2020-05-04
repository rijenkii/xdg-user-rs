fn main() -> Result<(), xdg_user::Error> {
    // You are reading the config file only when you create `UserDirs`
    let dirs = xdg_user::UserDirs::new()?;

    // You are not reading the config file here
    println!("Desktop:   {:?}", dirs.desktop());
    println!("Documents: {:?}", dirs.documents());
    println!("Downloads: {:?}", dirs.downloads());
    println!("Music:     {:?}", dirs.music());
    println!("Pictures:  {:?}", dirs.pictures());
    println!("Public:    {:?}", dirs.public());
    println!("Templates: {:?}", dirs.templates());
    println!("Videos:    {:?}", dirs.videos());

    Ok(())
}
