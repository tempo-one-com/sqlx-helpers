# Description
Ensemble de fonctions réutilisables pour les projets rust/sqlx

La base de donnée principale doit être Postgres. Son url doit être définie dans un .env ou les variables d'environnement avec le code DATABASE_URL.  
Pour Teliway, les codes doivent être les suivants:
- DATABASE_GTRA_URL
- DATABASE_GTLS_URL
- DATABASE_GTRI_URL
- DATABASE_EXPRESS_URL

# Usage
```
let databases = databases::Databases::init(env::vars()).await?;

let pgpool = databases.default;

let gtrapool = databases.get_by_code("gtra");
```

# Versions
## 0.2.0 12/10/2023
Dans init, indication du nb max connexions

## 0.1.0 13/09/2023
Struct Database pour gérer une base Postgres (default) et les bases téliways définies dans les variables d'environnement.