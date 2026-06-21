LOCAL_FIRST

Objectif

VespΣr est une application local-first.

Les données restent sur la machine de l'utilisateur.

L'application lit les fichiers locaux et affiche le contexte sans dépendre d'un service externe.

---

Principe

L'utilisateur garde le contrôle de ses fichiers.

VespΣr ne possède pas les données.

VespΣr ne remplace pas le filesystem.

---

Autorisé

VespΣr peut :

- lire des dossiers locaux ;
- lire des fichiers Markdown ;
- lire des fichiers JSON ;
- détecter des projets ;
- afficher des métadonnées ;
- ouvrir des chemins locaux avec les outils système.

---

Non autorisé dans le MVP

VespΣr ne doit pas :

- synchroniser automatiquement les données ;
- envoyer des données vers un cloud ;
- créer un compte utilisateur ;
- dépendre d'une API externe ;
- stocker des informations sensibles sans validation explicite.

---

Décision actuelle

Le MVP fonctionne sans connexion internet.

Les fichiers locaux restent accessibles même si VespΣr est supprimé.

---

À préciser plus tard

- Export éventuel de configuration
- Sauvegarde optionnelle
- Gestion de profils locaux
- Permissions Tauri exactes