discovery

Objectif

Définir comment VespΣr découvre les projets disponibles dans le filesystem.

La découverte doit être rapide, prévisible et entièrement locale.

---

Principe

Un projet est découvert à partir du filesystem.

VespΣr ne maintient pas de base de données de projets dans le MVP.

La liste affichée doit toujours refléter l'état réel du disque.

---

Source principale

Dans le MVP, VespΣr explore une ou plusieurs racines configurées.

Exemple :

C:\Ph3yNyx.OS\Devs

---

Processus de découverte

Racine
  ↓
Liste des dossiers
  ↓
Validation du projet
  ↓
Lecture des métadonnées
  ↓
Construction du ContextProject
  ↓
Affichage

---

Validation minimale

Un dossier peut être considéré comme un projet si au moins un élément connu est détecté :

README.md
docs/
src/
src-tauri/
package.json
.git/
vesper.json

La logique doit rester permissive.

L'absence d'un élément ne doit pas exclure automatiquement un projet.

---

Construction du ContextProject

Informations récupérées :

Nom du dossier
Chemin racine
Métadonnées éventuelles
Présence des dossiers clés
Dernière modification
Repository Git éventuel

---

Lecture des métadonnées

Si présent :

vesper.json

Le backend tente de le charger.

Si le fichier est invalide :

- journaliser l'erreur ;
- ignorer les champs problématiques ;
- continuer la découverte.

Une erreur ne doit jamais bloquer l'affichage du projet.

---

Ordre d'affichage

Version MVP :

Ordre alphabétique

Exemple :

Astr4lForge
Chr0nosV3rs
Lun4rMood
VespΣr

---

Actualisation

Dans le MVP :

Découverte au démarrage
+
Actualisation manuelle

Aucune surveillance temps réel du filesystem n'est nécessaire.

---

Gestion des erreurs

Cas possibles :

Projet déplacé

Le projet disparaît simplement de la liste.

---

Projet supprimé

Le projet disparaît simplement de la liste.

---

Accès refusé

Le projet est ignoré.

L'erreur peut être affichée dans les logs.

---

Métadonnées invalides

Le projet reste affiché avec les valeurs par défaut.

---

Performance

Objectifs :

- découverte rapide ;
- faible consommation mémoire ;
- aucune indexation complexe ;
- aucun cache permanent dans le MVP.

---

MVP v0.1

Inclus :

- découverte locale ;
- lecture des métadonnées ;
- détection des dossiers clés ;
- actualisation manuelle.

Exclus :

- indexation avancée ;
- surveillance temps réel ;
- recherche plein texte ;
- cache persistant ;
- synchronisation cloud.

---

Relation avec chr0

La découverte concerne uniquement les projets.

La découverte des sessions et synthèses sera traitée séparément.

VespΣr doit pouvoir afficher les résultats produits par chr0 sans dépendre de son fonctionnement interne.

---

Décisions connues

- Local First
- Filesystem First
- Découverte simple
- Aucun cache permanent
- Lecture seule

---

À préciser plus tard

- Racines multiples
- Favoris
- Projets masqués
- Tri par activité récente
- Détection automatique de nouveaux projets
- Recherche globale