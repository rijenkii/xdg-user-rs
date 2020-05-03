fn main() -> Result<(), xdg_user::Error> {
    let dirs = xdg_user::UserDirs::new()?;

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
