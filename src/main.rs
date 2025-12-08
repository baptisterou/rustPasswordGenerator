use std::io;       // Import du module standard pour lire des données depuis l'entrée standard (stdin)
use rand::Rng;     // Trait `Rng` nécessaire pour appeler `gen_range` sur un générateur aléatoire

fn main() {
    // Point d'entrée du programme
    println!("Rust Password generator");

    // `i_loop` pilote la boucle principale : true => on continue, false => on sort.
    // NB: On pourrait aussi écrire une boucle `loop { ... }` et `break` pour sortir.
    let mut i_loop: bool = true;

    // Boucle principale : tant que l'utilisateur veut générer un nouveau mot de passe
    while i_loop == true {
        // Étape 1 : demander la longueur du mot de passe à l'utilisateur
        println!("Choose password length");

        // Buffer pour stocker la ligne lue au clavier (inclut le \n final)
        let mut input = String::new();

        // Lecture bloquante depuis stdin. En cas d'échec, on panic avec un message explicite.
        io::stdin()
            .read_line(&mut input)
            .expect("Error while reading");

        // `trim()` retire espaces et retours à la ligne en début/fin, pour obtenir uniquement les chiffres
        let input = input.trim();

        // Conversion de la chaîne en entier non signé (usize). Si l'utilisateur saisit autre chose
        // qu'un entier valide (ex: lettres), `.parse()` échoue et le programme s'arrête avec `expect`.
        // Pédagogiquement, cela illustre la gestion d'erreur via `Result` et la conversion de types.
        let password_length: usize = input.parse().expect("Please put an integer.");

        // Pool de caractères autorisés pour construire le mot de passe.
        // Ici : 26 lettres miniscules + 26 majuscules + 10 chiffres + 10 symboles = 72 caractères.
        let pool = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";

        // Chaîne résultat qui contiendra le mot de passe généré, construite de manière incrémentale
        let mut password_generated = String::new();


        // Étape 2 : génération aléatoire caractère par caractère
        // - On parcourt un compteur de 0 jusqu'à `password_length - 1`
        // - À chaque itération :
        //     1) On crée un RNG lié au thread courant (thread-local). Ici on le recrée pour
        //        chaque caractère à des fins pédagogiques ; en pratique, on le créerait une seule
        //        fois avant la boucle pour éviter du coût inutile.
        //     2) On tire un index aléatoire dans l'intervalle [0, pool.len())
        //     3) On récupère le caractère correspondant dans `pool`
        //     4) On l'ajoute à la String résultat
        for _ in 0..password_length {
            // Générateur aléatoire (basé sur la source d'entropie du système)
            let mut rng = rand::thread_rng();

            // `gen_range(0..pool.len())` renvoie un entier uniforme dans [0, len-1]
            let index = rng.gen_range(0..pool.len());

            // Récupération du `index`-ième caractère de `pool`.
            // `chars()` itère sur les scalaires Unicode ; `nth` parcourt depuis le début à chaque appel.
            let c = pool.chars().nth(index).unwrap();

            // Ajout du caractère à la fin de la String `password_generated`
            password_generated.push(c);
        }

        // Affichage du mot de passe final à l'écran
        println!("New password: {}" , password_generated);

        // Étape 3 : demander si l'utilisateur souhaite recommencer
        let mut choose = String::new();
        println!("Do you want to generate a new password? y/n");

        // Lecture de la réponse utilisateur
        io::stdin()
            .read_line(&mut choose)
            .expect("Error while reading");

        // Normalisation (suppression des espaces/retours à la ligne)
        let choose = choose.trim();

        // Interprétation de la réponse :
        //  - "y" : on continue la boucle
        //  - "n" : on arrête la boucle
        //  - autre : on signale l'entrée invalide et on arrête pour rester simple
        if choose == "n"{
            i_loop = false;
        } else if choose == "y" {
            i_loop = true;
        } else {
            println!("Invalid input");
            i_loop = false;
        }
    }
}