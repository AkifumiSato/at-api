# develop env
1. docker build
```shell script
docker-compose build
```

2. start
```shell script
docker-compose up
```

3. bash
```shell script
docker-compose exec [backend|db] bash
```

# migration
1. init
```shell script
docker-compose exec backend bash
diesel migration generate [migration_name]
```

2. edit
edit migration file.

3. run
```shell script
diesel migration run
```

4. redo
```shell script
diesel migration redo
```

5. revert
```shell script
diesel migration revert
```
