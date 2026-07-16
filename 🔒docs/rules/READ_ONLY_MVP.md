READ_ONLY_MVP

Objectif

VespΣr est un cockpit de contexte.

Son rôle est de permettre à l'utilisateur de retrouver rapidement :

- ses projets ;
- leur documentation ;
- leur contexte ;
- leurs ressources ;
- leurs dernières sessions.

La version 1.0 est principalement en lecture seule.

---

Principe fondamental

Le filesystem est la source de vérité.

VespΣr lit les informations existantes.

VespΣr ne devient pas le gestionnaire de ces informations.

---

Autorisé dans le MVP

VespΣr peut :

- découvrir les projets locaux ;
- lire les fichiers Markdown ;
- lire les fichiers JSON de configuration ;
- afficher des métadonnées ;
- afficher des résumés ;
- ouvrir des dossiers ;
- ouvrir VS Code ;
- ouvrir un terminal ;
- ouvrir un repository Git ;
- ouvrir des URLs externes.

Exception explicite :

- initialiser, à la demande de l'utilisateur et après confirmation, les quatre
  sous-dossiers normalisés d'un projet déjà créé manuellement ;
- refuser l'opération si un sous-dossier cible existe déjà ;
- ne créer ni projet, ni document, ni fichier de métadonnées dans ce flux.

---

Interdit dans le MVP

VespΣr ne doit pas :

- créer un projet ;
- supprimer un projet ;
- modifier un document ;
- éditer un fichier Markdown ;
- générer automatiquement du contenu ;
- réécrire des documents ;
- produire des synthèses ;
- exécuter des scripts de traitement ;
- gérer des tâches ;
- suivre la productivité ;
- agir comme une IA conversationnelle.

---

Relation avec chr0

chr0 écrit.

chr0 synthétise.

chr0 construit la mémoire.

VespΣr consulte cette mémoire.

VespΣr ne remplace pas chr0.

---

Décisions connues

- Local First
- Filesystem First
- Human First
- Read Only MVP

---

À préciser plus tard

- Niveau exact des permissions Tauri
- Gestion des favoris
- Historique d'ouverture des projets
- Actions rapides personnalisées
