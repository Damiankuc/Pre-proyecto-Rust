
//Simulacion de clase usuario en rust
struct Usuario{
    nombre, cargo: String,
    edad: u8
}
fn main(){
    usuario1=Usuario{
        nombre: String::from("Damian"),
        cargo: String::from("Programador"),
        edad: 22
    };
println!("El nombre del usuario es {}, su cargo es {} y su edad es {}", usuario1.nombre, usuario1.cargo, usuario1.edad);

}

