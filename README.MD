//  - Le « pool » de caractères inclut lettres (minuscules/majuscules), chiffres et quelques symboles
//    courants. En contexte réel, adaptez ce pool selon vos contraintes (ex. éviter des caractères ambigus
//    comme O/0, l/I, ou au contraire les inclure pour plus d’entropie).
//  - Le code crée un générateur aléatoire (thread_rng) à chaque itération du mot de passe.
//  - L’accès à un caractère via `pool.chars().nth(index)` parcourt les caractères depuis le début
//    (complexité O(n) pour nth). Ici, c’est acceptable vu la petite taille du pool, mais pour un très
//    grand set, on préférerait opérer directement sur les octets (`as_bytes()[index]`) lorsque le set
//    est ASCII pur, ou stocker les chars dans un vecteur pour un accès indexé direct.
//  - La gestion d’erreur est volontairement simple/conservative (avec `.expect(...)`). Dans une appli
//    utilisateur, on préférerait des messages plus conviviaux et une vraie validation/reprise après erreur.
