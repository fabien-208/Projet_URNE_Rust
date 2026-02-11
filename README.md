# 🗳️ Projet URNE (Rust)


## 📖 À propos

Ce projet vise à modéliser et calculer informatiquement les résultats d'élections en utilisant de multiples méthodes de vote (systèmes de Condorcet, systèmes par points, votes alternatifs, etc.). Il est entièrement développé en **Rust** pour garantir des calculs rapides et une gestion de la mémoire parfaitement sécurisée.

## ✨ Algorithmes de vote supportés

Le système est conçu de manière très modulaire. Chaque méthode implémente le trait commun `VotingAlgorithm`. 
Les méthodes suivantes sont actuellement implémentées :

* **Pluralité** (Vote uninominal à un tour)
* **Copeland** (Méthode de Condorcet par duels 1v1)
* **Borda** (Système de vote par points décroissants)
* **Schulze** (Méthode des chemins les plus forts)
* **Bucklin** (Vote majoritaire à seuil progressif)
* **IRV** (Instant-Runoff Voting / Vote alternatif)
* **Smith/IRV** (Combinaison de la méthode de Condorcet et du vote alternatif)
* **TRS** (Two-Round System / Uninominal à deux tours)
* **Copeland-Borda** (Combinaison de Copeland et Borda)
* **Baldwin** (Élimination progressive basée sur les points de Borda)

## 📂 Architecture du Projet

Le code source est divisé de manière logique pour faciliter l'ajout de nouvelles méthodes :

* `src/types.rs` : Définition des structures de base (`Election`, `Ballot`, `VoteResult`, `CandidateId`). Les candidats sont optimisés sous forme d'ID (`usize`).
* `src/vote/` : Cœur algorithmique. Contient un module par système de vote.
* `src/parser.rs` : Module chargé de lire, parser et nettoyer les fichiers texte contenant les bulletins.

## 📊 Format des données attendu

Le parser du projet est conçu pour interpréter des fichiers texte (ex: `Data.txt`) respectant le format suivant :

```text
A;B;C;D;E       # Ligne 1 : Liste complète des candidats séparés par ';'
A>B>C           # Lignes suivantes : Bulletins avec ordre de préférence ('>')
A>D>E>B
C>D>B
B 
D>A
```