mod imobiliaria;

fn main() {
    let mut minha_imobiliaria = imobiliaria::Imobiliaria::new("Minha Imobiliária".to_string(), "Rua Principal, 123".to_string());

    minha_imobiliaria.novo_imovel("Avenida Central, 456".to_string(), 500000.0, 3, 2, 100.0);
    minha_imobiliaria.novo_imovel("Rua das Flores, 789".to_string(), 300000.0, 2, 1, 80.0);

    minha_imobiliaria.listar_imoveis();
}
