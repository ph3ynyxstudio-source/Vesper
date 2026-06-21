# Convention de commits obligatoire

Pour toute réponse impliquant une modification du projet, fournir systématiquement :

## Commit proposé FR

```text
type: description en français
```

## Commit proposé EN

```text
type: description in English
```

Types recommandés :

```text
feat:
fix:
docs:
refactor:
chore:
test:
style:
```

Exemple :

### Commit proposé FR

```text
docs: initialisation de la structure documentaire VespΣr
```

### Commit proposé EN

```text
docs: initialize VespΣr documentation structure
```

---

# Convention de facturation (Billing)

Pour chaque tâche réalisée, fournir également :

## Billing Summary

### Complexity

```text
Low | Medium | High
```

### Estimated Scope

```text
XS  : 1 à 3 fichiers
S   : 4 à 10 fichiers
M   : 10 à 25 fichiers
L   : 25+ fichiers
```

### Type of Work

Cocher les catégories concernées :

* Documentation
* Architecture
* Frontend
* Backend
* Rust
* React
* TypeScript
* Tauri
* Refactor
* Tests
* Tooling
* Research

### Risk Level

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

# Format de fin de réponse obligatoire

```md
## Validation

...

## Billing Summary

Complexity: Medium
Scope: S
Risk: Low
Breaking Changes: No
Documentation Impact: Required

## Commit proposé FR

docs: ...

## Commit proposé EN

docs: ...
```

Ne jamais terminer une réponse contenant des modifications sans :

* Validation
* Billing Summary
* Commit FR
* Commit EN
