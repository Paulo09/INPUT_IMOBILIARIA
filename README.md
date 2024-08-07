
                                                                       INPUT IMOBILIARIA - RUST

1. Crie uma struct Imobiliaria com campos nome, endereco e imoveis, no arquivo imobiliaria.rs
2. O campo imoveis deve ser um vetor de structs do tipo Imovel.
3. Crie a struct Imovel com campos endereco, preco, num_quartos, num_banheiros e metragem.
4. Implemente a função novo_imovel na struct Imobiliaria, que receba os parâmetros endereco, preco, num_quartos, num_banheiros e metragem.
5. A função deve criar um objeto do tipo Imovel com os parâmetros recebidos e adicioná-lo ao vetor imoveis.
6. Implemente a função listar_imoveis na struct Imobiliaria, que liste todos os imóveis do vetor imoveis.
7. Crie uma nova instância da struct Imobiliaria em main.rs, com o nome, endereço e um vector de imóveis vazio.
8. Chame o método novo_imovel() da instância da Imobiliaria para adicionar um novo imóvel à lista de imóveis.
9. Chame o método listar_imoveis() da instância da Imobiliaria para listar todos os imóveis adicionados.
Perguntas dessa tarefa
Crie o arquivo imobiliaria.rs

Crie o arquivo main.rs


Mostrar rascunhos


Criando o Arquivo imobiliaria.rs
Rust
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
Use o código com cuidado.

Criando o Arquivo main.rs
Rust
mod imobiliaria;

fn main() {
    let mut minha_imobiliaria = imobiliaria::Imobiliaria::new("Minha Imobiliária".to_string(), "Rua Principal, 123".to_string());

    minha_imobiliaria.novo_imovel("Avenida Central, 456".to_string(), 500000.0, 3, 2, 100.0);
    minha_imobiliaria.novo_imovel("Rua das Flores, 789".to_string(), 300000.0, 2, 1, 80.0);

    minha_imobiliaria.listar_imoveis();
}
Use o código com cuidado.

Explicação:
Estrutura Imovel: Define as características de um imóvel.
Estrutura Imobiliaria: Representa uma imobiliária com seus imóveis.
Método new: Construtor da imobiliária.
Método novo_imovel: Adiciona um novo imóvel ao vetor de imóveis da imobiliária.
Método listar_imoveis: Imprime os detalhes de todos os imóveis.
main.rs: Cria uma instância de Imobiliaria, adiciona imóveis e lista-os.
Pontos importantes:
Vec<Imovel>: Utilizado para armazenar uma lista de imóveis.
&mut self: Permite modificar o objeto Imobiliaria dentro do método novo_imovel.
Formatação de saída: Utiliza println! e {:2} para formatar a saída de forma clara e concisa.
Tipos de dados: Os tipos de dados escolhidos são adequados para representar as informações de um imóvel.
Este código fornece uma base sólida para representar imobiliárias e seus imóveis em Rust. Você pode expandir este código para incluir mais funcionalidades, como:

Busca de imóveis: Implementar filtros para buscar imóveis por preço, número de quartos, etc.
Atualização de informações: Permitir a atualização dos dados de um imóvel existente.
Remoção de imóveis: Remover um imóvel da lista.
Persistência de dados: Salvar os dados da imobiliária em um arquivo ou banco de dados.
Com este exemplo, você pode criar aplicações mais complexas relacionadas a gestão de imóveis.
