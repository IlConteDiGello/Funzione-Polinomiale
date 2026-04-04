/*
Data una funzione polinomiale di terzo grado,  inserire i coefficienti a, b ,c d, rappresentare il grafico.
Individuare un intervallo nel quale è presente uno zero. Trovare lo zero con il metodo di bisezione.
Produrre:
- Relazione dettagliata
- Presentazione (con Canva o Google Presentazione)

Il lavoro sarà esposto anche oralmente.

I team saranno scelti da voi. Massimo 4 componenti per team.*/
use std::{io, vec}; //importazione modulo per l'input dell'utente

fn main() {
    let mut a: f64; //variabili coefficienti
    let b: f64; //: i32 serve a dichiarare la varibiabile come tipo intero, signed (u32 per unsigned)
    let c: f64; //i tipi di interi signed e non sono: i8, i16, i32, i64, i128 per il numero di bit
    let d: f64;
    let intervallo_di_calcolo: f64;
    let x_minima: f64;
    let x_massima: f64;
    let mut punti: Vec<[f64; 2]> = Vec::new();
    let mut stringa_input = String::new();
    
    //inserimento a
    println!("Inserisci coefficiente a ( diverso da 0):");
    loop{
        io::stdin() //richiamo la funzione, se non avessi importato std::io
            .read_line(&mut stringa_input) //richiamo il metodo read_line
            //& significa che è un riferimento, cosi più parti del codice possono accederci senza doverlo copiare ogni volta
            .expect("Errore supergalattico di lettura amico");

        match  stringa_input.trim().parse::<f64>() {
            Ok(n) => {
                a = n;
                if a!=0.0 {
                    break;
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


    //b
    println!("Inserisci coefficiente b:");
    stringa_input = " ".to_string();
    loop{
        io::stdin()
        .read_line(&mut stringa_input)
        .expect("Errore supergalattico di lettura amico");

        match  stringa_input.trim().parse::<f64>() {
            Ok(n) => {
                b = n;
                break;
            },
            Err(e) => {
                println!("Errore, riprova: {}", e);
                stringa_input = " ".to_string();
            },
        }               
    }
    
    
    //c
    println!("Inserisci coefficiente c:");
    stringa_input = " ".to_string();
    loop{
        io::stdin()
        .read_line(&mut stringa_input)
        .expect("Errore supergalattico di lettura amico");

        match  stringa_input.trim().parse::<f64>() {
            Ok(n) => {
                c = n;
                break;
            },
            Err(e) => {
                println!("Errore, riprova: {}", e);
                stringa_input = " ".to_string();
            },
        }               
    }
    
    
    //d
    println!("Inserisci coefficiente d:");
    stringa_input = " ".to_string();
    loop{
        io::stdin()
        .read_line(&mut stringa_input)
        .expect("Errore supergalattico di lettura amico");

        match  stringa_input.trim().parse::<f64>() {
            Ok(n) => {
                d = n;
                break;
            },
            Err(e) => {
                println!("Errore, riprova: {}", e);
                stringa_input = " ".to_string();
            },
        }               
    }

    //Intervallo di calcolo
    println!("Inserisci l'intervallo di calcolo:");
    stringa_input = " ".to_string();
    loop{
        io::stdin()
        .read_line(&mut stringa_input)
        .expect("Errore supergalattico di lettura amico");

        match  stringa_input.trim().parse::<f64>() {
            Ok(n) => {
                intervallo_di_calcolo = n;
                break;
            },
            Err(e) => {
                println!("Errore, riprova: {}", e);
                stringa_input = " ".to_string();
            },
        }               
    }

    //x minima
    println!("Inserisci la x minima del grafico:");
    stringa_input = " ".to_string();
    loop{
        io::stdin()
        .read_line(&mut stringa_input)
        .expect("Errore supergalattico di lettura amico");

        match  stringa_input.trim().parse::<f64>() {
            Ok(n) => {
                x_minima = n;
                break;
            },
            Err(e) => {
                println!("Errore, riprova: {}", e);
                stringa_input = " ".to_string();
            },
        }               
    }

    //x massima
    println!("Inserisci la x massima del grafico:");
    stringa_input = " ".to_string();
    loop{
        io::stdin()
        .read_line(&mut stringa_input)
        .expect("Errore supergalattico di lettura amico");

        match  stringa_input.trim().parse::<f64>() {
            Ok(n) => {
                x_massima = n;
                break;
            },
            Err(e) => {
                println!("Errore, riprova: {}", e);
                stringa_input = " ".to_string();
            },
        }               
    }


    println!("coefficiente a: {}", a);
    println!("coefficiente b: {}", b);
    println!("coefficiente c: {}", c);
    println!("coefficiente d: {}", d);

    let mut x: f64 = x_minima;
    while x <= x_massima {
        punti.push([x, calcola(a, b, c, d, x)]);
        x += intervallo_di_calcolo;
    }

    use charming::{
    Chart,
    component::{Title, Grid, Axis},
    /*df,
    element::{    },
    series::{},*/
    HtmlRenderer
    };

    let chart = Chart::new()
        .title(Title::new().text("Funzione di terzo grado"))
        .grid(Grid::new())
        .x_axis(Axis::new().data(valori_x))
        .y_axis(Axis::new().data(valori_y));

    let mut renderer = HtmlRenderer::new("My-Chart", 800, 600);
    renderer.save(&chart, "/tmp/chart.html").unwrap();
    
} 

fn calcola(a: f64, b: f64, c: f64, d: f64, x: f64) -> f64 {
    return a*x*x*x + b*x*x + c*x + d;
}
    
    
    
    /*use plotly::{Plot, Scatter};

    let mut plot = Plot::new();
    let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
    plot.add_trace(trace);

    plot.show();
}*/
