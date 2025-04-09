use desafiografph_aula5::MyGraph;
use std::fs;

fn main() {
    let graph = MyGraph::new();
    
    // 1. Gerar representação DOT
    let dot_content = graph.to_dot();
    
    // 2. Salvar em arquivo
    fs::write("graph.dot", &dot_content).expect("Falha ao escrever arquivo DOT");
    
    println!("✅ Arquivo graph.dot gerado!");
    println!("🖼️  Para converter em imagem, execute:");
    println!("   dot -Tpng graph.dot -o graph.png");
    
    // Opcional: Mostrar conteúdo DOT no terminal
    println!("\nConteúdo do arquivo DOT:\n{}", dot_content);
}
