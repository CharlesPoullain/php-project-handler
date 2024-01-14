use dialoguer::{theme::ColorfulTheme, Select};
use rayon::prelude::*;
use std::{process::Command as ProcessCommand, sync::Arc, path::Path};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

fn main() {
    let branches_communes = vec!["branch_1", "branch_2"];
    println!("Sélectionnez une branche pour tous les projets:");

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&branches_communes)
        .default(0)
        .interact()
        .unwrap();

    let branche_choisie = branches_communes[selection];

    let projets = vec![
        "/Users/charlespoullain/Code/rust/gestion_projets_php/projects/project1",
        "/Users/charlespoullain/Code/rust/gestion_projets_php/projects/project2",
    ];

    let multi_progress = Arc::new(MultiProgress::new());

    projets.par_iter().for_each_with(multi_progress.clone(),  |mp, chemin| {
        
        let pb = mp.add(ProgressBar::new(2));
        pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));

        ProcessCommand::new("git")
            .args(&["checkout", branche_choisie])
            .current_dir(chemin)
            .output()
            .expect("Échec du git checkout");
        pb.inc(1);

        ProcessCommand::new("composer")
            .args(&["install"])
            .current_dir(chemin)
            .output()
            .expect("Échec du composer install");
        pb.inc(1);
        
        let path = Path::new(chemin);
        let nom_dossier = path.file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("Inconnu");

        pb.finish_with_message(format!("{} terminé !", nom_dossier));
    });

    println!("Mise à jour des projets terminée.");
}