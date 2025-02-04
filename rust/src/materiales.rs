#[derive(Debug)]
struct Material<'a> {
    id_tipo: u32,
    nombre_tipo: &'a str,
    marca: &'a str,
    id_casa_material: u32,
    precio_pieza: (f32, &'a str),
    piezas_total: f32,
    precio_total: f32,
    tmp_entrega: (f32, &'a str),
    notas: &'a str,
}

impl Material<'_> {
    fn new<'a>(
        id_tipo: u32,
        nombre_tipo: &'a str,
        marca: &'a str,
        id_casa_material: u32,
        precio_pieza: (f32, &'a str),
        piezas_total: f32,
        precio_total: f32,
        tmp_entrega: (f32, &'a str),
        notas: &'a str,
    ) -> Material<'a> {
        Material {
            id_tipo,
            nombre_tipo,
            marca,
            id_casa_material,
            precio_pieza,
            piezas_total,
            precio_total,
            tmp_entrega,
            notas,
        }
    }
}

fn main() {
    let mut cost_total_proj: f32 = 0.0;
    let mut material: Vec<Material> = Vec::new();
    let varilla = Material::new(
        1,
        "varilla",
        "s/marca",
        1,
        (13.20, "mt"),
        34.0,
        5002.0,
        (1.0, "semana"),
        "",
    );
    let cemento = Material::new(
        2,
        "CEMENTO",
        "CRUZ AZUL",
        1,
        (250.0, "BULTO 50"),
        100.0,
        40000.00,
        (1.0, "semana"),
        "no mojar",
    );

    material.push(varilla);
    material.push(cemento);

    let dias: f32 = get_dias(&material[0].tmp_entrega.0, &material[0].tmp_entrega.1);
    println!("{dias} dias");

    // generador costos
    for costo in material.into_iter() {
        cost_total_proj += costo.precio_total
    }
    println!("{cost_total_proj}");

    // let costo:f32 = get_costo(&material[0]);
    //println!("{:#?}", material);
}

fn get_dias(cantidad: &f32, unidad: &str) -> f32 {
    let mut dias: f32 = 0f32;
    match unidad {
        "horas" => dias = cantidad / 24.0,
        "dia" => dias = 1.0,
        "semana" => dias = 7.0,
        "mes" => dias = 30.0,
        &_ => dias = 365.0,
    }
    dias = dias * cantidad;
    dias
}

fn get_costo(costo: &f32) -> f32 {
    println!("{:#?}", costo);
    10.0
}


