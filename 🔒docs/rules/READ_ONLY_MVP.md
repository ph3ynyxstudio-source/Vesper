READ_ONLY_MVP

Objectif

VespΣr est un cockpit de contexte.

Son rôle est de permettre à l'utilisateur de retrouver rapidement :

- ses projets ;
- leur documentation ;
- leur contexte ;
- leurs ressources ;
- leurs dernières sessions.

VespΣr reste en lecture seule pour les projets et fichiers existants, sauf pour
les actions de création explicitement déclenchées, prévisualisées et confirmées
par l'utilisateur. Aucun renommage, déplacement, écrasement ou suppression
implicite n'est autorisé.

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

Écritures explicitement autorisées :

- créer un nouveau dossier projet sous la racine VespΣr autorisée ;
- créer, pour ce nouveau projet, les quatre sous-dossiers normalisés ;
- créer le dossier Chr0 associé et ses cinq sous-dossiers lorsqu'il est absent ;
- associer sans le modifier un dossier Chr0 existant et complet ;
- refuser l'opération si un sous-dossier cible existe déjà ;
- ne créer aucun document ni fichier de métadonnées dans ce flux.

VespΣr peut également enregistrer dans `vesper.json`, après une action
explicite, le statut ou l'identifiant d'icône choisi pour un projet.

---

Interdit dans le MVP

VespΣr ne doit pas :

- supprimer un projet ;
- renommer, déplacer ou écraser un projet existant ;
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
- Local First · Écritures confirmées

---

À préciser plus tard

- Niveau exact des permissions Tauri
- Gestion des favoris
- Historique d'ouverture des projets
- Actions rapides personnalisées
