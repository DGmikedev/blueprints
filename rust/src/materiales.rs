#[derive(Debug)]
struct Material<'a> {
    id_tipo: u32,
    nombre_tipo: &'a str,
    marca: &'a str,
    id_casa_material: u32,
    precio_pieza: (f32, &'a str),
    precio_total: f32,
    tmp_entrga: (u32, &'a str),
    notas: &'a str,
}

impl Material<'_> {
    fn new<'a>(
        id_tipo: u32,
        nombre_tipo: &'a str,
        marca: &'a str,
        id_casa_material: u32,
        precio_pieza: (f32, &'a str),
        precio_total: f32,
        tmp_entrga: (u32, &'a str),
        notas: &'a str,
    ) -> Material<'a> {
        Material {
            id_tipo,
            nombre_tipo,
            marca,
            id_casa_material,
            precio_pieza,
            precio_total,
            tmp_entrga,
            notas,
        }
    }
}
fn main() {
    let mut material: Vec<Material> = Vec::new();
    let varilla = Material::new(
        1,
        "varilla",
        "s/marca",
        1,
        (13.20, "mt"),
        5002.0,
        (1, "semana"),
        "",
    );
    let cemento = Material::new(
        2,
        "CEMENTO",
        "CRUZ AZUL",
        1,
        (250.0, "BULTO 50"),
        40000.00,
        (1, "semana"),
        "no mojar",
    );

    material.push(varilla);
    material.push(cemento);

    println!("{:#?}", material);
}
