
//Simulacion de clase usuario en rust

#[derive(Debug)]
struct Usuario {
    nombre: String,
    cargo: String,
    edad: u8,
}

struct Sistema {
    usuarios: Vec<Usuario>,
}

impl Sistema {
    fn new() -> Self {
        Sistema {
            usuarios: Vec::new(),
        }
    }

    fn agregar_usuario(&mut self, usuario: Usuario) {
        self.usuarios.push(usuario);
    }

    fn cantidad_usuarios(&self) -> usize {
        self.usuarios.len()
    }
}

fn main() {
    let mut sistema = Sistema::new();

    let usuario1 = Usuario {
        nombre: String::from("Damian"),
        cargo: String::from("Programador"),
        edad: 22,
    };

    let usuario2 = Usuario {
        nombre: String::from("Pancho"),
        cargo: String::from("Ingeniero"),
        edad: 27,
    };

    sistema.agregar_usuario(usuario1);
    sistema.agregar_usuario(usuario2);

    println!("Cantidad de usuarios: {}", sistema.cantidad_usuarios());

    for usuario in &sistema.usuarios {
        println!(
            "Nombre: {}, Cargo: {}, Edad: {}",
            usuario.nombre, usuario.cargo, usuario.edad
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contador_usuarios() {
        let mut sistema = Sistema::new();

        // Agregamos 5 usuarios
        for i in 0..5 {
            sistema.agregar_usuario(Usuario {
                nombre: format!("User{}", i),
                cargo: "Dev".to_string(),
                edad: 20,
            });
        }

        assert_eq!(sistema.cantidad_usuarios(), 5);

        // Agregamos uno más
        sistema.agregar_usuario(Usuario {
            nombre: "Extra".to_string(),
            cargo: "Tester".to_string(),
            edad: 25,
        });

        assert_eq!(sistema.cantidad_usuarios(), 6);
    }

    #[test]
    fn test_incremento_usuario() {
        let mut sistema = Sistema::new();

        let antes = sistema.cantidad_usuarios();

        sistema.agregar_usuario(Usuario {
            nombre: "Nuevo".to_string(),
            cargo: "Dev".to_string(),
            edad: 30,
        });

        let despues = sistema.cantidad_usuarios();

        assert_eq!(despues, antes + 1);
    }

    #[test]
    fn test_datos_usuario() {
        let mut sistema = Sistema::new();

        sistema.agregar_usuario(Usuario {
            nombre: "Roberto".to_string(),
            cargo: "Backend".to_string(),
            edad: 28,
        });

        assert_eq!(sistema.usuarios[0].nombre, "Roberto");
        assert_eq!(sistema.usuarios[0].cargo, "Backend");
        assert_eq!(sistema.usuarios[0].edad, 28);
    }
}