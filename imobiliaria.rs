struct Imovel {
    endereco: String,
    preco: f64,
    num_quartos: u8,
    num_banheiros: u8,
    metragem: f64,
}

struct Imobiliaria {
    nome: String,
    endereco: String,
    imoveis: Vec<Imovel>,
}

impl Imobiliaria {
    fn new(nome: String, endereco: String) -> Self {
        Imobiliaria {
            nome,
            endereco,
            imoveis: Vec::new(),
        }
    }

    fn novo_imovel(&mut self, endereco: String, preco: f64, num_quartos: u8, num_banheiros: u8, metragem: f64) {
        let novo_imovel = Imovel {
            endereco,
            preco,
            num_quartos,
            num_banheiros,
            metragem,
        };
        self.imoveis.push(novo_imovel);
    }

    fn listar_imoveis(&self) {
        for imovel in &self.imoveis {
            println!("Endereço: {}", imovel.endereco);
            println!("Preço: R$ {:.2}", imovel.preco);
            println!("Quartos: {}", imovel.num_quartos);
            println!("Banheiros: {}", imovel.num_banheiros);
            println!("Metragem: {:.2} m²", imovel.metragem);
            println!("--------------------");
        }
    }
}
