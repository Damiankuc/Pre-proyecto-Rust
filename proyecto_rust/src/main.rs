
//Simulacion de clase usuario en rust
struct Usuario{
    nombre: String,
    cargo: String,
    edad: u8
}
fn main(){
    let usuario1=Usuario{
        nombre: String::from("Damian"),
        cargo: String::from("Programador"),
        edad: 22
    };
println!("El nombre del usuario es {}, su cargo es {} y su edad es {}", usuario1.nombre, usuario1.cargo, usuario1.edad);

}

fn main(){
    let usuario2=Usuario{
        nombre: String::from("Pancho"),
        cargo: String::from("Ingeniero"),
        edad: 27
    };
println!("El nombre del usuario es {}, su cargo es {} y su edad es {}", usuario2.nombre, usuario2.cargo, usuario2.edad);

}


