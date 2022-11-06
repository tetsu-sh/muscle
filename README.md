## what to do
- record training history app

## using
- actix-web
- sqlx
- mysql5.7

### setting env
- make .env and following enviromental variable
    - DATABASE_URL
### layer
- domain
    - core domain object. repository interface
    - not to depend other layer
- middleware
    - custom functions in the back of framework
- repository
    - store, find object from databases. 
    - easily change?
- usecase
    - business logic.
    - organize domain,repository...
- utils
    - commone feature called from anylayer.
    - need to make good abstraction design.