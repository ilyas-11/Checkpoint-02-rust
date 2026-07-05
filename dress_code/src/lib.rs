#[derive(Debug, PartialEq, Eq)]
pub enum Jacket{
    Black,
    White,
    Flowers,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Hat{
    Snapback,
    Baseball,
    Fedora,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}
pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {
    let mut outfit = Outfit{
        jacket:Jacket::Black, 
        hat:Hat::Snapback,
    };
    match formality_level{
        Some(level)=>{
            if level>0{
                outfit.jacket=Jacket::White;
            }
        }
        _=>outfit.jacket=Jacket::Flowers
    }
    match invitation_message{
        Ok(_)=>outfit.hat=Hat::Fedora,
        Err(_)=>{
            match formality_level{
                None=>outfit.hat=Hat::Baseball,
                _=>outfit.hat=Hat::Snapback,
            }
        }
    }
    outfit
}