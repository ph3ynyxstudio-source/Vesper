# RESPONSE_CONVENTION

## Objectif

Toutes les réponses de Codex doivent suivre une structure cohérente afin de faciliter la validation, le suivi des modifications et la compréhension du projet.

---

# Structure obligatoire

Toute réponse impliquant une analyse, une proposition ou une modification doit suivre l'ordre suivant.

## Ce que j'ai compris

Résumé court de la demande.

Expliquer :

* le besoin ;
* l'objectif ;
* le résultat attendu.

---

## Analyse

Identifier :

* les fichiers concernés ;
* les composants concernés ;
* les dépendances concernées ;
* les impacts possibles.

Si aucune analyse n'est nécessaire :

```text
Aucune analyse particulière requise.
```

---

## Actions réalisées

Lister clairement :

* ce qui a été créé ;
* ce qui a été modifié ;
* ce qui a été supprimé ;
* ce qui a été volontairement laissé intact.

---

## Fichiers modifiés

Format attendu :

```text
src/components/Example.tsx
src/features/example/example.ts
docs/system/architecture.md
```

Si aucun fichier :

```text
Aucun fichier modifié.
```

---

## Validation

Confirmer :

* que la tâche est terminée ;
* que les contraintes ont été respectées ;
* que les éléments hors scope n'ont pas été modifiés.

---

## Points à vérifier

Lister :

* les validations humaines nécessaires ;
* les choix ouverts ;
* les décisions qui restent à prendre.

Si aucun :

```text
Aucun point bloquant identifié.
```

---

## Billing Summary

### Complexity

```text
Low | Medium | High
```

### Scope

```text
XS
S
M
L
```

### Risk

```text
Low | Medium | High
```

### Breaking Changes

```text
Yes / No
```

### Documentation Impact

```text
None
Recommended
Required
```

---

## Commit proposé FR

```text
type: description en français
```

---

## Commit proposé EN

```text
type: description in english
```

---

# Cas particuliers

## Documentation uniquement

Toujours inclure :

* Ce que j'ai compris
* Actions réalisées
* Fichiers modifiés
* Validation
* Billing Summary
* Commit FR
* Commit EN

---

## Analyse uniquement

Toujours inclure :

* Ce que j'ai compris
* Analyse
* Recommandation
* Aucun fichier modifié
* Billing Summary

---

## Refus d'exécution

Toujours expliquer :

* pourquoi ;
* quelles contraintes bloquent ;
* quelle information manque.

---

# Règle finale

Ne jamais terminer une réponse contenant une modification sans :

* Validation
* Billing Summary
* Commit proposé FR
* Commit proposé EN
