security

Objectif

Définir les limites de sécurité du MVP VespΣr.

VespΣr est une application de lecture et de navigation.

Le principe de sécurité principal est de limiter l'accès au strict nécessaire.

---

Principe

VespΣr doit avoir accès uniquement aux informations nécessaires à son fonctionnement.

Le MVP privilégie :

- simplicité ;
- prévisibilité ;
- contrôle utilisateur ;
- permissions minimales.

---

Lecture autorisée

VespΣr peut lire :

- les projets détectés ;
- les fichiers Markdown ;
- les fichiers JSON de configuration ;
- les dossiers nécessaires à la découverte des projets.

---

Écriture interdite

Dans le MVP :

VespΣr ne doit pas :

- créer des fichiers ;
- modifier des fichiers ;
- supprimer des fichiers ;
- renommer des fichiers ;
- déplacer des fichiers.

---

Limitation des chemins

Toute lecture doit être limitée aux racines configurées.

Exemple :

C:\Ph3yNyx.OS\Devs

Le backend Rust doit valider les chemins avant toute lecture.

---

Documents

Les documents sont affichés en lecture seule.

Aucun éditeur intégré n'est prévu dans le MVP.

---

Ouverture externe

VespΣr peut ouvrir :

- VS Code ;
- Terminal ;
- Explorer ;
- GitHub ;
- URLs externes.

Ces actions doivent être déclenchées explicitement par l'utilisateur.

---

Données utilisateur

Le MVP ne collecte pas :

- statistiques d'utilisation ;
- télémétrie ;
- analytics ;
- données cloud.

---

Relation avec chr0

VespΣr peut consulter les documents produits par chr0.

VespΣr ne doit pas les modifier.

---

Décisions connues

- Local First
- Read Only MVP
- Permissions minimales
- Aucune télémétrie

---

À préciser plus tard

- Permissions Tauri détaillées
- Gestion des favoris
- Historique local
- Cache éventuel
- Sandbox de lecture