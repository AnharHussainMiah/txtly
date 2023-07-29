pub fn draw(version: &str) {
    let logo = r#"
████████ ██   ██ ████████ ██      ██    ██ 
   ██     ██ ██     ██    ██       ██  ██  
   ██      ███      ██    ██        ████   
   ██     ██ ██     ██    ██         ██    
   ██    ██   ██    ██    ███████    ██    
        2023 (c) TXTLY v{version}
    "#;
    let logo = logo.replace("{version}", version);
    println!("{}", logo);
}
