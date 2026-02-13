//checks for a wireguard config file (wg0) and if it exists, backs it up and creates a new one. If it doesn't exist, creates one.

use std::path::Path;


pub fn run()
{
 
    let path = Path::new("/etc/wireguard/wg0.conf");
    
    if path.exsists() == true 
    {
        fs::renmae(path, bak.wg0.conf)?;
        println!("moved {} -> {}", original, backup);
    }

    File::create(path)?;
    println!("Created {}", path)?;



}
