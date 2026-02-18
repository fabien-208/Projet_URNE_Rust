# 🗳️ Projet Urne Rust

Une implémentation performante, modulaire et robuste en **Rust** dédiée à la théorie du choix social. Ce projet permet de simuler, comparer et analyser différents systèmes de vote à partir de fichiers de données.

## 📋 Description

Ce projet a pour but de traiter des élections via différents algorithmes de scrutins (majoritaires, préférentiels, Condorcet, etc.). Il est conçu pour être efficace, capable de gérer de grands volumes de données (fichiers de plusieurs millions de votes) et de gérer les cas d'erreurs de manière sécurisée.

## ✨ Fonctionnalités

Le cœur du projet réside dans le module `vote`, qui implémente les méthodes suivantes :

### Méthodes de Condorcet et dérivées
* **Schulze** : Méthode de Condorcet utilisant les chemins les plus forts.
* **Copeland** : Basé sur les duels gagnés contre les autres candidats.
* **Copeland-Borda** : Variante hybride.
* **Smith-IRV** : Application de l'Instant Runoff Voting restreint à l'ensemble de Smith.

### Méthodes Préférentielles et à Points
* **Borda** : Attribution de points selon le classement.
* **Baldwin** : Méthode d'élimination basée sur le score Borda.
* **Bucklin** : Approche par médiane et majorité absolue.
* **IRV (Instant Runoff Voting)** : Vote alternatif avec éliminations successives.

### Méthodes Majoritaires
* **Plurality** : Scrutin majoritaire uninominal à un tour.
* **TRS (Two-Round System)** : Scrutin majoritaire à deux tours.

## 📁 Structure du projet

```PROJET_URNE_RUST/
├── Data/               # Fichiers d'input (Scénarios de vote)
├── src/
│   ├── vote/           # Logique des algorithmes de scrutin
│   │   ├── schulze.rs
│   │   ├── borda.rs
│   │   ├── irv.rs
│   │   └── ...
│   ├── main.rs         # Point d'entrée : Orchestration du programme
│   ├── parser.rs       # Lecture et validation des fichiers .txt
│   └── types.rs        # Structures de données (Candidat, Bulletin, Resultat)
├── Cargo.toml          # Dépendances et métadonnées
└── README.md           # Documentation
```


## 🛠️ Installation

Prérequis : Avoir [Rust et Cargo](https://www.rust-lang.org/) installés.

1.  **Cloner le dépôt :**
    ```bash
    git clone https://github.com/fabien-208/Projet_URNE_Rust
    cd PROJET_URNE_RUST
    ```

2.  **Compiler le projet (Mode Release recommandé pour la performance) :**
    ```bash
    cargo build --release
    ```


## 🚀 Utilisation

L'application s'exécute en ligne de commande. Elle parse les fichiers situés dans le dossier `Data/`.

### Commande basique
```bash
cargo run --release