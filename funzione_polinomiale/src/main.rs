use std::io; //importazione modulo per l'input dell'utente
use charming::{
    Chart, 
    HtmlRenderer, 
    component::{Axis, Grid, Title}, 
    element::{AreaStyle, AxisType, Label, LineStyle, MarkLine, MarkLineData, MarkLineVariant, Symbol, Tooltip, Formatter, Trigger}, 
    series::Line
    };

fn main() {
    let a: f64 = leggi_f64_non_zero("Inserisci coefficiente a ( diverso da 0):"); //variabili coefficienti
    let b: f64 = leggi_f64("Inserisci coefficiente b:"); //: i32 serve a dichiarare la varibiabile come tipo intero, signed (u32 per unsigned)
    let c: f64 = leggi_f64("Inserisci coefficiente c:"); //i tipi di interi signed e non sono: i8, i16, i32, i64, i128 per il numero di bit
    let d: f64 = leggi_f64("Inserisci coefficiente d:");
    let step_di_calcolo: f64 = leggi_f64("Inserisci lo step di calcolo:");
    let x_minima: f64 = leggi_f64("Inserisci la x minima del grafico:");
    let x_massima: f64 = leggi_f64("Inserisci la x massima del grafico:");
    let mut punti: Vec<Vec<f64>> = Vec::new();
    let epsilon: f64 = leggi_f64_non_zero("Inserisci l'intervallo minimo per il metodo di bisezione");
    let tolleranza: f64 = leggi_f64_non_zero("Inserisci la tolleranza per la ricerca dello zero col metodo di bisezione");

    //debug
    println!("coefficiente a: {}", a);
    println!("coefficiente b: {}", b);
    println!("coefficiente c: {}", c);
    println!("coefficiente d: {}", d);
    println!("step: {}", step_di_calcolo);
    println!("x minima: {}", x_minima);
    println!("x massima: {}", x_massima);

    //crea l'array di punti
    let mut x: f64 = x_minima;
    while x <= x_massima {
        punti.push(vec![x, calcola(a, b, c, d, x)]);
        x += step_di_calcolo;
    }

    //bisezione
    match cerca_metodo_bisezione(x_minima, x_massima, epsilon, tolleranza, a, b, c, d) {
        Ok(zero) => {
            println!("Lo zero della funzione si trova in x = {}", zero);
        }
        Err(mess) => {
            println!("{}", mess);
        }
    }

    //imposta e crea il grafico
    let chart = Chart::new()
        .title(Title::new().text(format!("{}x³ + {}x² + {}x + {}", a, b, c, d))) //da cambiare, mettere funzione
        .grid(Grid::new())
        .x_axis(Axis::new().type_(AxisType::Value))
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
        )
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Axis)
        )
        .series(
            Line::new()
                .smooth(true)
                .symbol(Symbol::None)
                .data(punti),
        );

    let mut renderer = HtmlRenderer::new("My-Chart", 1800, 950);
    renderer.save(&chart, "/tmp/chart.html").unwrap();
    open::that("/tmp/chart.html").unwrap();
    
} 



//funzioni
fn calcola(a: f64, b: f64, c: f64, d: f64, x: f64) -> f64 {
    return a*x*x*x + b*x*x + c*x + d;
}

fn cerca_metodo_bisezione(mut k: f64, mut j: f64, epsilon: f64, tolleranza: f64, a:f64, b:f64, c:f64, d:f64) -> Result<f64, String> {
    if !(calcola(a, b, c, d, k) *  calcola(a, b, c, d, j)< 0.0) { //f(a) * f(b) < 0
        return Err("Non è possibile applicare il metodo di bisezione".to_string());
    }
    let mut appross = calcola(a, b, c, d, (k + j)/2.0);
    while (k - j).abs() > epsilon && appross < tolleranza {
        if calcola(a, b, c, d, k) * calcola(a, b, c, d, (k+j)/2.0)< 0.0 {
            j = (k+j)/2.0;
        }
        else {
            k = (k+j)/2.0;
        } 
        appross = calcola(a, b, c, d, (k + j)/2.0);
    }
    return Ok((k+j)/2.0);
}
    
fn leggi_f64_non_zero(s: &str) -> f64 {
    let mut stringa_input = String::new();
    loop{
        println!("{}", s);
        io::stdin() //richiamo la funzione, se non avessi importato std::io
            .read_line(&mut stringa_input) //richiamo il metodo read_line
            //& significa che è un riferimento, cosi più parti del codice possono accederci senza doverlo copiare ogni volta
            .expect("Errore supergalattico di lettura amico");

        match  stringa_input.trim().parse::<f64>() {
            Ok(n) => {
                if n!=0.0 {
                    return n;
                }
                println!("a deve essere diversa da 0, riprova");
                stringa_input = " ".to_string();
            },
            Err(e) => {
                println!("Errore, riprova: {}", e);
                stringa_input = " ".to_string();
            },
        }        
        
    }
}

fn leggi_f64(s: &str) -> f64 {
    let mut stringa_input = String::new();
    loop{
        println!("{}", s);
        io::stdin()
        .read_line(&mut stringa_input)
        .expect("Errore supergalattico di lettura amico");

        match  stringa_input.trim().parse::<f64>() {
            Ok(n) => {
                return n;
            },
            Err(e) => {
                println!("Errore, riprova: {}", e);
                stringa_input = " ".to_string();
            },
        }               
    }
}