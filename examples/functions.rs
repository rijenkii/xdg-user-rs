fn main() -> Result<(), xdg_user::Error> {
    // You are reading the config file every time you call these functions
    println!("Desktop:   {:?}", xdg_user::desktop()?);
    println!("Documents: {:?}", xdg_user::documents()?);
    println!("Downloads: {:?}", xdg_user::downloads()?);
    println!("Music:     {:?}", xdg_user::music()?);
    println!("Pictures:  {:?}", xdg_user::pictures()?);
    println!("Public:    {:?}", xdg_user::public()?);
    println!("Templates: {:?}", xdg_user::templates()?);
    println!("Videos:    {:?}", xdg_user::videos()?);

    Ok(())
}
