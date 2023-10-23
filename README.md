# Huitième dojo (23/10/23) : kata Jeu de la vie

## Énoncé du problème

Ce Kata consiste à calculer la prochaine génération du jeu de la vie de Conway, quelle que soit sa position de départ. Voir <http://en.wikipedia.org/wiki/Conway%27s_Game_of_Life> pour le contexte.

Vous commencez avec une grille bidimensionnelle de cellules, où chaque cellule est vivante ou morte. Dans cette version du problème, la grille est finie et aucune vie ne peut exister en dehors des bords. Lors du calcul de la prochaine génération de la grille, suivez ces règles :

1. Toute cellule vivante ayant moins de deux voisins vivants meurt, comme si elle était causée par une sous-population.
2. Toute cellule vivante ayant plus de trois voisins vivants meurt, comme par surpeuplement.
3. Toute cellule vivante ayant deux ou trois voisins vivants survit jusqu'à la génération suivante.
4. Toute cellule morte ayant exactement trois voisines vivantes devient une cellule vivante.

Vous devez écrire un programme capable d'accepter une grille arbitraire de cellules et qui produira une grille similaire montrant la génération suivante.

## Ouvrir le code

Ce dépôt contient la configuration nécessaire pour faire le kata directement dans un devcontainer. Le dossier peut être ouvert directement dans un VSCode avec l'extension `Dev Containers` installée.

Une fois le projet chargé, vous pouvez modifier le programme principal situé dans le fichier `src/main.rs`. Pour lancer le programme, il suffit d'exécuter la commande suivante :

```bash
cargo run
```

## Lancer les tests

Pour exécuter les tests, lancer la commande suivante :

```bash
cargo test
```

Tous les tests sont ignorés par défault dans ce dépôt. Après avoir fait passé le premier test, ouvrir le fichier `dojo4.rs` situé dans le dossier `tests` et enlever l'annotation `#[ignore]` sur le second test et relancer la commande précédente pour voir le résultat. Comme le nom du test l'indique, le test va échouer et cela montrera que votre système de test fait bien son travail. Un test est une fonction précédée d'une annotation `#[test]`. Pour résoudre notre kata, nous devrons en ajouter plusieurs.

Pour lancer _uniquement_ les tests ignorés sans éditer le fichier, vous pouvez lancer la commande suivante :

```bash
cargo test -- --ignored
```

Pour lancer _tous_ les tests, vous pouvez utiliser la commande suivante :

```bash
cargo test -- --include-ignored
```

Pour lancer uniquement un test spécifique, par exemple `useless_test`, utilisez cette commande :

```bash
cargo test useless_test
```

Si le test à lancer est _ignoré_, vous pouvez lancer la commande suivante :

```bash
cargo test always_fail -- --ignored
```

Pour en apprendre plus sur les tests en Rust, référez vous à la [documentation des tests][rust-tests].

[rust-tests]: https://doc.rust-lang.org/book/ch11-00-testing.html
